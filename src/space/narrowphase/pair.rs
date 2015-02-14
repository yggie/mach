use math::Vector;
use core::{ Body, State, UID };
use space::Contact;
use shapes::Shape;

#[cfg(test)]
#[path="../../../tests/space/narrowphase/pair_test.rs"]
mod tests;

/// A `Pair` object caches the relationship between two `Body` objects in close
/// proximity.
#[derive(Clone, Copy, Debug)]
pub struct Pair {
    /// The `UID` associated with the first `Body`.
    pub uid_0: UID,
    /// The `UID` associated with the second `Body`.
    pub uid_1: UID,
}

struct SupportPoint {
    indices: [usize; 2],
    position: Vector,
}

struct Simplex {
    vertices: [SupportPoint; 4],
}

impl Simplex {
    fn new(shapes: &[&Shape; 2], states: &[&State; 2]) -> Simplex {
        return Simplex {
            vertices: [
                Simplex::generate_support_point(Vector::new( 0.9, -0.1, -0.1), shapes, states),
                Simplex::generate_support_point(Vector::new(-0.1,  0.9, -0.1), shapes, states),
                Simplex::generate_support_point(Vector::new(-0.1, -0.1,  0.9), shapes, states),
                Simplex::generate_support_point(Vector::new( 1.0,  1.0,  1.0), shapes, states),
            ],
        };
    }

    fn centroid(&self) -> Vector {
        return self.vertices.iter().fold(Vector::new_zero(), |total, vertex| {
            total + vertex.position
        }) / 4.0;
    }

    fn has_vertex_with_indices(&self, indices: [usize; 2]) -> bool {
        return self.vertices.iter().find(|vertex| {
            vertex.indices == indices
        }).is_some();
    }

    fn surfaces_iter(&self) -> Box<Iterator<Item=(Vector, [usize; 3])>> {
        let centroid = self.centroid();
        let combinations = [
            [0, 1, 2],
            [0, 1, 3],
            [0, 2, 3],
            [1, 2, 3],
        ];

        return Box::new(range(0, 4).map(move |index| {
            let indices = combinations[index];
            let edge_0 = self.vertices[indices[1]].position - self.vertices[indices[0]].position;
            let edge_1 = self.vertices[indices[2]].position - self.vertices[indices[0]].position;
            let to_centroid = centroid - self.vertices[indices[0]].position;
            let mut surface_normal = edge_0.cross(edge_1).normalize();

            if surface_normal.dot(to_centroid) > 0.0 {
                surface_normal = -surface_normal;
            }

            // TODO put back the explicit return when the LLVM bug is fixed
            // https://github.com/rust-lang/rust/issues/22346
            (surface_normal, indices)
        }));
    }

    fn generate_support_point(direction: Vector, shapes: &[&Shape; 2], states: &[&State; 2]) -> SupportPoint {
        let dirs = [
            states[0].inverse_transform_direction(direction),
            states[1].inverse_transform_direction(-direction),
        ];

        let indices = [
            shapes[0].support_index_for(dirs[0]),
            shapes[1].support_index_for(dirs[1]),
        ];

        let point = states[0].transform_point(shapes[0].vertex(indices[0])) -
            states[1].transform_point(shapes[1].vertex(indices[1]));

        return SupportPoint {
            indices: indices,
            position: point,
        };
    }
}

impl Pair {
    /// Creates a new `Pair` object from the two input `Body` objects.
    pub fn new(body_0: &Body, body_1: &Body) -> Pair {
        Pair{ uid_0: body_0.id(), uid_1: body_1.id() }
    }

    /// Computes the `Contact` between the `Body` and returns the result if any.
    pub fn compute_contact(&self, body_0: &Body, body_1: &Body) -> Option<Contact> {
        let shapes = [body_0.shape(), body_1.shape()];
        let states = [body_0.state(), body_1.state()];

        match self.compute_enveloping_simplex(&shapes, &states) {
            Some(_) => {
                // TODO Placeholder for EPA
                return Some(Contact {
                    body_ids: [body_0.id(), body_1.id()],
                    point: Vector::new_zero(),
                    normal: Vector::new_zero(),
                });
            },

            None => return None,
        }
    }

    /// Computes a `Simplex` constructed from the Minkowski difference of the
    /// two geometries which envelopes the origin. If such a `Simplex` can be
    /// computed, the result is returned.
    fn compute_enveloping_simplex(&self, shapes: &[&Shape; 2], states: &[&State; 2]) -> Option<Simplex> {
        let mut simplex = Simplex::new(shapes, states);
        let surface_radius = shapes[0].surface_radius() + shapes[1].surface_radius();

        let mut iteration_count = 0;
        loop {
            let mut next_guess: Option<(Vector, usize)> = None;
            for (index, (normal, indices)) in simplex.surfaces_iter().enumerate() {
                let vertex_to_origin = -simplex.vertices[indices[0]].position;
                let distance_to_origin = vertex_to_origin.dot(normal);

                if distance_to_origin > surface_radius {
                    next_guess = Some((normal, index));
                    break;
                }
            }

            match next_guess {
                Some((direction, index)) => {
                    let new_vertex = Simplex::generate_support_point(direction, shapes, states);

                    if simplex.has_vertex_with_indices(new_vertex.indices) {
                        return None;
                    } else {
                        simplex.vertices[index] = new_vertex;
                    }
                },

                None => return Some(simplex),
            }

            // TODO should implement simplex history to avoid infinite loops?
            iteration_count = iteration_count + 1;
            if iteration_count > 150 {
                return None;
            }
        }
    }
}
