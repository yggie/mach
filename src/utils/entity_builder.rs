use std::rc::Rc;
use std::cell::RefCell;

use {ID, Scalar};
use maths::{Quat, Vect};
use entities::{BodyParams, EntityStore, ShapeDesc};

#[derive(Clone)]
pub struct EntityBuilder<'a, S: EntityStore + 'static> {
    entity_store: Rc<RefCell<&'a mut S>>,
    params: BodyParams,
}

impl<'a, S: EntityStore> EntityBuilder<'a, S> {
    inline_chainable_params_methods! {
        struct_signature: EntityBuilder<'a, S>,
        struct_name: EntityBuilder,
        field_name: params,
    }

    pub fn from_store(store: &mut S) -> EntityBuilder<S> {
        EntityBuilder {
            entity_store: Rc::new(RefCell::new(store)),
            params: BodyParams::default(),
        }
    }

    pub fn create_rigid_body(self) -> ID {
        self.entity_store.borrow_mut().create_rigid_body(&self.params)
    }
}
