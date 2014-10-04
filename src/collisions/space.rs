use bodies::Body;
use collisions::Contact;

use std::rc::Rc;

/// A Space is a data structure for storing and querying space.
pub trait Space<'a> {
    /// Adds a new physical entity to the structure.
    fn add(&mut self, Rc<Body>);
    /// Returns the number of physical entities in the structure.
    fn size(&self) -> uint;
    /// Traverses the structure to look for any contact. Once a contact is
    /// encountered, the callback function is immediately called.
    fn each_contact(&mut self, |Contact<'a>|);
}
