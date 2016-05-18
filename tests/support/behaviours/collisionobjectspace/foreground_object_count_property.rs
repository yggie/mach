use collisions::CollisionObjectSpace;

use tests::support::{Property, PropertyCheck};
use tests::support::behaviours::collisionobjectspace::CollisionObjectSpaceAction;

pub struct ForegroundObjectCountProperty { }

impl ForegroundObjectCountProperty {
    pub fn new<T>() -> Box<Property<T, Action=CollisionObjectSpaceAction>> where T: CollisionObjectSpace<()>{
        Box::new(ForegroundObjectCountProperty { })
    }
}

impl<T> Property<T> for ForegroundObjectCountProperty where T: CollisionObjectSpace<()> {
    type Action = CollisionObjectSpaceAction;

    fn property_check_for(&self, object_space: &T, action: &Self::Action) -> Option<Box<PropertyCheck<T>>> {
        let foreground_objects_count = object_space.foreground_objects_iter().count();
        let check = match action {
            &CollisionObjectSpaceAction::CreateForegroundObject(ref _handle) => {
                ForegroundObjectCountCheck {
                    count_before: foreground_objects_count,
                    expected_change: 1,
                }
            },

            _otherwise => {
                ForegroundObjectCountCheck {
                    count_before: foreground_objects_count,
                    expected_change: 0,
                }
            },
        };

        return Some(Box::new(check) as Box<PropertyCheck<T>>);
    }
}

#[derive(Clone)]
struct ForegroundObjectCountCheck {
    count_before: usize,
    expected_change: isize,
}

impl<T> PropertyCheck<T> for ForegroundObjectCountCheck where T: CollisionObjectSpace<()> {
    fn assert(&self, object_space: &T) -> Result<(), String> {
        let foreground_objects_count = object_space.foreground_objects_iter().count();

        return if foreground_objects_count as isize == self.count_before as isize + self.expected_change {
            Ok(())
        } else {
            Err(format!("expected the number of foreground objects to change by {}, but was instead changed by {}", self.expected_change, foreground_objects_count - self.count_before))
        };
    }
}
