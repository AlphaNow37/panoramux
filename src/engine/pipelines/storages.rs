use crate::engine::components::color::Color;
use crate::theory::datastrutures::arena::{Arena, ArenaIndex};
use std::any::type_name;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct ItemHandle<T>(ArenaIndex<T>);
impl<T> ItemHandle<T> {
    pub fn to_int(self) -> usize {
        self.0.to_int()
    }
    fn cast<U>(self) -> ItemHandle<U> {
        ItemHandle(self.0.cast())
    }
}

pub struct ItemStore<U: Into<T>, T: bytemuck::Pod + Into<U>> {
    buffer: wgpu::Buffer,
    data: Arena<T>,
    should_be_updated: bool,
    binding: u32,
    u: PhantomData<U>
}
impl<U: Into<T>, T: bytemuck::Pod + Into<U>> ItemStore<U, T> {
    fn get_buffer(device: &wgpu::Device, capacity: usize) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            size: capacity as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
            label: Some(format!("Item store of {}", type_name::<T>()).leak()),
        })
    }
    pub fn new(device: &wgpu::Device, binding: u32) -> Self {
        Self {
            data: Arena::new(),
            buffer: Self::get_buffer(device, 1024),
            should_be_updated: false,
            binding,
            u: PhantomData,
        }
    }
    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, should_recreate_bind_group: &mut bool) {
        if self.should_be_updated {
            let required_space = self.data.data.len() * size_of::<T>();
            if (self.buffer.size() as usize) < required_space {
                self.buffer.destroy();
                self.buffer = Self::get_buffer(&device, (required_space+1) * 10);
                *should_recreate_bind_group = true;
            }
            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&self.data.data));
        }
    }
    pub fn set(&mut self, idx: ItemHandle<U>, value: U) {
        self.should_be_updated = true;
        self.data.set(idx.0.cast(), value.into());
    }
    pub fn get(&self, idx: ItemHandle<U>) -> U {
        self.data.get(idx.0.cast()).clone().into()
    }
    pub fn add(&mut self, value: U) -> ItemHandle<U> {
        self.should_be_updated = true;
        ItemHandle(self.data.add(value.into())).cast()
    }
    pub fn remove(&mut self, idx: ItemHandle<U>) {
        self.data.remove(idx.0.cast())
    }
    pub fn reserve_one(&mut self) -> ItemHandle<U> {
        self.should_be_updated = true;
        ItemHandle(self.data.add(T::zeroed())).cast()
    }
}

pub type ColorItemStore = ItemStore<Color, [f32; 4]>;

pub struct ItemStores {
    pub colors: ColorItemStore,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}
impl ItemStores {
    fn get_bind_group(device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout, colors: &ColorItemStore) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Item stores bind group"),
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: colors.binding,
                    resource: colors.buffer.as_entire_binding()
                }
            ]
        })
    }
    pub fn new(device: &wgpu::Device) -> Self {
        let colors = ItemStore::new(device, 0);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Item stores bind groupe layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: colors.binding,
                    count: None,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage {
                            read_only: true,
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    }
                }
            ]
        });
        let bind_group = Self::get_bind_group(device, &bind_group_layout, &colors);

        Self {
            colors,
            bind_group_layout,
            bind_group,
        }
    }
    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut should_recreate_bind_group = false;
        self.colors.update(device, queue, &mut should_recreate_bind_group);
        if should_recreate_bind_group {
            self.bind_group = Self::get_bind_group(device, &self.bind_group_layout, &self.colors);
        }
    }
    pub fn put(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_bind_group(1, Some(&self.bind_group), &[]);
    }
}
