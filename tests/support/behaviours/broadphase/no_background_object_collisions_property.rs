use collisions::Broadphase;

use tests::support::{Property, PropertyCheck};
use tests::support::behaviours::broadphase::BroadphaseAction;

pub struct NoBackgroundObjectCollisionsProperty { }

impl NoBackgroundObjectCollisionsProperty {
    pub fn new<T>() -> Box<Property<T, Action=BroadphaseAction>> where T: Broadphase<()> {
        Box::new(NoBackgroundObjectCollisionsProperty { })
    }
}

impl<T> Property<T> for NoBackgroundObjectCollisionsProperty where T: Broadphase<()> {
    type Action = BroadphaseAction;

    fn property_check_for(&self, _broadphase: &T, _action: &Self::Action) -> Option<Box<PropertyCheck<T>>> {
        // TODO apply for only mutating actions?
        return Some(Box::new(NoBackgroundObjectCollisionsCheck { }) as Box<PropertyCheck<T>>);
    }
}

struct NoBackgroundObjectCollisionsCheck { }

impl<T> PropertyCheck<T> for NoBackgroundObjectCollisionsCheck where T: Broadphase<()> {
    fn assert(&self, broadphase: &T) -> Result<(), String> {
        for objects in broadphase.possible_collision_pairs_iter() {
            if objects.0.is_background && objects.1.is_background {
                // TODO aggregate all pairs?
                return Err(format!("expected background objects not to collide, but two background objects did collide ({} and {})", objects.0.id, objects.1.id));
            }
        }

        return Ok(());
    }
}
