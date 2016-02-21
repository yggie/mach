use ID;
use detection::ContactSet;

pub struct ContactEvent {
    pub bodies: (ID, ID),
    pub contact_set: ContactSet,
}
