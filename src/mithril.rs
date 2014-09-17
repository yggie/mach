#![crate_name = "mithril"]
#![crate_type = "lib"]

//! Contains the implementation of the Mithril Physics engine, an open-source
//! physics engine built on the Rust programming language.

/// The math module contains all the logic associated with primitive mathematical
/// operations.
pub mod math {
    pub use self::vector::Vector;
    pub use self::matrix::Matrix;
    pub use self::transform::Transform;

    mod vector;
    mod matrix;
    mod transform;
}

/// The shapes module defines the shared traits for all geometric models.
pub mod shapes {
    pub use self::shape::Shape;

    pub use self::primitives::Sphere;
    pub use self::primitives::Cube;

    mod shape;

    mod primitives {
        pub use self::sphere::Sphere;
        pub use self::cube::Cube;

        mod sphere;
        mod cube;
    }
}
