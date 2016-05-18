use std::{cell, rc};

use collisions::CollisionData;

pub type Ref<'a, T> = cell::Ref<'a, T>;
pub type RefMut<'a, T> = cell::RefMut<'a, T>;

#[derive(Debug)]
pub struct CollisionDataHandle<D>(rc::Rc<cell::RefCell<CollisionData<D>>>);

impl<D> CollisionDataHandle<D> {
    pub fn new(data: CollisionData<D>) -> CollisionDataHandle<D> {
        CollisionDataHandle(rc::Rc::new(cell::RefCell::new(data)))
    }

    pub fn borrow(&self) -> Ref<CollisionData<D>> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<CollisionData<D>> {
        self.0.borrow_mut()
    }
}

impl<D> Clone for CollisionDataHandle<D> {
    fn clone(&self) -> Self {
        CollisionDataHandle(self.0.clone())
    }
}
