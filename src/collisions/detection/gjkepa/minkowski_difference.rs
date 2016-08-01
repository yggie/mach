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
}

impl<'a> SupportMap for MinkowskiDifference<'a> {
    fn support_points_iter<'b>(&'b self, direction: Vec3D) -> Box<Iterator<Item=Vec3D> + 'b> {
        let other_direction = -direction;
        let iterator = self.0.support_points_iter(direction)
            .flat_map(move |point| {
                self.1.support_points_iter(other_direction)
                    .map(move |other_point| {
                        point - other_point
                    })
            });

        return Box::new(iterator);
    }
}
