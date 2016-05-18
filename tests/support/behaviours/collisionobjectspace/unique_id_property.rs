use ID;
use collisions::{CollisionObject, CollisionObjectSpace};

use tests::support::{Property, PropertyCheck, ResultHandle};
use tests::support::behaviours::collisionobjectspace::CollisionObjectSpaceAction;

pub struct UniqueIDProperty { }

impl UniqueIDProperty {
    pub fn new<T>() -> Box<Property<T, Action=CollisionObjectSpaceAction>> where T: CollisionObjectSpace<()> {
        Box::new(UniqueIDProperty { })
    }
}

impl<T> Property<T> for UniqueIDProperty where T: CollisionObjectSpace<()> {
    type Action = CollisionObjectSpaceAction;

    fn property_check_for(&self, object_space: &T, action: &Self::Action) -> Option<Box<PropertyCheck<T>>> {
        let existing_ids: Vec<ID> = object_space.objects_iter().map(|obj| obj.id).collect();
        let check = match action {
            &CollisionObjectSpaceAction::CreateForegroundObject(ref handle) => {
                UniqueIDCheck {
                    existing_ids: existing_ids,
                    result_handle: handle.clone(),
                }
            },

            &CollisionObjectSpaceAction::CreateBackgroundObject(ref handle) => {
                UniqueIDCheck {
                    existing_ids: existing_ids,
                    result_handle: handle.clone(),
                }
            },
        };

        return Some(Box::new(check) as Box<PropertyCheck<T>>);
    }
}

struct UniqueIDCheck {
    existing_ids: Vec<ID>,
    result_handle: ResultHandle<CollisionObject<()>>,
}

impl<T> PropertyCheck<T> for UniqueIDCheck where T: CollisionObjectSpace<()> {
    fn assert(&self, _object_space: &T) -> Result<(), String> {
        let result = self.result_handle.borrow();

        if self.existing_ids.contains(&result.id) {
            Err(format!("expected the generated ID ({}) to be unique, but was already included in {:?}", result.id, self.existing_ids))
        } else {
            Ok(())
        }
    }
}
