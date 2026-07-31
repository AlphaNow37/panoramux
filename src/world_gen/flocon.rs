use rand::{Rng, RngExt};
use crate::engine::pipelines::mesh::Mesh;

enum FloconBranche {
    Coupure {
        middle: Box<FloconBranche>,
        side: Box<FloconBranche>,
        pre_length: f32,
    }, // middle, sides
    ExtrCarre {
        pre_length: f32,
    },
}

impl FloconBranche {
    pub fn new_random(size_budget: f32, curr_height: f32, rng: &mut impl Rng) -> Self {
        if size_budget < 1. {
            return Self::ExtrRonde {

            }
        }
        Self::Coupure(
            Box::new(Self::new_random(
                size_budget / rng.random_range(1.5..2.0),
                rng,
            )),
            Box::new(Self::new_random(
                size_budget / rng.random_range(2.2..3.0),
                rng,
            )),
        )
    }
    pub fn add_to_mesh(&self, mesh: &mut Mesh) {

    }
}

pub struct Flocon {
    branches: FloconBranche,
}
