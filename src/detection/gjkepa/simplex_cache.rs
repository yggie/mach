#[cfg(test)]
#[path="../../../tests/detection/gjkepa/simplex_cache_test.rs"]
mod simplex_cache_test;

extern crate rand;

use self::rand::Rng;

use {Scalar, TOLERANCE};
use maths::Vect;
use shapes::PlaneLocation;

use super::simplex::Simplex;
use super::minkowski_difference::{MinkowskiDifference, IndexPair};

static NOT_ON_SURFACE: [usize; 4] = [0, 1, 2, 3];

#[derive(Clone, Debug)]
pub struct SimplexCache {
    index_pairs: [IndexPair; 4],
}

impl SimplexCache {
    pub fn new(diff: &MinkowskiDifference) -> SimplexCache {
        let mut index_pairs = Vec::new();
        let mut rng = rand::thread_rng();

        while index_pairs.len() != 3 {
            let guess = Vect::new(
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
            );

            let candidate_support_points = diff.support_index_pairs(&guess);

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
            let norm = Vect::cross(&a, b).normalize();

            [1.0, -1.0 as Scalar].iter()
                .filter_map(|&multiplier| {
                    diff.support_index_pairs(&(norm * multiplier)).iter()
                        .take(1)
                        .find(|support_point| {
                            Vect::dot(&norm, diff.vertex(support_point) - datum).abs() > TOLERANCE
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

    pub fn update_to_contain_origin<'a>(&mut self, diff: MinkowskiDifference<'a>) -> Option<Simplex<'a>> {
        let surface_radius = diff.0.shape().surface_radius() +
            diff.1.shape().surface_radius();

        let mut history = self.index_pairs.clone().to_vec();
        let mut simplex = Simplex::new(self, diff);

        for _iteration in 0..1000 {
            let next_guess = simplex.surfaces_iter()
                .zip(NOT_ON_SURFACE.iter())
                .find(|&((ref plane, _vertex_indices), _not_on_surface)| {
                    plane.offset_for_origin() > surface_radius + TOLERANCE
                });

            let ((plane, _vertex_indices), &not_on_surface) = match next_guess {
                Some(data) => data,
                None => return Some(simplex),
            };

            let new_support_points = simplex.diff.support_index_pairs(plane.normal());
            let new_support_point_option = new_support_points.into_iter()
                .find(|candidate_point| {
                    !history.iter().any(|pt| pt == candidate_point) &&
                        plane.location_of(&simplex.diff.vertex(&candidate_point)) == PlaneLocation::Above
                });

            let new_support_point = match new_support_point_option {
                Some(new_support_point) => new_support_point,
                _ => return None,
            };

            let vertex = simplex.diff.vertex(&new_support_point);
            simplex.support_points[not_on_surface] = (vertex, new_support_point);
            self.index_pairs[not_on_surface] = new_support_point.clone();
            history.push(new_support_point);
        }

        panic!("Took over 1000 iterations while seeking the origin");
    }

    #[inline]
    pub fn index_pairs(&self) -> &[IndexPair; 4] {
        &self.index_pairs
    }
}
