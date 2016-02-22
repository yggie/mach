use entities::BodyHandle;
use detection::ContactSet;

pub struct ContactEvent {
    pub bodies: (BodyHandle, BodyHandle),
    pub contact_set: ContactSet,
}
