#[cfg(test)]
#[path="../../../../tests/collisions/detection/gjkepa/minkowski_difference_test.rs"]
mod tests;

use maths::Vec3D;
use shapes::Shape;
use collisions::{BasicCollisionData, SupportMap};

#[derive(Clone)]
pub struct MinkowskiDifference<'a>(pub &'a BasicCollisionData, pub &'a BasicCollisionData);

impl<'a> MinkowskiDifference<'a> {
    pub fn reversed(self) -> MinkowskiDifference<'a> {
        MinkowskiDifference(self.1, self.0)
    }

    pub fn support_point(&self, direction: Vec3D) -> Vec3D {
        let shapes: (&'a Shape, &'a Shape) = (self.0.shape(), self.1.shape());
        let transforms = (self.0.transform(), self.1.transform());

        let direction_in_body_coordinates = (
            transforms.0.apply_inverse_to_direction( direction),
            transforms.1.apply_inverse_to_direction(-direction),
        );

        let support_mapped_points = (
            shapes.0.support_points_iter(direction_in_body_coordinates.0).next().unwrap(),
            shapes.1.support_points_iter(direction_in_body_coordinates.1).next().unwrap(),
        );

        return transforms.0.apply_to_point(support_mapped_points.0) -
            transforms.1.apply_to_point(support_mapped_points.1);
    }
}
