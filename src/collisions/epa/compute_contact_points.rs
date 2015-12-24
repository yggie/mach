use {NEG_INFINITY, TOLERANCE};
use maths::Vector;
use utils::compute_surfaces_for_convex_hull;
use geometries::Surface;
use collisions::gjk;
use collisions::narrowphase::Intersection;

// TODO actually return contact points
pub fn compute_contact_points(simplex_with_origin: gjk::SimplexContainingOrigin) -> Intersection {
    Polytope::new(&simplex_with_origin).compute_contact_points()
}

#[cfg(test)]
#[path="../../../tests/private/collisions/epa/polytope_test.rs"]
mod polytope_tests;

struct Polytope<'a> {
    diff: &'a gjk::MinkowskiDifference<'a>,
    support_points: Vec<gjk::SupportPoint>,
    surfaces: Vec<Surface>,
}

enum IntersectionType {
    Vertex(usize),
    Edge(usize),
    Face,
}

impl<'a> Polytope<'a> {
    fn new(simplex_with_origin: &'a gjk::SimplexContainingOrigin<'a>) -> Polytope<'a> {
        let diff = simplex_with_origin.diff();

        let mut support_points = simplex_with_origin.simplex().support_points().clone().to_vec();
        let mut surfaces: Vec<Surface> = simplex_with_origin.simplex().surfaces_iter(diff).collect();

        for _ in 0..1000 {
            let candidate_point = surfaces.iter()
                .filter_map(|surface| {
                    let mut new_support_points = diff.support_points(&surface.normal);

                    if new_support_points.iter().any(|point| support_points.contains(point)) || {
                        let candidate_point = &new_support_points[0];

                        let point_on_surface = diff.vertex(&support_points[surface.indices.0]);
                        let point_to_surface = point_on_surface - diff.vertex(candidate_point);
                        let surface_offset = -point_to_surface.dot(surface.normal);

                        surface_offset < TOLERANCE
                    } {
                        return None;
                    }

                    let new_support_point = new_support_points.pop()
                        .expect("Expected there to be only one support point at this step");

                    {
                        let candidate_point = &new_support_point;

                        let point_on_surface = diff.vertex(&support_points[surface.indices.0]);
                        let point_to_surface = point_on_surface - diff.vertex(candidate_point);
                        let surface_offset = -point_to_surface.dot(surface.normal);

                        if surface_offset < TOLERANCE {
                            panic!(format!("OOO SHALALALA ({}, {})", support_points.len(), surfaces.len()));
                        }
                    }

                    return Some(new_support_point);
                })
                .take(1)
                .next();

            match candidate_point {
                Some(support_point) => {
                    support_points.push(support_point);

                    let vertex_positions: Vec<Vector> = support_points.iter()
                        .map(|point| diff.vertex(&point))
                        .collect();

                    surfaces = compute_surfaces_for_convex_hull(&vertex_positions).iter()
                        .map(|surface| {
                            Surface {
                                normal: surface.normal,
                                indices: (surface.nodes[0], surface.nodes[1], surface.nodes[2]),
                            }
                        })
                        .collect();
                },

                None => {
                    return Polytope {
                        diff: diff,
                        surfaces: surfaces,
                        support_points: support_points,
                    };
                },
            }
        }

        panic!("Took more than 1000 iterations to create an expanded polytope from the simplex");
    }

    fn compute_contact_points(&self) -> Intersection {
        debug_assert!(self.surfaces.len() >= 4, "the polytope formed was degenerate");

        let fake_surface = Surface {
            normal: Vector::new_zero(),
            indices: (0, 0, 0),
        };

        let diff = self.diff;
        let (penetration_depth, closest_surface) = self.surfaces.iter()
            .fold((NEG_INFINITY, fake_surface), |(origin_to_closest_surface_offset, closest_surface), surface| {
                let point_on_surface = diff.vertex(&self.support_points[surface.indices.0]);
                let origin_surface_offset = -point_on_surface.dot(surface.normal);

                if origin_surface_offset > origin_to_closest_surface_offset {
                    (origin_surface_offset, surface.clone())
                } else {
                    (origin_to_closest_surface_offset, closest_surface)
                }
            });

        let support_points = [
            &self.support_points[closest_surface.indices.0],
            &self.support_points[closest_surface.indices.1],
            &self.support_points[closest_surface.indices.2],
        ];

        let contact_types = (
            self.infer_contact_type(support_points[0].0, support_points[1].0, support_points[2].0),
            self.infer_contact_type(support_points[0].1, support_points[1].1, support_points[2].1),
        );

        let contact_point = match contact_types {
            (IntersectionType::Vertex(index), _other) => {
                let correction = closest_surface.normal * penetration_depth / 2.0;
                diff.bodies.0.vertex(index) + correction
            },

            (_other, IntersectionType::Vertex(index)) => {
                let correction = closest_surface.normal * penetration_depth / -2.0;
                diff.bodies.1.vertex(index) + correction
            },

            (IntersectionType::Edge(_), IntersectionType::Edge(_)) => {
                println!("UNHANDLED CONTACT TYPE [EDGE|EDGE]");
                Vector::new_zero()
            },

            (IntersectionType::Face, IntersectionType::Edge(_)) => {
                println!("UNHANDLED CONTACT TYPE [FACE|EDGE]");
                Vector::new_zero()
            },

            (IntersectionType::Edge(_), IntersectionType::Face) => {
                println!("UNHANDLED CONTACT TYPE [EDGE|FACE]");
                Vector::new_zero()
            },

            (IntersectionType::Face, IntersectionType::Face) => {
                println!("UNHANDLED CONTACT TYPE [FACE|FACE]");
                Vector::new_zero()
            },
        };

        return Intersection::new(contact_point, closest_surface.normal, penetration_depth);
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
