use std::mem;

use Scalar;
use maths::{State, Transform};
use entities::{Material, RigidBody, Body};

use support::inputs;

#[derive(Clone)]
pub struct EntityBuilder {
    shape: inputs::Shape,
    rotation: inputs::UnitQuat,
    translation: inputs::Vect,
}

impl EntityBuilder {
    pub fn new_cube(size: Scalar) -> EntityBuilder {
        EntityBuilder {
            shape: inputs::Shape::Cuboid(size, size, size),
            rotation: inputs::UnitQuat::identity(),
            translation: inputs::Vect {
                values: (0.0, 0.0, 0.0)
            }
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
        let transform = Transform::new(self.translation.to_value(), self.rotation.to_value());

        Box::new(RigidBody::new_with_id(
            unsafe { mem::transmute(0u32) },
            self.shape.to_value(),
            &Material::default(),
            State::new_from_transform(&transform),
        ))
    }
}
