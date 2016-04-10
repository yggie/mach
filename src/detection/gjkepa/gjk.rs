#[cfg(test)]
#[path="../../../tests/detection/gjkepa/gjk_test.rs"]
mod tests;

use {Scalar, TOLERANCE};
use algorithms::IterativeAlgorithm;

use super::simplex::Simplex;
use super::simplex_cache::SimplexCache;
use super::minkowski_difference::{MinkowskiDifference, IndexPair};

static NOT_ON_SURFACE: [usize; 4] = [0, 1, 2, 3];

pub struct GJK<'a> {
    simplex: Simplex<'a>,
    result_was_successful: Option<bool>,
    simplex_cache: &'a mut SimplexCache,
    surface_radius: Scalar,
    history: Vec<IndexPair>,
}

impl<'a> GJK<'a> {
    pub fn new(simplex_cache: &'a mut SimplexCache, diff: MinkowskiDifference<'a>) -> GJK<'a> {
        let surface_radius = diff.0.shape().surface_radius() +
            diff.1.shape().surface_radius();

        let history = simplex_cache.index_pairs.clone().to_vec();
        let simplex = Simplex::new(simplex_cache, diff);

        GJK {
            history: history,
            simplex: simplex,
            simplex_cache: simplex_cache,
            surface_radius: surface_radius,
            result_was_successful: None,
        }
    }
}

impl<'a> IterativeAlgorithm for GJK<'a> {
    type Result = Option<Simplex<'a>>;

    fn result(self) -> Option<Simplex<'a>> {
        self.result_was_successful.and_then(move |success| {
            if success {
                Some(self.simplex)
            } else {
                None
            }
        })
    }

    fn has_converged(&self) -> bool {
        self.result_was_successful.is_some()
    }

    fn next_iteration(&mut self) {
        let next_guess = self.simplex.surfaces_iter()
            .zip(NOT_ON_SURFACE.iter())
            .find(|&((ref plane, _vertex_indices), _not_on_surface)| {
                plane.offset_for_origin() > self.surface_radius + TOLERANCE
            });

        let ((plane, _vertex_indices), &not_on_surface) = match next_guess {
            Some(data) => data,

            None => {
                self.result_was_successful = Some(true);
                return;
            },
        };

        let new_support_points = self.simplex.diff.support_index_pairs(plane.normal());
        let new_support_point_option = new_support_points.into_iter()
            .find(|candidate_point| {
                !self.history.iter().any(|pt| pt == candidate_point) &&
                    plane.location_of(&self.simplex.diff.vertex(&candidate_point)).is_above_plane()
            });

        let new_support_point = match new_support_point_option {
            Some(new_support_point) => new_support_point,

            _otherwise => {
                self.result_was_successful = Some(false);
                return;
            },
        };

        let vertex = self.simplex.diff.vertex(&new_support_point);
        self.simplex.support_points[not_on_surface] = (vertex, new_support_point);
        self.simplex_cache.index_pairs[not_on_surface] = new_support_point.clone();
        self.history.push(new_support_point);
    }
}
