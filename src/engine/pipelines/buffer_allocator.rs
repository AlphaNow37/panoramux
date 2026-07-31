use crate::engine::pipelines::material::MaterialHandle;
use crate::engine::pipelines::mesh::MeshHandle;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ops::Range;

#[derive(Clone, Copy, Debug)]
pub struct AllocSlot {
    alloc_id: usize,
    relative_idx: usize,
}

#[derive(Debug, Clone, Copy)]
struct Block {
    start_offset: usize,
    size: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct MeshAllocation {
    nb_permanent: usize,
    nb_frame: usize,
    material: MaterialHandle,
    mesh: MeshHandle,
    block: Block,
}

pub struct BufferAllocator<T: bytemuck::Pod> {
    pub values: Vec<T>,
    allocs: Vec<MeshAllocation>,
    key_to_idx: HashMap<(MaterialHandle, MeshHandle), usize>,
    material_to_idx: HashMap<MaterialHandle, Vec<usize>>,
    free_blocks: Vec<Block>,
}
impl<T: bytemuck::Pod> BufferAllocator<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: vec![T::zeroed(); capacity],
            allocs: Vec::new(),
            key_to_idx: HashMap::new(),
            material_to_idx: HashMap::new(),
            free_blocks: vec![Block {
                start_offset: 0,
                size: capacity,
            }],
        }
    }

    fn find_free_block_starting_by(&self, offset: usize) -> Option<usize> {
        self.free_blocks
            .binary_search_by(|block| block.start_offset.cmp(&offset))
            .ok()
    }
    fn find_free_block_ending_by(&self, offset: usize) -> Option<usize> {
        self.free_blocks
            .binary_search_by(|block| (block.start_offset + block.size).cmp(&offset))
            .ok()
    }
    fn find_new_free_idx(&self, offset: usize) -> usize {
        self.free_blocks
            .binary_search_by(|block| block.start_offset.cmp(&offset))
            .unwrap_or_else(|idx| idx)
    }
    fn find_best_fit(&mut self, min_size: usize, max_size: usize) -> Block {
        loop {
            let mut best_block = None;
            for (i, block) in self.free_blocks.iter().enumerate() {
                if min_size <= block.size && block.size <= max_size {
                    return *block;
                }
                if block.size < min_size {
                    continue;
                }
                match best_block {
                    None => best_block = Some(i),
                    Some(best) => {
                        if block.size < self.free_blocks[best].size {
                            best_block = Some(i);
                        }
                    }
                }
            }

            if let Some(idx2) = best_block {
                let block2 = self.free_blocks[idx2];
                let returned_block;
                if block2.size <= max_size {
                    returned_block = block2;
                    self.free_blocks.remove(idx2);
                } else {
                    returned_block = Block {
                        start_offset: block2.start_offset,
                        size: max_size,
                    };
                    self.free_blocks[idx2].start_offset += max_size;
                    self.free_blocks[idx2].size -= max_size;
                }
                return returned_block;
            }

            self.grow_buffer();
        }
    }
    fn move_values(&mut self, block_src: Block, block_dst: Block) {
        self.values.copy_within(
            block_src.start_offset..block_src.start_offset + block_src.size,
            block_dst.start_offset,
        )
    }
    fn grow_buffer(&mut self) {
        let curr_len = self.values.len();
        self.values
            .extend(std::iter::repeat_n(T::zeroed(), curr_len));
        match self.free_blocks.last_mut() {
            Some(b) if b.start_offset + b.size == curr_len => {
                b.size += curr_len
            }
            _ => self.free_blocks.push(Block {
                start_offset: curr_len,
                size: curr_len,
            }),
        }
    }
    fn resize_block(&mut self, block: Block, min_size: usize, max_size: usize) -> Block {
        let end_offset = block.start_offset + block.size;
        if let Some(idx2) = self.find_free_block_starting_by(end_offset) {
            let block2 = self.free_blocks[idx2];
            let final_size = block.size + block2.size;
            if min_size <= final_size && final_size <= max_size {
                self.free_blocks.remove(idx2);
                return Block {
                    start_offset: block.start_offset,
                    size: final_size,
                };
            }
            if final_size > max_size {
                let new_block = Block {
                    start_offset: block.start_offset,
                    size: max_size,
                };
                self.free_blocks[idx2].start_offset = new_block.start_offset + new_block.size;
                self.free_blocks[idx2].size = final_size - max_size;
                return new_block;
            }
        }
        let new_block = self.find_best_fit(min_size, max_size);
        match (
            self.find_free_block_ending_by(block.start_offset),
            self.find_free_block_starting_by(block.start_offset + block.size),
        ) {
            (None, None) => {
                let free_idx = self.find_new_free_idx(block.start_offset);
                self.free_blocks.insert(free_idx, block);
            }
            (Some(idx3), None) => {
                debug_assert_eq!(
                    self.free_blocks[idx3].size + self.free_blocks[idx3].start_offset,
                    block.start_offset
                );
                self.free_blocks[idx3].size += block.size;
            }
            (None, Some(idx3)) => {
                debug_assert_eq!(
                    self.free_blocks[idx3].start_offset,
                    block.start_offset + block.size
                );
                self.free_blocks[idx3].start_offset -= block.size;
                self.free_blocks[idx3].size += block.size;
            }
            (Some(idx3), Some(idx4)) => {
                debug_assert_eq!(
                    self.free_blocks[idx3].size + self.free_blocks[idx3].start_offset,
                    block.start_offset
                );
                debug_assert_eq!(
                    self.free_blocks[idx4].start_offset,
                    block.start_offset + block.size
                );
                self.free_blocks[idx3].size += block.size + self.free_blocks[idx4].size;
                self.free_blocks.remove(idx4);
            }
        }
        self.move_values(block, new_block);
        new_block
    }

    fn find_slot(
        &mut self,
        material: MaterialHandle,
        mesh: MeshHandle,
        is_permanent: bool,
    ) -> AllocSlot {
        match self.key_to_idx.entry((material, mesh)) {
            Entry::Vacant(entry) => {
                let alloc_id = self.allocs.len();
                entry.insert(alloc_id);
                self.material_to_idx
                    .entry(material)
                    .or_default()
                    .push(alloc_id);
                const MIN_NEW_BLOCK_SIZE: usize = 4;
                const MAX_NEW_BLOCK_SIZE: usize = 8;
                let new_block = self.find_best_fit(MIN_NEW_BLOCK_SIZE, MAX_NEW_BLOCK_SIZE);
                self.allocs.push(MeshAllocation {
                    block: new_block,
                    nb_permanent: 0,
                    nb_frame: 0,
                    material,
                    mesh,
                });
                AllocSlot {
                    alloc_id,
                    relative_idx: 0,
                }
            }
            Entry::Occupied(entry) => {
                let alloc_id = *entry.get();
                let alloc = self.allocs[alloc_id];
                if alloc.nb_permanent + alloc.nb_frame + 1 > alloc.block.size {
                    self.allocs[alloc_id].block = self.resize_block(
                        alloc.block,
                        alloc.block.size * 3 / 2 + 4,
                        alloc.block.size * 2 + 4,
                    );
                }
                if is_permanent {
                    self.values.swap(
                        alloc.block.start_offset + alloc.nb_permanent,
                        alloc.block.start_offset + alloc.nb_permanent + alloc.nb_frame,
                    );
                    AllocSlot {
                        alloc_id,
                        relative_idx: alloc.nb_permanent,
                    }
                } else {
                    AllocSlot {
                        alloc_id,
                        relative_idx: alloc.nb_frame + alloc.nb_permanent,
                    }
                }
            }
        }
    }

    pub fn reserve_permanent(&mut self, material: MaterialHandle, mesh: MeshHandle) -> AllocSlot {
        let slot = self.find_slot(material, mesh, true);
        self.allocs[slot.alloc_id].nb_permanent += 1;
        slot
    }
    pub fn set(&mut self, slot: AllocSlot, value: T) {
        let mesh_alloc = self.allocs[slot.alloc_id];
        let idx = mesh_alloc.block.start_offset + slot.relative_idx;
        self.values[idx] = value;
    }
    pub fn add(&mut self, material: MaterialHandle, mesh: MeshHandle, value: T) {
        let slot = self.find_slot(material, mesh, false);
        self.set(slot, value);
        self.allocs[slot.alloc_id].nb_frame += 1;
    }

    pub fn required_total_size(&self) -> usize {
        self.values.len()
    }
    pub fn reset_after_frame(&mut self) {
        for alloc in &mut self.allocs {
            alloc.nb_frame = 0;
        }
    }
    pub fn foreach_block(
        &self,
        material: MaterialHandle,
        mut f: impl FnMut(Range<u32>, MeshHandle),
    ) {
        let Some(alloc_ids) = self.material_to_idx.get(&material) else {
            return;
        };
        for alloc_id in alloc_ids {
            let alloc = self.allocs[*alloc_id];
            debug_assert_eq!(alloc.material, material);
            f(
                alloc.block.start_offset as u32
                    ..(alloc.block.start_offset + alloc.nb_permanent + alloc.nb_frame)
                        as u32,
                alloc.mesh,
            )
        }
    }
}
