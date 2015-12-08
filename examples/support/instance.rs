extern crate mach;

use std::rc::Rc;
use std::iter::Cycle;
use std::slice::Iter;

use support::PolygonModel;

static COLORS: [(f32, f32, f32, f32); 3] = [
    (1.0, 0.0, 0.0, 1.0),
    (0.0, 1.0, 0.0, 1.0),
    (0.0, 0.0, 1.0, 1.0),
];

pub struct Instance {
    pub id: mach::ID,
    pub color: (f32, f32, f32, f32),
    pub scale: (f32, f32, f32),
    pub polygon_model: Rc<PolygonModel>,
}

pub struct InstanceFactory {
    color_generator: Cycle<Iter<'static, (f32, f32, f32, f32)>>,
}

impl InstanceFactory {
    pub fn new() -> InstanceFactory {
        InstanceFactory {
            color_generator: COLORS.into_iter().cycle(),
        }
    }

    pub fn generate(&mut self, id: mach::ID, scale: (f32, f32, f32), polygon_model: Rc<PolygonModel>) -> Instance {
        Instance {
            id: id,
            scale: scale,
            color: *self.color_generator.next().unwrap(),
            polygon_model: polygon_model,
        }
    }
}
