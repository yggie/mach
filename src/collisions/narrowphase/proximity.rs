use rand::random;

use math::{ Vector, TOLERANCE };
use core::{ Body, Handle, State };
use utils::compute_surfaces_for_convex_hull;
use shapes::Shape;

/// A `Proximity` object caches the relationship between two bodies in close
/// proximity.
#[derive(Clone, Debug)]
pub struct Proximity<H: Handle> {
    /// The unique handles to the pair of `Body` instances. These values are
    /// never used internally, simply as an aid for the external implementation
    /// to identify the related `Body` instances.
    pub handles: [H; 2],
}

enum ContactType {
    Vertex(usize),
    Edge(usize),
    Face,
}

#[derive(Clone, Copy, Debug)]
struct SupportPoint {
    indices: [usize; 2],
    position: Vector,
}

#[derive(Clone, Copy, Debug)]
struct Simplex {
    vertices: [SupportPoint; 4],
}

impl Simplex {
    fn new(shapes: [&Shape; 2], states: [&State; 2]) -> Simplex {
        let mut support_points: Vec<SupportPoint> = Vec::new();
        while support_points.len() < 4 {
            let vector = Vector::new(
                random::<f32>() - 0.5,
                random::<f32>() - 0.5,
                random::<f32>() - 0.5,
            );
            let candidate_support_point = Simplex::generate_support_points(vector.normalize(), shapes, states)[0];

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

    fn new_containing_origin(shapes: [&Shape; 2], states: [&State; 2]) -> Option<Simplex> {
        let mut simplex = Simplex::new(shapes, states);
        let surface_radius = shapes[0].surface_radius() + shapes[1].surface_radius();

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
                    let new_support_points = Simplex::generate_support_points(direction, shapes, states);

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

    fn surfaces_iter<'a>(&'a self) -> Box<Iterator<Item=(Vector, [usize; 3], usize)> + 'a> {
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

    fn generate_support_points(direction: Vector, shapes: [&Shape; 2], states: [&State; 2]) -> Vec<SupportPoint> {
        let dirs = [
            states[0].inverse_transform_direction(direction),
            states[1].inverse_transform_direction(-direction),
        ];

        let indices = [
            shapes[0].support_indices_for(dirs[0]),
            shapes[1].support_indices_for(dirs[1]),
        ];

        let mut support_points = Vec::new();

        for &index_0 in indices[0].iter() {
            for &index_1 in indices[1].iter() {
                let point = states[0].transform_point(shapes[0].vertex(index_0)) -
                    states[1].transform_point(shapes[1].vertex(index_1));

                support_points.push(SupportPoint {
                    indices: [index_0, index_1],
                    position: point,
                });
            }
        }

        return support_points;
    }
}

struct Polytope {
    vertices: Vec<SupportPoint>,
    surfaces: Vec<(Vector, [usize; 3])>,
}

impl Polytope {
    fn new(simplex: &Simplex) -> Polytope {
        Polytope {
            vertices: vec!(simplex.vertices[0], simplex.vertices[1], simplex.vertices[2], simplex.vertices[3]),
            surfaces: simplex.surfaces_iter().map(|(a, b, _)| (a, b)).collect(),
        }
    }

    fn has_matching_support_point(&self, support_point: &SupportPoint) -> bool {
        return self.vertices.iter().find(|vertex| {
            vertex.position == support_point.position
        }).is_some();
    }

    fn expand_fully<H: Handle>(&mut self, body_0: &Body<H>, body_1: &Body<H>) {
        let shapes = [body_0.shape(), body_1.shape()];
        let states = [body_0.state(), body_1.state()];

        for _ in (0..1000) {
            if !self.expand(shapes, states) { return }
        }

        unreachable!();
    }

    fn expand(&mut self, shapes: [&Shape; 2], states: [&State; 2]) -> bool {
        let new_point: Vec<SupportPoint> = self.surfaces.iter()
            .filter_map(|&(surface_normal, surface_indices)| {
                let new_support_points = Simplex::generate_support_points(surface_normal, shapes, states);

                let candidate_support_point = new_support_points.iter()
                    .find(|point| {
                        let root = self.vertices[surface_indices[0]].position;
                        let relative_to_root = point.position - root;
                        let distance_from_surface = relative_to_root.dot(surface_normal);

                        return !self.has_matching_support_point(&point) &&
                            distance_from_surface.abs() > TOLERANCE;
                    });

                match candidate_support_point {
                    Some(&support_point) => return Some(support_point),

                    None => return None,
                }
            })
            .take(1)
            .collect();

        match new_point.get(0) {
            Some(&support_point) => {
                self.vertices.push(support_point);
                let vertex_positions: Vec<Vector> = self.vertices.iter()
                    .map(|vertex| vertex.position)
                    .collect();
                let surfaces = compute_surfaces_for_convex_hull(&vertex_positions);

                self.surfaces = surfaces.iter()
                    .map(|surface| (surface.normal, surface.nodes))
                    .collect();
                return true;
            },

            None => return false,
        }
    }
}

impl<H: Handle> Proximity<H> {
    /// Creates a new `Proximity` object with the specified handles.
    pub fn new(handle_0: H, handle_1: H) -> Proximity<H> {
        Proximity { handles: [ handle_0, handle_1 ] }
    }

    /// Computes the `Contact` between the `Body` and returns the result if any.
    pub fn find_intersection(&self, body_0: &Body<H>, body_1: &Body<H>) -> Option<(Vector, Vector)> {
        let shapes = [body_0.shape(), body_1.shape()];
        let states = [body_0.state(), body_1.state()];

        return Simplex::new_containing_origin(shapes, states).map(|simplex| {
                let mut polytope = Polytope::new(&simplex);
                polytope.expand_fully(body_0, body_1);

                let (contact_normal, contact_center) = Proximity::<H>::contact_for_polytope(&polytope, body_0, body_1);
                println!("CONTACT AT {}", contact_center);
                return (contact_center, contact_normal);
        });
    }

    fn contact_for_polytope(polytope: &Polytope, body_0: &Body<H>, body_1: &Body<H>) -> (Vector, Vector) {
        let mut closest_surface: Option<(f32, Vector, [usize; 3])> = None;
        for &(surface_normal, indices) in polytope.surfaces.iter() {
            let current_depth = surface_normal.dot(polytope.vertices[indices[0]].position);

            match closest_surface {
                Some((depth, _, _)) if current_depth < depth => {
                    closest_surface = Some((current_depth, surface_normal, indices));
                },

                None => {
                    closest_surface = Some((current_depth, surface_normal, indices));
                },

                _ => { /* do nothing */ },
            }
        }

        let (depth, contact_normal, indices) = closest_surface.unwrap();
        let mut contact_center = Vector::new_zero();

        for i in (0..2) {
            let mapped_indices = [
                polytope.vertices[indices[0]].indices[i],
                polytope.vertices[indices[1]].indices[i],
                polytope.vertices[indices[2]].indices[i],
            ];

            match Proximity::<H>::infer_contact_type(mapped_indices) {
                ContactType::Vertex(vertex_index) => {
                    if i == 1 {
                        let correction = contact_normal * depth / 2.0;
                        contact_center = body_1.vertex(vertex_index) + correction;
                    } else {
                        let correction = contact_normal * depth / -2.0;
                        contact_center = body_0.vertex(vertex_index) + correction;
                    }
                    break;
                },

                ContactType::Edge(_) => {
                    // TODO implement this
                    println!("CONTACT EDGE");
                    // unimplemented!();
                },

                ContactType::Face => {
                    // TODO implement this
                    println!("CONTACT FACE");
                    // unimplemented!();
                },
            }
        }

        return (contact_normal, contact_center);
    }

    fn infer_contact_type(indices: [usize; 3]) -> ContactType {
        if indices[0] == indices[1] && indices[1] == indices[2] {
            return ContactType::Vertex(indices[0]);
        } else if indices[0] == indices[1] || indices[0] == indices[2] || indices[1] == indices[2] {
            return ContactType::Edge(0);
        } else {
            return ContactType::Face;
        }
    }
}
