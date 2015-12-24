extern crate rand;

#[cfg(test)]
#[path="../../../tests/private/collisions/gjk/simplex_test.rs"]
mod tests;

use self::rand::Rng;

use {Scalar, TOLERANCE};
use maths::Vector;
use geometries::Surface;
use collisions::gjk::{MinkowskiDifference, SupportPoint};

static SURFACE_INDICES_COMBINATIONS: [(usize, usize, usize); 4] = [
    (1, 2, 3),
    (0, 2, 3),
    (0, 1, 3),
    (0, 1, 2),
];

static NOT_ON_SURFACE: [usize; 4] = [0, 1, 2, 3];

pub struct SimplexContainingOrigin<'a>(&'a Simplex, &'a MinkowskiDifference<'a>);

impl<'a> SimplexContainingOrigin<'a> {
    pub fn simplex(&self) -> &Simplex {
        self.0
    }

    pub fn diff(&self) -> &MinkowskiDifference {
        self.1
    }
}

#[derive(Debug)]
pub struct Simplex {
    support_points: [SupportPoint; 4],
}

impl Simplex {
    pub fn new(diff: &MinkowskiDifference) -> Simplex {
        let mut support_points = Vec::new();
        let mut rng = rand::thread_rng();

        while support_points.len() != 3 {
            let guess = Vector::new(
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
            );

            let candidate_support_points = diff.support_points(&guess);

            if candidate_support_points.len() != 1 {
                continue;
            }

            if let Some(support_point) = candidate_support_points.first() {
                if !support_points.contains(support_point) {
                    support_points.push(support_point.clone());
                }
            }
        }

        let support_point_0 = support_points[0];
        let support_point_1 = support_points[1];
        let support_point_2 = support_points[2];
        let support_point_3 = {
            let datum = diff.vertex(&support_point_0);
            let a = diff.vertex(&support_point_2) - datum;
            let b = diff.vertex(&support_point_1) - datum;
            let norm = Vector::cross(&a, b).normalize();

            [1.0, -1.0 as Scalar].iter()
                .filter_map(|&multiplier| {
                    diff.support_points(&(norm * multiplier)).iter()
                        .take(1)
                        .find(|support_point| {
                            Vector::dot(&norm, diff.vertex(support_point) - datum).abs() > TOLERANCE
                        })
                        .map(|support_point| support_point.clone())
                })
                .next()
                .expect("Could not generate a simplex")
        };

        return Simplex {
            support_points: [
                support_point_0,
                support_point_1,
                support_point_2,
                support_point_3,
            ],
        };
    }

    pub fn reshape_to_contain_origin<'a>(&'a mut self, diff: &'a MinkowskiDifference) -> Option<SimplexContainingOrigin> {
        let surface_radius = diff.bodies.0.shape().surface_radius() +
            diff.bodies.1.shape().surface_radius();

        let mut history = self.support_points.clone().to_vec();

        for _loop_index in 0..1000 {
            let next_guess = self.surfaces_iter(diff)
                .zip(NOT_ON_SURFACE.iter())
                .find(|&(ref surface, _not_on_surface)| {
                    let point_on_surface = diff.vertex(&self.support_points[surface.indices.0]);
                    let distance_to_origin = -point_on_surface.dot(surface.normal);

                    return distance_to_origin > surface_radius + TOLERANCE;
                });

            let (surface, &not_on_surface) = match next_guess {
                Some(data) => data,
                None => return Some(SimplexContainingOrigin(self, &diff)),
            };

            let new_support_points = diff.support_points(&surface.normal);
            let point_on_plane = diff.vertex(&self.support_points[surface.indices.0]);
            let new_support_point_option = new_support_points.into_iter()
                .find(|candidate_point| {
                    !history.iter().any(|pt| pt == candidate_point) &&
                        surface.normal.dot(diff.vertex(&candidate_point) - point_on_plane) > TOLERANCE
                });

            let new_support_point = match new_support_point_option {
                Some(new_support_point) => new_support_point,
                _ => return None,
            };

            self.support_points[not_on_surface] = new_support_point.clone();
            history.push(new_support_point);
        }

        panic!("Took over 1000 iterations while seeking the origin");
    }

    pub fn centroid(&self, diff: &MinkowskiDifference) -> Vector {
        self.support_points.iter()
            .fold(Vector::new_zero(), |total, support_point| {
                total + diff.vertex(support_point)
            }) / 4.0
    }

    pub fn surfaces_iter<'a>(&'a self, diff: &'a MinkowskiDifference) -> Box<Iterator<Item=Surface> + 'a> {
        let centroid = self.centroid(diff);

        let iterator = SURFACE_INDICES_COMBINATIONS.iter()
            .map(move |&indices| {
                let datum = diff.vertex(&self.support_points[indices.0]);
                let edge_0 = diff.vertex(&self.support_points[indices.1]) - datum;
                let edge_1 = diff.vertex(&self.support_points[indices.2]) - datum;
                let vertex_to_centroid = centroid - datum;
                let mut surface_normal = edge_0.cross(edge_1).normalize();

                if surface_normal.dot(vertex_to_centroid) > 0.0 {
                    surface_normal = -surface_normal;
                }

                return Surface {
                    normal: surface_normal,
                    indices: indices,
                };
            });

        return Box::new(iterator);
    }

    pub fn support_points(&self) -> &[SupportPoint; 4] {
        &self.support_points
    }
}
