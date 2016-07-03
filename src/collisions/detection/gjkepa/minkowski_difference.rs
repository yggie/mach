#[cfg(test)]
#[path="../../../../tests/collisions/detection/gjkepa/minkowski_difference_test.rs"]
mod tests;

use maths::Vec3D;
use collisions::{CollisionData, SupportMap};

#[derive(Clone)]
pub struct MinkowskiDifference<'a>(pub &'a CollisionData, pub &'a CollisionData);

impl<'a> MinkowskiDifference<'a> {
    pub fn reversed(self) -> MinkowskiDifference<'a> {
        MinkowskiDifference(self.1, self.0)
    }

    pub fn support_point(&self, direction: Vec3D) -> Vec3D {
        let support_points = (
            self.0.support_points_iter( direction).next().unwrap(),
            self.1.support_points_iter(-direction).next().unwrap(),
        );

        return support_points.0 - support_points.1;
    }
}
