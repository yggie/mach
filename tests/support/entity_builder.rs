use std::mem;

use Scalar;
use maths::Vect;
use entities::{BodyParams, Material, RigidBody, Body};

use support::inputs;

#[derive(Clone)]
pub struct EntityBuilder {
    shape: inputs::ShapeDesc,
    rotation: inputs::UnitQuat,
    translation: inputs::Vect,
}

impl EntityBuilder {
    pub fn cube(size: Scalar) -> EntityBuilder {
        EntityBuilder {
            shape: inputs::ShapeDesc::Cuboid(size, size, size),
            .. EntityBuilder::default()
        }
    }

    pub fn with_translation(self, x: Scalar, y: Scalar, z: Scalar) -> EntityBuilder {
        EntityBuilder {
            translation: inputs::Vect(x, y, z),
            .. self
        }
    }

    pub fn with_translation_vect(self, vect: Vect) -> EntityBuilder {
        self.with_translation(vect.x, vect.y, vect.z)
    }

    pub fn with_rotation(self, unit_quat: inputs::UnitQuat) -> EntityBuilder {
        EntityBuilder {
            rotation: unit_quat,
            .. self
        }
    }

    pub fn build_body(self) -> Box<Body> {
        let translation: Vect = self.translation.into();

        Box::new(RigidBody::with_id(
            unsafe { mem::transmute(0u32) },
            &BodyParams::shape(self.shape.into())
                .with_material(Material::default())
                .with_translation(translation.x, translation.y, translation.z)
                .with_rotation(self.rotation.into()),
        ))
    }
}

impl Default for EntityBuilder {
    fn default() -> EntityBuilder {
        EntityBuilder {
            shape: inputs::ShapeDesc::Cuboid(1.0, 1.0, 1.0),
            rotation: inputs::UnitQuat::identity(),
            translation: inputs::Vect(0.0, 0.0, 0.0),
        }
    }
}
