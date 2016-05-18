use std::cell::{Ref, RefCell};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ResultHandle<T>(Arc<RefCell<Option<T>>>);

impl<T> ResultHandle<T> {
    pub fn new() -> ResultHandle<T> {
        ResultHandle(Arc::new(RefCell::new(None)))
    }

    pub fn set_data(&mut self, data: T) {
        *self.0.borrow_mut() = Some(data);
    }

    pub fn borrow(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |option| option.as_ref().unwrap())
    }
}
