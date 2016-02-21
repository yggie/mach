use std::{rc, cell};

use entities::Body;

pub type Rc<T> = rc::Rc<T>;
pub type Ref<'a, T> = cell::Ref<'a, T>;
pub type RefMut<'a, T> = cell::RefMut<'a, T>;

pub struct BodyHandle(Rc<cell::RefCell<Box<Body>>>);

impl BodyHandle {
    pub fn new(value: Box<Body>) -> BodyHandle {
        BodyHandle(Rc::new(cell::RefCell::new(value)))
    }

    pub fn borrow(&self) -> Ref<Box<Body>> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<Box<Body>> {
        self.0.borrow_mut()
    }
}

impl Clone for BodyHandle {
    fn clone(&self) -> Self {
        BodyHandle(self.0.clone())
    }
}
