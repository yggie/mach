use std::fmt;

use {Scalar, TOLERANCE};
use maths::{Matrix, Vec3D};
use collisions::SupportMap;
use collisions::geometry::{Direction, Geometry};
use collisions::geometry::convex_shapes::ShapeRef;

/// Defines the traits for all geometric property descriptions.
pub trait ConvexShape: Geometry + fmt::Debug {
    fn downcast<'a>(&'a self) -> ShapeRef<'a>;

    /// Computes the volume for the shape.
    fn volume(&self) -> Scalar;

    /// Returns the normalized inertia tensor for the shape.
    fn inertia(&self) -> Matrix;

    /// Obtains the vertex with the index specified.
    fn vertex(&self, usize) -> Vec3D;

    /// Returns the number of vertices in the `ConvexShape`.
    fn vertices_len(&self) -> usize;

    /// Returns an iterator over all the vertices in the shape.
    fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vec3D> + 'a>;

    /// Returns the index of the vertex furthest in the direction specified,
    /// primarily used by collision detection routines.
    fn support_indices_for(&self, Direction) -> Vec<usize>;

    /// Returns the _surface radius_ of the ConvexShape. The surface radius is
    /// the tolerance used to determine if a collision has occurred, it is
    /// useful to avoid problems with singularities such as edge-edge
    /// collisions. By default it returns the Engine’s tolerance, _i.e._
    /// `mach::maths::TOLERANCE`.
    #[inline]
    fn surface_radius(&self) -> Scalar {
        TOLERANCE
    }

    // TODO move to some kind of utility module
    fn box_clone(&self) -> Box<ConvexShape>;
}

impl Clone for Box<ConvexShape> {
    fn clone(&self) -> Box<ConvexShape> {
        self.box_clone()
    }
}

impl SupportMap for Box<ConvexShape> {
    fn support_points_iter<'b>(&'b self, direction: Direction) -> Box<Iterator<Item=Vec3D> + 'b> {
        let vec = self.support_indices_for(direction).iter()
            .map(|&index| self.vertex(index))
            .collect::<Vec<Vec3D>>();

        return Box::new(vec.into_iter());
    }
}
