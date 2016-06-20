#[cfg(test)]
#[path="../../../../tests/collisions/detection/gjkepa/contact_tracker_test.rs"]
mod tests;

use maths::{ApproxEq, Vec3D};
use utils::UnitVec3DGenerator;
use collisions::BasicCollisionData;
use collisions::detection::gjkepa::{GJKSimplex, MinkowskiDifference};

#[derive(Clone, Debug)]
pub struct ContactTracker {
    simplex: GJKSimplex,
}

impl ContactTracker {
    pub fn new(data_0: &BasicCollisionData, data_1: &BasicCollisionData) -> ContactTracker {
        let diff = MinkowskiDifference(data_0, data_1);
        let mut vertices: Vec<Vec3D> = Vec::with_capacity(4);
        let mut generator = UnitVec3DGenerator::new();

        while vertices.len() != 4 {
            let guess = Vec3D::from(generator.next());

            let candidate_point = diff.support_point(guess);

            if !vertices.iter().any(|point| point.approx_eq(candidate_point)) {
                vertices.push(candidate_point);
            }
        }

        return ContactTracker {
            simplex: GJKSimplex::new(
                vertices[0],
                vertices[1],
                vertices[2],
                vertices[3],
            ),
        };
    }

    #[inline(always)]
    pub fn simplex(&self) -> &GJKSimplex {
        &self.simplex
    }

    #[inline(always)]
    pub fn simplex_mut(&mut self) -> &mut GJKSimplex {
        &mut self.simplex
    }
}
