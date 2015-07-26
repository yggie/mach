//! The `shapes` module defines the shared traits for all geometric models.

use std::fmt;

use math::{ Matrix, TOLERANCE, Vector };

pub use self::shape_entity::ShapeEntity;
pub use self::primitives::Cube;

/// Defines the traits for all geometric property descriptions.
pub trait Shape: fmt::Display + 'static {
    /// Computes the volume for the shape.
    fn volume(&self) -> f32;

    /// Returns the normalized inertia tensor for the shape.
    fn inertia_tensor(&self) -> Matrix;

    /// Obtains the vertex with the index specified.
    fn vertex(&self, usize) -> Vector;

    /// Returns the number of vertices in the `Shape`.
    fn vertices_len(&self) -> usize;

    /// Returns an iterator over all the vertices in the shape.
    fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=&Vector> + 'a>;

    /// Returns the index of the vertex furthest in the direction specified,
    /// primarily used by collision detection routines.
    fn support_indices_for(&self, Vector) -> Vec<usize>;

    /// Returns the _surface radius_ of the Shape. The surface radius is the
    /// tolerance used to determine if a collision has occurred, it is useful to
    /// avoid problems with singularities such as edge-edge collisions. By
    /// default it returns the Engine’s tolerance, _i.e._
    /// `mithril::math::TOLERANCE`.
    #[inline]
    fn surface_radius(&self) -> f32 {
        return TOLERANCE;
    }
}

mod shape_entity;

mod primitives {
    // TODO re-enable Sphere once GJK-EPA has stabilized
    // pub use self::sphere::Sphere;
    pub use self::cube::Cube;

    // mod sphere;
    mod cube;
}
