extern crate quickcheck;

use collisions::Broadphase;

use tests::support::Action;
use tests::support::behaviours::collisionobjectspace::CollisionObjectSpaceAction;

#[derive(Clone, Debug)]
pub enum BroadphaseAction {
    Update,
    CollisionObjectSpaceAction(CollisionObjectSpaceAction),
}

impl<T> Action<T> for BroadphaseAction where T: Broadphase<()> {
    fn perform(&mut self, target: &mut T) {
        match self {
            &mut BroadphaseAction::Update => {
                target.update();
            },

            &mut BroadphaseAction::CollisionObjectSpaceAction(ref mut action) => {
                action.perform(target);
            },
        }
    }
}

impl quickcheck::Arbitrary for BroadphaseAction {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        match random.next_u32() % 5 {
            0 => BroadphaseAction::Update,

            1...4 => BroadphaseAction::CollisionObjectSpaceAction(quickcheck::Arbitrary::arbitrary(random)),
            _otherwise => unreachable!(),
        }
    }
}
