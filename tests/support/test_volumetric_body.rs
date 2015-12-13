extern crate quickcheck;

use std::mem;

use mach::maths::State;
use mach::entities::{Material, RigidBody, VolumetricBody};

use support::{TestShape, TestTransform};

#[derive(Clone, Debug)]
pub struct TestVolumetricBody {
    shape: TestShape,
    transform: TestTransform,
}

impl TestVolumetricBody {
    pub fn as_volumetric_body(self) -> Box<VolumetricBody> {
        Box::new(RigidBody::new_with_id(
            unsafe { mem::transmute(0u32) },
            self.shape.as_shape(),
            &Material::default(),
            State::new_from_transform(&self.transform.as_transform()),
        ))
    }
}

impl quickcheck::Arbitrary for TestVolumetricBody {
    fn arbitrary<G: quickcheck::Gen>(generator: &mut G) -> Self {
        TestVolumetricBody {
            shape: quickcheck::Arbitrary::arbitrary(generator),
            transform: quickcheck::Arbitrary::arbitrary(generator),
        }
    }
}
