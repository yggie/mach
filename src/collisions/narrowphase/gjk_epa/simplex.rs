use {Scalar, TOLERANCE};
use maths::{ApproxEq, Vector};
use shapes::Shape;
use entities::VolumetricBody;

#[derive(Clone, Copy, Debug)]
pub struct SupportPoint {
    pub indices: [usize; 2],
    pub position: Vector,
}

#[derive(Clone, Copy, Debug)]
pub struct Simplex {
    pub vertices: [SupportPoint; 4],
}

impl Simplex {
    fn new(bodies: [&VolumetricBody; 2]) -> Simplex {
        let relative_position = bodies[1].transform().translation() - bodies[0].transform().translation();

        debug_assert!(relative_position.length_sq() > TOLERANCE, "relative position is almost zero!");

        let support_point_0 = Simplex::generate_support_points(relative_position, bodies)[0];
        let support_point_1 = Simplex::generate_support_points(-relative_position, bodies)[0];

        let support_point_2 = {
            let a = support_point_0.position - relative_position;
            let b = support_point_1.position - relative_position;
            let norm = a.cross(b).normalize();

            [1.0, -1.0 as Scalar].iter()
                .flat_map(|&multiplier| {
                    Simplex::generate_support_points(norm * multiplier, bodies)
                }).find(|support_point| {
                    support_point.indices != support_point_0.indices &&
                        support_point.indices != support_point_1.indices
                }).expect("should have found a match here")
        };

        let support_point_3 = {
            let a = support_point_2.position - support_point_0.position;
            let b = support_point_1.position - support_point_0.position;
            let norm = a.cross(b).normalize();

            [1.0, -1.0].iter()
                .flat_map(|&multiplier| {
                    Simplex::generate_support_points(norm * multiplier, bodies)
                }).find(|support_point| {
                    support_point.indices != support_point_0.indices &&
                        support_point.indices != support_point_1.indices
                }).expect("should have found a match here")
        };

        return Simplex {
            vertices: [
                support_point_0,
                support_point_1,
                support_point_2,
                support_point_3,
            ]
        };
    }

    pub fn new_containing_origin(bodies: [&VolumetricBody; 2]) -> Option<Simplex> {
        let mut simplex = Simplex::new(bodies);
        let surface_radius = bodies[0].shape().surface_radius() + bodies[1].shape().surface_radius();

        for _ in 0..1000 {
            // find any surface facing the origin
            let next_guess = simplex.surfaces_iter()
                .map(|(normal, indices, not_in_index)| {
                    (normal, indices[0], not_in_index)
                })
                .find(|&(normal, index, _): &(Vector, usize, usize)| {
                    let vertex_to_origin = -simplex.vertices[index].position;
                    let distance_to_origin = vertex_to_origin.dot(normal);

                    return distance_to_origin > surface_radius + TOLERANCE;
                });

            let (direction, index_on_surface, index_to_replace) = match next_guess {
                Some(data) => data,
                None => return Some(simplex),
            };

            let new_support_points = Simplex::generate_support_points(direction, bodies);
            let new_support_point = new_support_points.iter().find(|point| {
                !simplex.has_matching_support_point(&point)
            });

            let vertex = match new_support_point {
                // update the simplex with the new support point if the
                // support point is in the direction of the surface
                // normal
                Some(vertex) if direction.dot(vertex.position - simplex.vertices[index_on_surface].position) > TOLERANCE => {
                    vertex
                },

                _ => return None,
            };

            simplex.vertices[index_to_replace] = vertex.clone();
        }

        unreachable!();
    }

    fn centroid(&self) -> Vector {
        return self.vertices.iter().fold(Vector::new_zero(), |total, vertex| {
            total + vertex.position
        }) / 4.0;
    }

    fn has_matching_support_point(&self, support_point: &SupportPoint) -> bool {
        return self.vertices.iter().find(|vertex| {
            ApproxEq::approx_eq(vertex.position, support_point.position)
        }).is_some();
    }

    pub fn surfaces_iter<'a>(&'a self) -> Box<Iterator<Item=(Vector, [usize; 3], usize)> + 'a> {
        let centroid = self.centroid();
        let combinations = [
            [1, 2, 3],
            [0, 2, 3],
            [0, 1, 3],
            [0, 1, 2],
        ];

        return Box::new((0..4).map(move |index| {
            let indices = combinations[index];
            let edge_0 = self.vertices[indices[1]].position - self.vertices[indices[0]].position;
            let edge_1 = self.vertices[indices[2]].position - self.vertices[indices[0]].position;
            let vertex_to_centroid = centroid - self.vertices[indices[0]].position;
            let mut surface_normal = edge_0.cross(edge_1).normalize();

            if surface_normal.dot(vertex_to_centroid) > 0.0 {
                surface_normal = -surface_normal;
            }

            return (surface_normal, indices, index);
        }));
    }

    pub fn generate_support_points(direction: Vector, bodies: [&VolumetricBody; 2]) -> Vec<SupportPoint> {
        let shapes = [bodies[0].shape(), bodies[1].shape()];
        let transforms = [bodies[0].transform(), bodies[1].transform()];

        let dirs = [
            transforms[0].apply_inverse_to_direction(direction),
            transforms[1].apply_inverse_to_direction(-direction),
        ];

        let indices = [
            shapes[0].support_indices_for(dirs[0]),
            shapes[1].support_indices_for(dirs[1]),
        ];

        let mut support_points = Vec::new();

        for &index_0 in indices[0].iter() {
            for &index_1 in indices[1].iter() {
                let point = transforms[0].apply_to_point(shapes[0].vertex(index_0)) -
                    transforms[1].apply_to_point(shapes[1].vertex(index_1));

                support_points.push(SupportPoint {
                    indices: [index_0, index_1],
                    position: point,
                });
            }
        }

        return support_points;
    }
}