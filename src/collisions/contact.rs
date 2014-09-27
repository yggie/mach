use bodies::Body;

use std::rc::Rc;

#[deriving(Clone)]
pub struct Contact {
    a: Rc<Body>,
    b: Rc<Body>,
}

impl Contact {
    pub fn new(a: Rc<Body>, b: Rc<Body>) -> Contact {
        Contact{ a: a, b: b }
    }
}
