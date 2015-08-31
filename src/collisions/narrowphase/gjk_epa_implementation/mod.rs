mod simplex;
mod polytope;

use self::simplex::Simplex;
use self::polytope::Polytope;

use core::VolumetricBody;
use maths::Vector;
use collisions::NarrowPhase;
use collisions::narrowphase::Intersection;

enum IntersectionType {
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
    pub fn find_intersection(&self, entity_0: &VolumetricBody, entity_1: &VolumetricBody) -> Option<Intersection> {
        Simplex::new_containing_origin([entity_0, entity_1]).map(|simplex| {
            let mut polytope = Polytope::new(&simplex);
            polytope.expand_fully([entity_0, entity_1]);

            let intersection = GjkEpaImplementation::contact_for_polytope(&polytope, [entity_0, entity_1]);
            return intersection;
        })
    }

    fn contact_for_polytope(polytope: &Polytope, entities: [&VolumetricBody; 2]) -> Intersection {
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

        let contact_type_0 = GjkEpaImplementation::infer_contact_type(0, polytope, indices);
        let contact_type_1 = GjkEpaImplementation::infer_contact_type(1, polytope, indices);

        let contact_center = match (contact_type_0, contact_type_1) {
            (IntersectionType::Vertex(vertex_index), _) => {
                let correction = contact_normal * depth / -2.0;
                entities[0].vertex(vertex_index) + correction
            },

            (_, IntersectionType::Vertex(vertex_index)) => {
                let correction = contact_normal * depth / 2.0;
                entities[1].vertex(vertex_index) + correction
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

        // for i in (0..2) {
        //     let mapped_indices = [
        //         polytope.vertices[indices[0]].indices[i],
        //         polytope.vertices[indices[1]].indices[i],
        //         polytope.vertices[indices[2]].indices[i],
        //     ];
        //
        //     match GjkEpaImplementation::infer_contact_type(mapped_indices) {
        //         IntersectionType::Edge(_) => {
        //             // TODO implement this
        //             println!("!!!CONTACT EDGE!!!");
        //             // unimplemented!();
        //         },
        //
        //         IntersectionType::Face => {
        //             // TODO implement this
        //             println!("!!!CONTACT FACE!!!");
        //             // unimplemented!();
        //         },
        //     }
        // }

        return Intersection::new(contact_center, contact_normal);
    }

    fn infer_contact_type(entity_number: usize, polytope: &Polytope, indices: [usize; 3]) -> IntersectionType {
        let mapped_indices = [
            polytope.vertices[indices[0]].indices[entity_number],
            polytope.vertices[indices[1]].indices[entity_number],
            polytope.vertices[indices[2]].indices[entity_number],
        ];

        return GjkEpaImplementation::infer_contact_type_with_indices(mapped_indices);
    }

    fn infer_contact_type_with_indices(indices: [usize; 3]) -> IntersectionType {
        if indices[0] == indices[1] && indices[1] == indices[2] {
            return IntersectionType::Vertex(indices[0]);
        } else if indices[0] == indices[1] || indices[0] == indices[2] || indices[1] == indices[2] {
            return IntersectionType::Edge(0);
        } else {
            return IntersectionType::Face;
        }
    }
}

impl NarrowPhase for GjkEpaImplementation {
    #[inline(always)]
    fn find_intersection(&self, entity_0: &VolumetricBody, entity_1: &VolumetricBody) -> Option<Intersection> {
        (self as &GjkEpaImplementation).find_intersection(entity_0, entity_1)
    }
}
