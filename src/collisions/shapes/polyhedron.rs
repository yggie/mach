use maths::{CrossProduct, DotProduct, Vec3D};
use utils::compute_surfaces_for_convex_hull;
use collisions::shapes::{ConvexHull3D, Face};

pub struct Polyhedron {
    convex_hull: ConvexHull3D,
    triangulated_faces: Vec<[usize; 3]>,
}

impl Polyhedron {
    pub fn convex_hull(vertices: &[Vec3D]) -> Polyhedron {
        // TODO bit unsafe, but we trust the vertices are part of the convex
        // hull
        let vertices_clone = Vec::from(vertices);
        let surfaces = compute_surfaces_for_convex_hull(&vertices_clone);

        let triangulated_faces = surfaces.iter()
            .map(|surface| {
                let vertex_0 = vertices[surface.nodes[0]];
                let vertex_1 = vertices[surface.nodes[1]];
                let vertex_2 = vertices[surface.nodes[2]];

                let counter_clockwise_normal = (vertex_2 - vertex_1).cross(vertex_0 - vertex_1).normalize();

                if counter_clockwise_normal.dot(surface.normal).is_sign_positive() {
                    return surface.nodes;
                } else {
                    return [surface.nodes[2], surface.nodes[1], surface.nodes[0]];
                }
            })
            .collect::<Vec<[usize; 3]>>();

        Polyhedron {
            convex_hull: ConvexHull3D::new(vertices_clone),
            triangulated_faces: triangulated_faces,
        }
    }

    #[inline(always)]
    pub fn vertices(&self) -> &Vec<Vec3D> {
        self.convex_hull.vertices()
    }

    pub fn faces_iter<'a>(&'a self) -> Box<Iterator<Item=Face<'a>> + 'a> {
        let iterator = self.triangulated_faces.iter()
            .map(move |indices| {
                Face::counter_clockwise_from(
                    self.convex_hull.vertices(),
                    indices.clone(),
                )
            });

        return Box::new(iterator);
    }

    pub fn add_vertex(&mut self, vertex: Vec3D) -> bool {
        let mut vertices = Vec::<Vec3D>::from(self.convex_hull.clone());
        vertices.push(vertex);

        let polyhedron = Polyhedron::convex_hull(&vertices);

        let new_vertex_was_accepted = polyhedron.vertices().iter()
            .find(|new_vertex| **new_vertex == vertex)
            .is_some();

        if new_vertex_was_accepted {
            self.convex_hull = polyhedron.convex_hull;
            self.triangulated_faces = polyhedron.triangulated_faces;

            return true;
        } else {
            return false;
        }
    }
}
