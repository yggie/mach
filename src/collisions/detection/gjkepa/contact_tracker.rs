#[cfg(test)]
#[path="../../../../tests/collisions/detection/gjkepa/contact_tracker_test.rs"]
mod tests;

use maths::Vec3D;
use utils::{UniqueVec3DGenerator, UnitVec3DGenerator};
use collisions::CollisionData;
use collisions::geometry::{Direction, SupportMap};
use collisions::detection::gjkepa::{GJKSimplex, MinkowskiDifference};

#[derive(Clone, Debug)]
pub struct ContactTracker {
    simplex: GJKSimplex,
}

impl ContactTracker {
    pub fn new(data_0: &CollisionData, data_1: &CollisionData) -> ContactTracker {
        let diff = MinkowskiDifference(data_0, data_1);
        let mut generator = UnitVec3DGenerator::new();

        let mut counter = 0;
        while counter < 1000 {
            let unique_vertices_iter = UniqueVec3DGenerator::new(|| {
                let guess = Direction::from(generator.gen_next());

                diff.support_points_iter(guess).next().unwrap()
            });

            let unique_vertices: Vec<Vec3D> = unique_vertices_iter.take(4).collect();

            if let Ok(simplex) = GJKSimplex::from_vertices(unique_vertices[0], unique_vertices[1], unique_vertices[2], unique_vertices[3]) {
                return ContactTracker {
                    simplex: simplex,
                };
            }

            counter += 1;
        }

        panic!("took more than 1000 iterations to construct a GJKSimplex");
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
