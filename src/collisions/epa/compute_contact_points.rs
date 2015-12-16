extern crate rand;

use self::rand::Rng;

use {NEG_INFINITY, Scalar, TOLERANCE};
use maths::Vector;
use geometries::Surface;
use collisions::gjk;
use collisions::narrowphase::Intersection;

// TODO actually return contact points
pub fn compute_contact_points(simplex_with_origin: gjk::SimplexContainingOrigin) -> Intersection {
    Polytope::new(&simplex_with_origin).compute_contact_points()
}

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

static SURFACE_INDICES_COMBINATIONS: [(usize, usize, usize); 4] = [
    (1, 2, 3),
    (0, 2, 3),
    (0, 1, 3),
    (0, 1, 2),
];

impl<'a> Polytope<'a> {
    fn new(simplex_with_origin: &'a gjk::SimplexContainingOrigin<'a>) -> Polytope<'a> {
        let diff = simplex_with_origin.diff();

        let mut support_points: Vec<gjk::SupportPoint> = Vec::new();

        let mut rng = rand::thread_rng();

        while support_points.len() < 4 {
            let guess = Vector::new(
                rng.next_f32() - 0.5 as Scalar,
                rng.next_f32() - 0.5 as Scalar,
                rng.next_f32() - 0.5 as Scalar,
            );
            let candidate_support_points = diff.support_points(&guess);

            if candidate_support_points.len() == 1 && !support_points.contains(&candidate_support_points[0]) {
                support_points.push(candidate_support_points[0]);
            }
        }

        let mid_point = support_points.iter()
            .fold(Vector::new_zero(), |total, support_point| {
                total + diff.vertex(support_point)
            }) / 4.0;

        let new_surface = |support_points: &Vec<gjk::SupportPoint>, indices: (usize, usize, usize)| -> Surface {
            let datum = diff.vertex(&support_points[indices.0]);
            let edge_0 = diff.vertex(&support_points[indices.1]) - datum;
            let edge_1 = diff.vertex(&support_points[indices.2]) - datum;
            let vertex_to_mid = mid_point - datum;
            let mut surface_normal = edge_0.cross(edge_1).normalize();

            if surface_normal.dot(vertex_to_mid) > 0.0 {
                surface_normal = -surface_normal;
            }

            return Surface {
                normal: surface_normal,
                indices: indices,
            };
        };

        let mut surfaces: Vec<Surface> = SURFACE_INDICES_COMBINATIONS.iter()
            .map(|&indices| new_surface(&support_points, indices))
            .collect();

        for _ in 0..1000 {
            let mut candidate_points: Vec<(Surface, usize, gjk::SupportPoint)> = surfaces.iter()
                .enumerate()
                .filter_map(|(surface_index, surface)| {
                    diff.support_points(&surface.normal).iter()
                        .find(|point| {
                            !support_points.contains(point) && {
                                let root = diff.vertex(&support_points[surface.indices.0]);
                                let relative_to_root = diff.vertex(point) - root;
                                let distance_from_surface = relative_to_root.dot(surface.normal);

                                distance_from_surface > TOLERANCE
                            }
                        })
                        .map(|&point| (surface.clone(), surface_index, point))
                })
                .collect();

            match candidate_points.pop() {
                Some((surface, surface_index, support_point)) => {
                    let new_point_index = support_points.len();

                    support_points.push(support_point);

                    surfaces.remove(surface_index);
                    surfaces.push(new_surface(&support_points, (surface.indices.0, surface.indices.1, new_point_index)));
                    surfaces.push(new_surface(&support_points, (surface.indices.0, surface.indices.2, new_point_index)));
                    surfaces.push(new_surface(&support_points, (surface.indices.1, surface.indices.2, new_point_index)));
                },

                None => {
                    return Polytope {
                        diff: diff,
                        support_points: support_points,
                        surfaces: surfaces,
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
            .fold((NEG_INFINITY, fake_surface), |(closest_surface_to_origin, closest_surface), surface| {
                let surface_to_origin = -surface.normal.dot(diff.vertex(&self.support_points[surface.indices.0]));

                if surface_to_origin > closest_surface_to_origin {
                    (surface_to_origin, surface.clone())
                } else {
                    (closest_surface_to_origin, closest_surface)
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
                let correction = closest_surface.normal * penetration_depth / -2.0;
                diff.bodies.0.vertex(index) + correction
            },

            (_other, IntersectionType::Vertex(index)) => {
                let correction = closest_surface.normal * penetration_depth / 2.0;
                diff.bodies.0.vertex(index) + correction
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
