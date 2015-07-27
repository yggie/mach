use rand::random;

use maths::{ Vector, TOLERANCE };
use shapes::{ Shape, ShapeEntity };

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
    fn new(entities: [&ShapeEntity; 2]) -> Simplex {
        let mut support_points: Vec<SupportPoint> = Vec::new();
        while support_points.len() < 4 {
            let vector = Vector::new(
                random::<f32>() - 0.5,
                random::<f32>() - 0.5,
                random::<f32>() - 0.5,
            );
            let candidate_support_point = Simplex::generate_support_points(vector.normalize(), entities)[0];

            if support_points.iter().find(|p| { p.indices == candidate_support_point.indices }).is_none() {
                support_points.push(candidate_support_point);
            }
        }

        return Simplex {
            vertices: [
                support_points[0],
                support_points[1],
                support_points[2],
                support_points[3],
            ],
        };
    }

    pub fn new_containing_origin(entities: [&ShapeEntity; 2]) -> Option<Simplex> {
        let mut simplex = Simplex::new(entities);
        let surface_radius = entities[0].shape().surface_radius() + entities[1].shape().surface_radius();

        for _ in (0..1000) {
            let mut next_guess: Option<(Vector, usize, usize)> = None;
            // find any surface facing the origin
            for (normal, indices, not_in_index) in simplex.surfaces_iter() {
                let vertex_to_origin = -simplex.vertices[indices[0]].position;
                let distance_to_origin = vertex_to_origin.dot(normal);

                if distance_to_origin > surface_radius {
                    next_guess = Some((normal, indices[0], not_in_index));
                    break;
                }
            }

            match next_guess {
                Some((direction, index_on_surface, index_to_replace)) => {
                    let new_support_points = Simplex::generate_support_points(direction, entities);

                    let new_support_point = new_support_points.iter().find(|point| {
                        !simplex.has_matching_support_point(&point)
                    });

                    match new_support_point {
                        // update the simplex with the new support point if the
                        // support point is in the direction of the surface
                        // normal
                        Some(vertex) if direction.dot(vertex.position - simplex.vertices[index_on_surface].position) > TOLERANCE => {
                            simplex.vertices[index_to_replace] = *vertex;
                        },

                        _ => return None,
                    }
                },

                None => return Some(simplex),
            }
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
            vertex.position == support_point.position
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

    pub fn generate_support_points(direction: Vector, entities: [&ShapeEntity; 2]) -> Vec<SupportPoint> {
        let shapes = [entities[0].shape(), entities[1].shape()];
        let transforms = [entities[0].transform(), entities[1].transform()];

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
