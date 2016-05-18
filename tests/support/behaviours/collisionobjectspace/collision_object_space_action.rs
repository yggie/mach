extern crate quickcheck;

use maths::Transform;
use shapes::Cuboid;
use collisions::{CollisionData, CollisionObject, CollisionObjectSpace};

use tests::support::{Action, ResultHandle};

#[derive(Clone, Debug)]
pub enum CollisionObjectSpaceAction {
    CreateForegroundObject(ResultHandle<CollisionObject<()>>),
    CreateBackgroundObject(ResultHandle<CollisionObject<()>>),
}

impl<T> Action<T> for CollisionObjectSpaceAction where T: CollisionObjectSpace<()> {
    fn perform(&mut self, target: &mut T) {
        match self {
            &mut CollisionObjectSpaceAction::CreateForegroundObject(ref mut handle) => {
                handle.set_data(target.create_foreground_object(test_prototype()));
            },

            &mut CollisionObjectSpaceAction::CreateBackgroundObject(ref mut handle) => {
                handle.set_data(target.create_background_object(test_prototype()));
            },
        }
    }
}

unsafe impl Send for CollisionObjectSpaceAction { }

impl quickcheck::Arbitrary for CollisionObjectSpaceAction {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        match random.next_u32() % 2 {
            0 => CollisionObjectSpaceAction::CreateForegroundObject(ResultHandle::new()),
            1 => CollisionObjectSpaceAction::CreateBackgroundObject(ResultHandle::new()),
            _otherwise => unreachable!(),
        }
    }
}

fn test_prototype() -> CollisionData<()> {
    CollisionData::<()>::test_dummy(Box::new(Cuboid::cube(1.0)), Transform::identity())
}
