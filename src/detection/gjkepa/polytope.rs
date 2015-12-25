use NEG_INFINITY;
use maths::Vect;
use utils::compute_surfaces_for_convex_hull;
use detection::Intersection;
use geometries::{PlaneLocation, Plane};

use super::simplex::Simplex;
use super::minkowski_difference::{MinkowskiDifference, IndexPair};

enum IntersectionType {
    Vertex(usize),
    Edge(usize),
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

    /// TODO return more than 1 point
    pub fn compute_contact_points(&self) -> Intersection {
        let fake_plane = Plane::from_point(&Vect::new(1.0, 0.0, 0.0), &Vect::new(0.0, 0.0, 0.0));

        let (penetration_depth, closest_plane, closest_vertex_indices) = self.surfaces.iter()
            .fold((NEG_INFINITY, fake_plane, (0, 0, 0)), |(origin_to_closest_plane_offset, closest_plane, closest_vertex_indices), &(ref plane, vertex_indices)| {
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
            (IntersectionType::Vertex(index), _other) => {
                let correction = closest_plane.normal() * penetration_depth / 2.0;
                self.diff.bodies.0.vertex(index) + correction
            },

            (_other, IntersectionType::Vertex(index)) => {
                let correction = closest_plane.normal() * penetration_depth / -2.0;
                self.diff.bodies.1.vertex(index) + correction
            },

            (IntersectionType::Edge(_), IntersectionType::Edge(_)) => {
                println!("UNHANDLED CONTACT TYPE [EDGE|EDGE]");
                Vect::new_zero()
            },

            (IntersectionType::Face, IntersectionType::Edge(_)) => {
                println!("UNHANDLED CONTACT TYPE [FACE|EDGE]");
                Vect::new_zero()
            },

            (IntersectionType::Edge(_), IntersectionType::Face) => {
                println!("UNHANDLED CONTACT TYPE [EDGE|FACE]");
                Vect::new_zero()
            },

            (IntersectionType::Face, IntersectionType::Face) => {
                println!("UNHANDLED CONTACT TYPE [FACE|FACE]");
                Vect::new_zero()
            },
        };

        return Intersection::new(contact_point, closest_plane.normal().clone(), penetration_depth);
    }

    fn infer_contact_type(&self, index_0: usize, index_1: usize, index_2: usize) -> IntersectionType {
        if index_0 == index_1 && index_1 == index_2 {
            IntersectionType::Vertex(index_0)
        } else if index_0 == index_1 || index_0 == index_2 {
            IntersectionType::Edge(index_0)
        } else if index_1 == index_2 {
            IntersectionType::Edge(index_1)
        } else {
            IntersectionType::Face
        }
    }
}
