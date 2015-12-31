use std::mem;

use Scalar;
use entities::{BodyParams, Material, RigidBody, Body};

use support::inputs;

#[derive(Clone)]
pub struct EntityBuilder {
    shape: inputs::ShapeDesc,
    rotation: inputs::UnitQuat,
    translation: inputs::Vect,
}

impl EntityBuilder {
    pub fn new_cube(size: Scalar) -> EntityBuilder {
        EntityBuilder {
            shape: inputs::ShapeDesc::Cuboid(size, size, size),
            .. EntityBuilder::default()
        }
    }

    pub fn with_translation(self, x: Scalar, y: Scalar, z: Scalar) -> EntityBuilder {
        EntityBuilder {
            translation: inputs::Vect {
                values: (x, y, z)
            },
            .. self
        }
    }

    pub fn with_rotation(self, unit_quat: inputs::UnitQuat) -> EntityBuilder {
        EntityBuilder {
            rotation: unit_quat,
            .. self
        }
    }

    pub fn build_body(self) -> Box<Body> {
        Box::new(RigidBody::new_with_id(
            unsafe { mem::transmute(0u32) },
            &BodyParams::default()
                .as_shape(self.shape.to_value())
                .with_material(Material::default())
                .with_translation(self.translation.to_value())
                .with_rotation(self.rotation.to_value()),
        ))
    }
}

impl Default for EntityBuilder {
    fn default() -> EntityBuilder {
        EntityBuilder {
            shape: inputs::ShapeDesc::Cuboid(1.0, 1.0, 1.0),
            rotation: inputs::UnitQuat::identity(),
            translation: inputs::Vect {
                values: (0.0, 0.0, 0.0)
            }
        }
    }
}
