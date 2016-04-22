extern crate rand;

use self::rand::Rng;

use {Scalar, TOLERANCE};
use maths::{CrossProduct, DotProduct, Vec3D};

use super::minkowski_difference::{MinkowskiDifference, IndexPair};

#[derive(Clone, Debug)]
pub struct SimplexCache {
    pub index_pairs: [IndexPair; 4],
}

impl SimplexCache {
    pub fn new(diff: &MinkowskiDifference) -> SimplexCache {
        let mut index_pairs = Vec::new();
        let mut rng = rand::thread_rng();

        while index_pairs.len() != 3 {
            let guess = Vec3D::new(
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
            );

            let candidate_support_points = diff.support_index_pairs(guess);

            if candidate_support_points.len() != 1 {
                continue;
            }

            if let Some(support_point) = candidate_support_points.first() {
                if !index_pairs.contains(support_point) {
                    index_pairs.push(support_point.clone());
                }
            }
        }

        let support_point_0 = index_pairs[0];
        let support_point_1 = index_pairs[1];
        let support_point_2 = index_pairs[2];
        let support_point_3 = {
            let datum = diff.vertex(&support_point_0);
            let a = diff.vertex(&support_point_2) - datum;
            let b = diff.vertex(&support_point_1) - datum;
            let norm = a.cross(b).normalize();

            [1.0, -1.0 as Scalar].iter()
                .filter_map(|&multiplier| {
                    diff.support_index_pairs(norm * multiplier).iter()
                        .find(|support_point| {
                            norm.dot(diff.vertex(support_point) - datum).abs() > TOLERANCE
                        })
                        .map(|support_point| support_point.clone())
                })
                .next()
                .expect("Could not generate a simplex")
        };

        return SimplexCache {
            index_pairs: [
                support_point_0,
                support_point_1,
                support_point_2,
                support_point_3,
            ],
        };
    }

    #[inline]
    pub fn index_pairs(&self) -> &[IndexPair; 4] {
        &self.index_pairs
    }
}
