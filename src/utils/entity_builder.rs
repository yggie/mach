use std::rc::Rc;
use std::cell::{Ref, RefCell};

use {ID, Scalar};
use maths::{Quat, Vect};
use entities::{BodyParams, EntityStore, ShapeDesc};

pub struct EntityBuilder<'a, ES: EntityStore + 'static> {
    entity_store: Rc<RefCell<&'a mut ES>>,
    params: BodyParams,
}

impl<'a, ES: EntityStore> EntityBuilder<'a, ES> {
    inline_chainable_params_methods! {
        struct_signature: EntityBuilder<'a, ES>,
        struct_name: EntityBuilder,
        field_name: params,
    }

    pub fn from_store(store: &mut ES) -> EntityBuilder<ES> {
        EntityBuilder {
            entity_store: Rc::new(RefCell::new(store)),
            params: BodyParams::default(),
        }
    }

    pub fn entity_store(&self) -> Ref<&mut ES> {
        self.entity_store.borrow()
    }

    pub fn create_rigid_body(self) -> ID {
        self.entity_store.borrow_mut().create_rigid_body(&self.params)
    }

    pub fn create_static_body(self) -> ID {
        self.entity_store.borrow_mut().create_static_body(&self.params)
    }
}

impl<'a, ES> Clone for EntityBuilder<'a, ES> where ES: EntityStore {
    fn clone(&self) -> Self {
        EntityBuilder {
            entity_store: self.entity_store.clone(),
            params: self.params.clone(),
        }
    }
}
