//! Contains the implementation of the Mach Physics engine, an open-source
//! physics engine built on the Rust programming language.

#![crate_name = "mach"]
#![crate_type = "lib"]

// TODO: renable this, undergoing lots of changes, docs will just slow me down
// #![warn(missing_docs)]

#[macro_use]
#[cfg(test)]
#[path="../tests/support/mod.rs"]
pub mod support;

mod world;
mod mach_world;
mod custom_world;

pub mod utils;
#[macro_use]
pub mod maths;
pub mod shapes;
pub mod dynamics;
pub mod entities;
pub mod detection;
pub mod broadphase;
pub mod narrowphase;

pub use self::maths::Vect;
pub use self::world::World;
pub use self::shapes::{Shape, ShapeSpec};
pub use self::dynamics::{Dynamics, MachDynamics};
pub use self::entities::{Body, RigidBody, StaticBody};
pub use self::detection::{MachSpace, Space};
pub use self::mach_world::MachWorld;
pub use self::custom_world::CustomWorld;

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

/// A shared pointer which gives access to the contained type instance.
pub type SharedCell<T> = Rc<RefCell<T>>;

macro_rules! set_precision {
    ($precision:ident, $tolerance:expr) => {
        use std::$precision;

        /// A floating point type used throughout the engine, depends on the precision
        /// that the engine was compiled with.
        pub type Scalar = $precision;

        /// The PI constant. This is simply an alias for the constants in the standard
        /// library, depending on the precision the library was compiled with.
        pub static PI: $precision = $precision::consts::PI;
        /// Infinity. This is simply an alias for the constants in the standard library,
        /// depending on the precision the library was compiled with.
        pub static INFINITY: $precision = $precision::INFINITY;
        /// Negative infinity. This is simply an alias for the constants in the standard
        /// library, depending on the precision the library was compiled with.
        pub static NEG_INFINITY: $precision = $precision::NEG_INFINITY;
        /// The tolerance used to resolve floating point differences.
        pub static TOLERANCE: $precision = $tolerance;
    };
}

#[cfg(feature = "high_precision")]
set_precision!(f64, 1e-6);

#[cfg(not(feature = "high_precision"))]
set_precision!(f32, 1e-4);

/// A unique identifier used to uniquely identify entities in the engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ID(u32);

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID({})", self.0)
    }
}

// TODO will this be the future design?

pub mod temp {
    use std::cell::{Ref, RefMut};

    use {ID, Scalar};
    use maths::{IntegratableMut, Integrator, Vect};
    use entities::{Body, BodyHandle, BodyParams, EntityStore};
    use detection::{ContactEvent, Detection};
    use broadphase::Broadphase;
    use narrowphase::Narrowphase;

    struct World<B: Broadphase<EntityStore=ES>, N: Narrowphase, D: Detection, ES: EntityStore, I: Integrator, CS: ConstraintSolver> {
        broadphase: B,
        narrowphase: N,
        detection: D,
        entity_store: ES,
        integrator: I,
        constraint_solver: CS,
    }

    impl<B, N, D, ES, I, CS> World<B, N, D, ES, I, CS> where B: Broadphase<EntityStore=ES>, N: Narrowphase, D: Detection, ES: EntityStore, I: Integrator, CS: ConstraintSolver {
        pub fn update(&mut self, time_step: Scalar) -> Vec<ContactEvent> {
            // update entity positions
            for mut integratable in self.entity_store.integratable_iter_mut() {
                self.integrator.integrate_in_place(&mut integratable, time_step, Vect::zero());
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
                self.constraint_solver.solve_in_place(&mut self.entity_store, time_step, &contact_events);

                self.narrowphase.update();
                self.broadphase.update(&self.narrowphase);
            }

            return contact_events;
        }

        fn notify_body_created(&mut self, id: ID) {
            let body_handle = self.entity_store.find_body_handle(id)
                .expect("expected to find body that was just created, but failed!");

            self.narrowphase.notify_body_created(body_handle);
            self.broadphase.notify_body_created(&self.entity_store, body_handle);
        }
    }

    impl<B, N, D, ES, I, CS> EntityStore for World<B, N, D, ES, I, CS> where B: Broadphase<EntityStore=ES>, N: Narrowphase, D: Detection, ES: EntityStore, I: Integrator, CS: ConstraintSolver {
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

        fn find_body_handle(&self, id: ID) -> Option<&BodyHandle> {
            self.entity_store.find_body_handle(id)
        }

        fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Box<Body>>> + 'a> {
            self.entity_store.bodies_iter()
        }

        fn bodies_iter_mut<'a>(&'a mut self) -> Box<Iterator<Item=RefMut<Box<Body>>> + 'a> {
            self.entity_store.bodies_iter_mut()
        }

        fn integratable_iter_mut<'a, 'b>(&'a mut self) -> Box<Iterator<Item=IntegratableMut> + 'a> {
            self.entity_store.integratable_iter_mut()
        }
    }

    trait ConstraintSolver {
        fn solve_in_place<ES: EntityStore>(&self, store: &mut ES, time_step: Scalar, contact_events: &Vec<ContactEvent>);
    }
}
