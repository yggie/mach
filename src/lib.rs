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

    use {ID, Scalar};
    use maths::Integrator;
    use entities::EntityStore;

    struct World<B: Broadphase, N: Narrowphase, C: ContactDetector, S: EntityStore, I: Integrator> {
        broadphase: B,
        narrowphase: N,
        contact_detector: C,
        entity_store: S,
        integrator: I,
    }

    struct Contact(u32);

    impl<B: Broadphase, N: Narrowphase, C: ContactDetector, S: EntityStore, I: Integrator> World<B, N, C, S, I> {
        fn update(&mut self, time_step: Scalar) {
            // update entity positions
            for mut integratable in self.entity_store.integratable_iter_mut() {
                self.integrator.integrate_in_place(&mut integratable, time_step);
            }

            self.narrowphase.update(&self.entity_store);
            self.broadphase.update(&self.entity_store, &self.narrowphase);

            let entity_pairs: Vec<(ID, ID)> = self.broadphase.entity_pairs_iter()
                // TODO something like: .map(|pair| self.entity_store.preload_transform(pair))
                .filter(|&pair| self.narrowphase.test(pair))
                .collect();

            let contacts: Vec<Contact> = entity_pairs.iter()
                .fold(Box::new(std::iter::empty()) as Box<Iterator<Item=Contact>>, |iter: Box<Iterator<Item=Contact>>, pair: &(ID, ID)| -> Box<Iterator<Item=Contact>> {
                    Box::new(iter.chain(self.contact_detector.contacts_iter(*pair)))
                })
                .collect();
        }
    }

    trait Broadphase {
        fn update<S: EntityStore, N: Narrowphase>(&mut self, &S, &N);
        fn entity_pairs_iter(&self) -> Box<Iterator<Item=(ID, ID)>>;
    }

    trait Narrowphase {
        fn update<S: EntityStore>(&mut self, &S);
        // possibly could be preloaded with positional data
        fn test(&self, (ID, ID)) -> bool;
    }

    trait ContactDetector {
        fn update(&mut self);
        fn contacts_iter(&mut self, (ID, ID)) -> Box<Iterator<Item=Contact>>;
    }
}
