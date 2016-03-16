use std::cell::{Ref, RefMut};

use {ID, Scalar, World};
use maths::{Integrator, Vect};
use solvers::ConstraintSolver;
use entities::{Body, BodyHandle, BodyParams, EntityStore, RigidBody};
use detection::{ContactEvent, Detection};
use broadphase::Broadphase;
use narrowphase::Narrowphase;

pub struct CustomWorld<B: Broadphase<EntityStore=ES>, N: Narrowphase, D: Detection, ES: EntityStore, I: Integrator, CS: ConstraintSolver> {
    pub broadphase: B,
    pub narrowphase: N,
    pub detection: D,
    pub entity_store: ES,
    pub integrator: I,
    pub constraint_solver: CS,
    pub gravity: Vect,
}

impl<B, N, D, ES, I, CS> World for CustomWorld<B, N, D, ES, I, CS> where B: Broadphase<EntityStore=ES>, N: Narrowphase, D: Detection, ES: EntityStore, I: Integrator, CS: ConstraintSolver {
    fn update(&mut self, time_step: Scalar) -> Vec<ContactEvent> {
        // update entity positions
        for mut rigid_body in self.entity_store.rigid_body_iter_mut() {
            self.integrator.integrate_in_place(&mut rigid_body.as_integratable_mut(), time_step, self.gravity);
        }

        self.narrowphase.update();
        self.broadphase.update(&self.narrowphase);

        let potentially_colliding_pairs: Vec<(BodyHandle, BodyHandle)> = self.broadphase.contact_candidate_pairs_iter(&self.entity_store)
            // TODO something like: .map(|pair| self.entity_store.preload_transform(pair))
            .filter(|handles| self.narrowphase.test(&handles.0, &handles.1))
            .collect();

        let contact_events: Vec<ContactEvent> = potentially_colliding_pairs.iter()
            .filter_map(|pair| {
                self.detection.compute_contacts(&pair.0, &pair.1)
            })
        .collect();

        if contact_events.len() > 0 {
            self.constraint_solver.solve_with_contacts(time_step, &contact_events);

            self.narrowphase.update();
            self.broadphase.update(&self.narrowphase);
        }

        return contact_events;
    }

    fn set_gravity(&mut self, gravity: Vect) {
        self.gravity = gravity;
    }
}

impl<B, N, D, ES, I, CS> CustomWorld<B, N, D, ES, I, CS> where B: Broadphase<EntityStore=ES>, N: Narrowphase, D: Detection, ES: EntityStore, I: Integrator, CS: ConstraintSolver {
    fn notify_body_created(&mut self, id: ID) {
        let body_handle = self.entity_store.find_body_handle(id)
            .expect("expected to find body that was just created, but failed!");

        self.narrowphase.notify_body_created(body_handle);
        self.broadphase.notify_body_created(&self.entity_store, body_handle);
    }
}

impl<B, N, D, ES, I, CS> EntityStore for CustomWorld<B, N, D, ES, I, CS> where B: Broadphase<EntityStore=ES>, N: Narrowphase, D: Detection, ES: EntityStore, I: Integrator, CS: ConstraintSolver {
    fn create_rigid_body(&mut self, params: &BodyParams) -> ID {
        let id = self.entity_store.create_rigid_body(params);

        self.notify_body_created(id);

        return id;
    }

    fn create_static_body(&mut self, params: &BodyParams) -> ID {
        let id = self.entity_store.create_static_body(params);

        self.notify_body_created(id);

        return id;
    }

    fn find_body(&self, id: ID) -> Option<Ref<Box<Body>>> {
        self.entity_store.find_body(id)
    }

    fn find_rigid_body(&self, id: ID) -> Option<Ref<Box<RigidBody>>> {
        self.entity_store.find_rigid_body(id)
    }

    fn find_body_handle(&self, id: ID) -> Option<&BodyHandle> {
        self.entity_store.find_body_handle(id)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Box<Body>>> + 'a> {
        self.entity_store.bodies_iter()
    }

    fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=RefMut<Box<Body>>> + 'a> {
        self.entity_store.bodies_iter_mut()
    }

    fn rigid_body_iter_mut<'a, 'b>(&'a mut self) -> Box<Iterator<Item=RefMut<Box<RigidBody>>> + 'a> {
        self.entity_store.rigid_body_iter_mut()
    }
}
