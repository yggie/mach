extern crate quickcheck;

use std::mem;

use mach::maths::State;
use mach::entities;
use mach::entities::{Material, RigidBody};

use support::inputs;

#[derive(Clone, Debug)]
pub struct VolumetricBody {
    shape: inputs::Shape,
    transform: inputs::Transform,
}

impl VolumetricBody {
    pub fn to_object(self) -> Box<entities::VolumetricBody> {
        Box::new(RigidBody::new_with_id(
            unsafe { mem::transmute(0u32) },
            self.shape.to_object(),
            &Material::default(),
            State::new_from_transform(&self.transform.to_object()),
        ))
    }
}

impl quickcheck::Arbitrary for VolumetricBody {
    fn arbitrary<G: quickcheck::Gen>(generator: &mut G) -> Self {
        VolumetricBody {
            shape: quickcheck::Arbitrary::arbitrary(generator),
            transform: quickcheck::Arbitrary::arbitrary(generator),
        }
    }
}
