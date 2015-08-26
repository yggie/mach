mod simplex;
mod polytope;

use self::simplex::Simplex;
use self::polytope::Polytope;

use maths::Vector;
use shapes::ShapeEntity;
use collisions::NarrowPhase;
use collisions::narrowphase::Intersection;

enum ContactType {
    Vertex(usize),
    Edge(usize),
    Face,
}

/// The object which encapsulates the default implementation of the GJK-EPA
/// algorithm for the engine.
pub struct GjkEpaImplementation;

impl GjkEpaImplementation {
    /// Returns the intersection information, if any, between two shape
    /// entities.
    pub fn find_intersection(&self, entity_0: &ShapeEntity, entity_1: &ShapeEntity) -> Option<Intersection> {
        Simplex::new_containing_origin([entity_0, entity_1]).map(|simplex| {
            let mut polytope = Polytope::new(&simplex);
            polytope.expand_fully([entity_0, entity_1]);

            let intersection = GjkEpaImplementation::contact_for_polytope(&polytope, [entity_0, entity_1]);
            return intersection;
        })
    }

    fn contact_for_polytope(polytope: &Polytope, entities: [&ShapeEntity; 2]) -> Intersection {
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

            match GjkEpaImplementation::infer_contact_type(mapped_indices) {
                ContactType::Vertex(vertex_index) => {
                    println!("CONTACT VERTEX");
                    if i == 1 {
                        let correction = contact_normal * depth / 2.0;
                        contact_center = entities[1].vertex(vertex_index) + correction;
                    } else {
                        let correction = contact_normal * depth / -2.0;
                        contact_center = entities[0].vertex(vertex_index) + correction;
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

        return Intersection::new(contact_center, contact_normal);
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

impl NarrowPhase for GjkEpaImplementation {
    #[inline(always)]
    fn find_intersection(&self, entity_0: &ShapeEntity, entity_1: &ShapeEntity) -> Option<Intersection> {
        (self as &GjkEpaImplementation).find_intersection(entity_0, entity_1)
    }
}
