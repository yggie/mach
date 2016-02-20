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
pub mod geometries;

pub use self::maths::Vect;
pub use self::world::World;
pub use self::shapes::{Shape, ShapeSpec};
pub use self::dynamics::{Dynamics, MachDynamics};
pub use self::entities::{RigidBody, StaticBody};
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
    use std;
    use std::cell::{Ref, RefMut};

    use {ID, Scalar};
    use maths::{IntegratableMut, Integrator, Vect};
    use entities::{Body, BodyParams, EntityStore};
    use broadphase::Broadphase;

    struct World<B: Broadphase<EntityStore=ES>, N: Narrowphase, CD: ContactDetection, ES: EntityStore, I: Integrator, CS: ConstraintSolver> {
        broadphase: B,
        narrowphase: N,
        contact_detector: CD,
        entity_store: ES,
        integrator: I,
        constraint_solver: CS,
    }

    struct Contact(u32);

    impl<B, N, CD, ES, I, CS> World<B, N, CD, ES, I, CS> where B: Broadphase<EntityStore=ES>, N: Narrowphase, CD: ContactDetection, ES: EntityStore, I: Integrator, CS: ConstraintSolver {
        pub fn update(&mut self, time_step: Scalar) -> Vec<Contact> {
            // update entity positions
            for mut integratable in self.entity_store.integratable_iter_mut() {
                self.integrator.integrate_in_place(&mut integratable, time_step, Vect::zero());
            }

            self.narrowphase.update(&self.entity_store);
            self.broadphase.update(&self.entity_store, &self.narrowphase);

            let entity_pairs: Vec<(ID, ID)> = self.broadphase.contact_candidate_pairs_iter(&self.entity_store)
                // TODO something like: .map(|pair| self.entity_store.preload_transform(pair))
                .filter(|&pair| self.narrowphase.test(pair))
                .collect();

            let contacts: Vec<Contact> = entity_pairs.iter()
                .fold(Box::new(std::iter::empty()) as Box<Iterator<Item=Contact>>, |iter: Box<Iterator<Item=Contact>>, pair: &(ID, ID)| -> Box<Iterator<Item=Contact>> {
                    Box::new(iter.chain(self.contact_detector.contacts_iter(*pair)))
                })
                .collect();

            if contacts.len() > 0 {
                self.constraint_solver.solve_in_place(&mut self.entity_store, time_step, &contacts);

                self.narrowphase.update(&self.entity_store);
                self.broadphase.update(&self.entity_store, &self.narrowphase);
            }

            return contacts;
        }

        fn notify_body_created(&mut self, id: ID) {
            let body = self.entity_store.find_body(id)
                .expect("expected to find body that was just created, but failed!");

            self.narrowphase.notify_body_created(&self.entity_store, &**body);
            self.broadphase.notify_body_created(&self.entity_store, &**body);
        }
    }

    impl<B, N, CD, ES, I, CS> EntityStore for World<B, N, CD, ES, I, CS> where B: Broadphase<EntityStore=ES>, N: Narrowphase, CD: ContactDetection, ES: EntityStore, I: Integrator, CS: ConstraintSolver {
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

    pub trait Narrowphase {
        fn notify_body_created<ES: EntityStore>(&mut self, &ES, &Body);
        fn update<S: EntityStore>(&mut self, &S);
        // possibly could be preloaded with positional data
        fn test(&self, (ID, ID)) -> bool;
    }

    trait ContactDetection {
        fn update(&mut self);
        fn contacts_iter(&mut self, (ID, ID)) -> Box<Iterator<Item=Contact>>;
    }

    trait ConstraintSolver {
        fn solve_in_place<ES: EntityStore>(&self, store: &mut ES, time_step: Scalar, contacts: &Vec<Contact>);
    }
}
