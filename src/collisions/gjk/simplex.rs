use {Scalar, TOLERANCE};
use maths::Vector;
use collisions::gjk::{MinkowskiDifference, SupportPoint};

static SURFACE_COMBINATIONS: [((usize, usize, usize), usize); 4] = [
    ((1, 2, 3), 0),
    ((0, 2, 3), 1),
    ((0, 1, 3), 2),
    ((0, 1, 2), 3),
];

pub struct SimplexContainingOrigin<'a>(&'a Simplex, &'a MinkowskiDifference<'a>);

#[derive(Debug)]
pub struct Simplex {
    support_points: [SupportPoint; 4],
}

impl Simplex {
    pub fn new(diff: &MinkowskiDifference) -> Simplex {
        let relative_position = diff.center();

        let support_point_0 = diff.support_points( &relative_position)[0].clone();
        let support_point_1 = diff.support_points(&-relative_position)[0].clone();

        let support_point_2 = {
            let guesses = [
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
            ];

            guesses.iter()
                .flat_map(|guess| {
                    diff.support_points(guess)
                }).find(|support_point| {
                    support_point != &support_point_0 &&
                        support_point != &support_point_1
                }).expect("should have found a match here")
        };

        let support_point_3 = {
            let datum = diff.vertex(&support_point_0);
            let a = diff.vertex(&support_point_2) - datum;
            let b = diff.vertex(&support_point_1) - datum;
            let norm = Vector::cross(&a, b).normalize();

            [1.0, -1.0 as Scalar].iter()
                .flat_map(|&multiplier| {
                    diff.support_points(&(norm * multiplier))
                }).find(|support_point| {
                    support_point != &support_point_0 &&
                        support_point != &support_point_1 &&
                        support_point != &support_point_2
                }).expect("should have found a match here")
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

        for _ in 0..1000 {
            let next_guess = self.surfaces_iter(diff)
                .find(|&(normal, _plane, point)| {
                    let vertex_to_origin = diff.vertex(&self.support_points[point]);
                    let distance_to_origin = -vertex_to_origin.dot(normal);

                    return distance_to_origin > surface_radius + TOLERANCE;
                });

            let (normal, plane, not_on_plane) = match next_guess {
                Some(data) => data,
                None => return Some(SimplexContainingOrigin(self, &diff)),
            };

            let new_support_points = diff.support_points(&normal);
            let new_support_point_option = new_support_points.into_iter()
                .find(|candidate_point| {
                    !history.iter().any(|pt| pt == candidate_point)
                });

            let point_on_plane = diff.vertex(&self.support_points[plane.0]);
            let new_support_point = match new_support_point_option {
                // update the simplex with the new support point if the
                // support point is in the direction of the surface
                // normal
                Some(new_support_point) if normal.dot(diff.vertex(&new_support_point) - point_on_plane) > TOLERANCE => {
                    new_support_point
                },

                _ => return None,
            };

            self.support_points[not_on_plane] = new_support_point.clone();
            history.push(new_support_point);
        }

        panic!("Took over 1000 iterations while seeking the origin");
    }

    fn centroid(&self, diff: &MinkowskiDifference) -> Vector {
        self.support_points.iter()
            .fold(Vector::new_zero(), |total, support_point| {
                total + diff.vertex(support_point)
            }) / 4.0
    }

    fn surfaces_iter<'a>(&'a self, diff: &'a MinkowskiDifference) -> Box<Iterator<Item=(Vector, (usize, usize, usize), usize)> + 'a> {
        let centroid = self.centroid(diff);

        let iterator = SURFACE_COMBINATIONS.iter()
            .map(move |&(plane, point)| {
                let datum = diff.vertex(&self.support_points[plane.0]);
                let edge_0 = diff.vertex(&self.support_points[plane.1]) - datum;
                let edge_1 = diff.vertex(&self.support_points[plane.2]) - datum;
                let vertex_to_centroid = centroid - datum;
                let mut surface_normal = edge_0.cross(edge_1).normalize();

                if surface_normal.dot(vertex_to_centroid) > 0.0 {
                    surface_normal = -surface_normal;
                }

                return (surface_normal, plane, point);
            });

        return Box::new(iterator);
    }

    pub fn support_point(&self, index: usize) -> &SupportPoint {
        &self.support_points[index]
    }
}
