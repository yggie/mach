//! The `shapes` module defines the shared traits for all geometric models.

#![unstable]

use math::{ TOLERANCE, Vector };

pub use self::primitives::Cube;

/// Defines the traits for all geometric property descriptions.
pub trait Shape: Eq + Send {
    /// Computes the volume for the shape.
    fn volume(&self) -> f32;
    /// Obtains the vertex with the index specified.
    fn vertex(&self, usize) -> Vector;
    /// Returns the number of vertices in the `Shape`.
    fn vertices_len(&self) -> usize;
    /// Returns an iterator over all the vertices in the shape.
    fn vertices_iter(&self) -> Box<Iterator<Item=&Vector>>;
    /// Returns the index of the vertex furthest in the direction specified,
    /// primarily used by collision detection routines.
    fn support_index_for(&self, Vector) -> usize;
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

mod primitives {
    // TODO re-enable Sphere once GJK-EPA has stabilized
    // pub use self::sphere::Sphere;
    pub use self::cube::Cube;

    // mod sphere;
    mod cube;
}
