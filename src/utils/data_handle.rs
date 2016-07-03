use std::{cell, rc};

pub type Ref<'a, T> = cell::Ref<'a, T>;
pub type RefMut<'a, T> = cell::RefMut<'a, T>;

#[derive(Debug)]
pub struct DataHandle<T>(rc::Rc<cell::RefCell<T>>);

impl<T> DataHandle<T> {
    pub fn new(data: T) -> DataHandle<T> {
        DataHandle(rc::Rc::new(cell::RefCell::new(data)))
    }

    pub fn borrow(&self) -> Ref<T> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.0.borrow_mut()
    }
}

impl<T> Clone for DataHandle<T> {
    fn clone(&self) -> Self {
        DataHandle(self.0.clone())
    }
}
