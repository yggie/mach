#[cfg(test)]
#[path="../../../tests/detection/gjkepa/polytope_test.rs"]
mod polytope_test;

use NEG_INFINITY;
use maths::Vect;
use utils::compute_surfaces_for_convex_hull;
use geometry::{Line, Plane, PlaneLocation};
use detection::ContactSet;

use super::simplex::Simplex;
use super::minkowski_difference::{MinkowskiDifference, IndexPair};

enum FeatureType {
    Vertex(usize),
    Edge(usize, usize),
    Face,
}

pub struct Polytope<'a> {
    diff: MinkowskiDifference<'a>,
    pub surfaces: Vec<(Plane, (usize, usize, usize))>,
    pub support_points: Vec<(Vect, IndexPair)>,
}

impl<'a> Polytope<'a> {
    pub fn new(simplex: Simplex<'a>) -> Polytope<'a> {
        let surfaces = simplex.surfaces_iter().collect();

        let mut polytope = Polytope {
            diff: simplex.diff,
            surfaces: surfaces,
            support_points: simplex.support_points.clone().to_vec(),
        };

        for _ in 0..1000 {
            let candidate_point = polytope.surfaces.iter()
                .filter_map(|&(ref plane, _vertex_indices)| {
                    let mut new_index_pairs = polytope.diff.support_index_pairs(plane.normal());

                    let any_points_already_tested = new_index_pairs.iter()
                        .any(|&index_pair| {
                            polytope.support_points.iter()
                                .any(|&(_vertex, existing_pair)| {
                                    index_pair == existing_pair
                                })
                        });

                    if any_points_already_tested || {
                        let point = polytope.diff.vertex(&new_index_pairs[0]);

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
                    polytope.support_points.push((polytope.diff.vertex(&index_pair), index_pair));

                    let vertex_positions: Vec<Vect> = polytope.support_points.iter()
                        .map(|&(ref vertex, _index_pair)| vertex.clone())
                        .collect();

                    polytope.surfaces = compute_surfaces_for_convex_hull(&vertex_positions).iter()
                        .map(|surface| {
                            let (_vertex, index_pair) = polytope.support_points[surface.nodes[0]];
                            let point_on_surface = polytope.diff.vertex(&index_pair);

                            return (Plane::from_point(&point_on_surface, &surface.normal), (surface.nodes[0], surface.nodes[1], surface.nodes[2]));
                        })
                        .collect();
                },

                None => {
                    return polytope;
                },
            }
        }

        panic!("Took more than 1000 iterations to create an expanded polytope from the simplex");
    }

    pub fn compute_contact_points(&self) -> ContactSet {
        let dummy_plane = Plane::from_point(&Vect::new(1.0, 0.0, 0.0), &Vect::new(0.0, 0.0, 0.0));

        let (penetration_depth, closest_plane, closest_vertex_indices) = self.surfaces.iter().skip(1)
            .fold((NEG_INFINITY, dummy_plane, (0, 0, 0)), |(origin_to_closest_plane_offset, closest_plane, closest_vertex_indices), &(ref plane, vertex_indices)| {
                let offset = plane.offset_for_origin();

                if offset > origin_to_closest_plane_offset {
                    (offset, plane.clone(), vertex_indices)
                } else {
                    (origin_to_closest_plane_offset, closest_plane, closest_vertex_indices)
                }
            });

        let support_points = [
            &self.support_points[closest_vertex_indices.0],
            &self.support_points[closest_vertex_indices.1],
            &self.support_points[closest_vertex_indices.2],
        ];

        let contact_types = (
            self.infer_contact_type((support_points[0].1).0, (support_points[1].1).0, (support_points[2].1).0),
            self.infer_contact_type((support_points[0].1).1, (support_points[1].1).1, (support_points[2].1).1),
        );

        let contact_point = match contact_types {
            (FeatureType::Vertex(index), _other) => {
                let correction = closest_plane.normal() * penetration_depth / 2.0;
                self.diff.0.vertex(index) + correction
            },

            (_other, FeatureType::Vertex(index)) => {
                let correction = closest_plane.normal() * penetration_depth / 2.0;
                self.diff.1.vertex(index) - correction
            },

            (FeatureType::Edge(index_00, index_01), FeatureType::Edge(index_10, index_11)) => {
                let vertex_00 = self.diff.0.vertex(index_00);
                let vertex_01 = self.diff.0.vertex(index_01);
                let vertex_10 = self.diff.1.vertex(index_10);
                let vertex_11 = self.diff.1.vertex(index_11);

                let line_0 = Line::from_points(vertex_00, vertex_01);
                let line_1 = Line::from_points(vertex_10, vertex_11);

                line_0.closest_point_to_line(&line_1)
            },

            (FeatureType::Face, FeatureType::Edge(_index_10, _index_11)) => {
                panic!("UNHANDLED CONTACT TYPE [FACE|EDGE]");
            },

            (FeatureType::Edge(_index_00, _index_01), FeatureType::Face) => {
                panic!("UNHANDLED CONTACT TYPE [EDGE|FACE]");
            },

            (FeatureType::Face, FeatureType::Face) => {
                panic!("UNHANDLED CONTACT TYPE [FACE|FACE]");
            },
        };

        let contact_normal = -closest_plane.normal();
        return ContactSet::new(
            Plane::from_point(&(contact_point - contact_normal * penetration_depth), &contact_normal),
            vec!(contact_point),
        );
    }

    fn infer_contact_type(&self, index_0: usize, index_1: usize, index_2: usize) -> FeatureType {
        match (index_0, index_1, index_2) {
            (a, b, c) if a == b && b == c => FeatureType::Vertex(a),
            (a, b, c) if b == c => FeatureType::Edge(a, b),
            (a, b, c) if a == c => FeatureType::Edge(a, b),
            (a, b, c) if a == b => FeatureType::Edge(a, c),
            _otherwise => FeatureType::Face,
        }
    }
}
