extern crate mach;

use std::iter::Cycle;
use std::slice::Iter;

static COLORS: [[f32; 4]; 3] = [
    [1.0, 0.0, 0.0, 1.0],
    [0.0, 1.0, 0.0, 1.0],
    [0.0, 0.0, 1.0, 1.0],
];

pub struct Instance {
    pub id: mach::ID,
    pub color: [f32; 4],
    pub shape_spec: mach::ShapeSpec,
}

pub struct InstanceFactory {
    color_generator: Cycle<Iter<'static, [f32; 4]>>,
}

impl InstanceFactory {
    pub fn new() -> InstanceFactory {
        InstanceFactory {
            color_generator: COLORS.into_iter().cycle(),
        }
    }

    pub fn generate(&mut self, id: mach::ID, shape: &mach::Shape) -> Instance {
        Instance {
            id: id,
            color: *self.color_generator.next().unwrap(),
            shape_spec: shape.spec(),
        }
    }
}
