#[cfg(test)]
#[path="../../../tests/detection/gjkepa/epa_test.rs"]
mod tests;

use maths::Vect;
use utils::compute_surfaces_for_convex_hull;
use geometry::{Plane, PlaneLocation};
use algorithms::IterativeAlgorithm;

use super::simplex::Simplex;
use super::polytope::Polytope;

pub struct EPA<'a> {
    polytope: Polytope<'a>,
    has_converged: bool,
}

impl<'a> EPA<'a> {
    pub fn new(simplex: Simplex<'a>) -> EPA<'a> {
        EPA {
            polytope: Polytope::from_simplex(simplex),
            has_converged: false,
        }
    }
}

impl<'a> IterativeAlgorithm for EPA<'a> {
    type Result = Polytope<'a>;

    fn result(self) -> Self::Result {
        self.polytope
    }

    fn has_converged(&self) -> bool {
        self.has_converged
    }

    fn next_iteration(&mut self) {
        if self.has_converged {
            return;
        }

        let candidate_point = self.polytope.surfaces.iter()
            .filter_map(|&(ref plane, _vertex_indices)| {
                let mut new_index_pairs = self.polytope.diff.support_index_pairs(plane.normal());

                let any_points_already_tested = new_index_pairs.iter()
                    .any(|&index_pair| {
                        self.polytope.support_points.iter()
                            .any(|&(_vertex, existing_pair)| {
                                index_pair == existing_pair
                            })
                    });

                if any_points_already_tested || {
                    let point = self.polytope.diff.vertex(&new_index_pairs[0]);

                    plane.location_of(&point) != PlaneLocation::Above
                } {
                    return None;
                }

                let new_index_pair = new_index_pairs.pop()
                    .expect("Expected there to be only one support point at this step");

                return Some(new_index_pair);
            })
            .take(1)
            .next();

        match candidate_point {
            Some(index_pair) => {
                self.polytope.support_points.push((self.polytope.diff.vertex(&index_pair), index_pair));

                let vertex_positions: Vec<Vect> = self.polytope.support_points.iter()
                    .map(|&(ref vertex, _index_pair)| vertex.clone())
                    .collect();

                self.polytope.surfaces = compute_surfaces_for_convex_hull(&vertex_positions).iter()
                    .map(|surface| {
                        let (_vertex, index_pair) = self.polytope.support_points[surface.nodes[0]];
                        let point_on_surface = self.polytope.diff.vertex(&index_pair);

                        return (Plane::from_point(&point_on_surface, &surface.normal), (surface.nodes[0], surface.nodes[1], surface.nodes[2]));
                    })
                    .collect();
            },

            None => self.has_converged = true,
        }
    }
}
