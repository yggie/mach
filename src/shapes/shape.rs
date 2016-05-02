use std::fmt;

use {Scalar, TOLERANCE};
use maths::{Matrix, Vec3D};
use shapes::ShapeRef;

/// Defines the traits for all geometric property descriptions.
pub trait Shape: fmt::Debug {
    fn downcast<'a>(&'a self) -> ShapeRef<'a>;

    /// Computes the volume for the shape.
    fn volume(&self) -> Scalar;

    /// Returns the normalized inertia tensor for the shape.
    fn inertia(&self) -> Matrix;

    /// Obtains the vertex with the index specified.
    fn vertex(&self, usize) -> Vec3D;

    /// Returns the number of vertices in the `Shape`.
    fn vertices_len(&self) -> usize;

    /// Returns an iterator over all the vertices in the shape.
    fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vec3D> + 'a>;

    /// Returns the index of the vertex furthest in the direction specified,
    /// primarily used by collision detection routines.
    fn support_indices_for(&self, Vec3D) -> Vec<usize>;

    /// Returns the _surface radius_ of the Shape. The surface radius is the
    /// tolerance used to determine if a collision has occurred, it is useful to
    /// avoid problems with singularities such as edge-edge collisions. By
    /// default it returns the Engine’s tolerance, _i.e._
    /// `mach::maths::TOLERANCE`.
    #[inline]
    fn surface_radius(&self) -> Scalar {
        TOLERANCE
    }

    fn box_clone(&self) -> Box<Shape>;
}

impl Clone for Box<Shape> {
    fn clone(&self) -> Box<Shape> {
        self.box_clone()
    }
}
