use std::{cell, rc};

use collisions::{CollisionData, NarrowphaseData};

pub type Ref<'a, T> = cell::Ref<'a, T>;
pub type RefMut<'a, T> = cell::RefMut<'a, T>;

#[derive(Debug)]
pub struct CollisionDataHandle<T>(rc::Rc<cell::RefCell<CollisionData<T>>>) where T: NarrowphaseData;

impl<T> CollisionDataHandle<T> where T: NarrowphaseData {
    pub fn new(data: CollisionData<T>) -> CollisionDataHandle<T> {
        CollisionDataHandle(rc::Rc::new(cell::RefCell::new(data)))
    }

    pub fn borrow(&self) -> Ref<CollisionData<T>> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<CollisionData<T>> {
        self.0.borrow_mut()
    }
}

impl<T> Clone for CollisionDataHandle<T> where T: NarrowphaseData {
    fn clone(&self) -> Self {
        CollisionDataHandle(self.0.clone())
    }
}
