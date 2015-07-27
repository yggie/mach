use super::simplex::{ SupportPoint, Simplex };

use shapes::ShapeEntity;
use maths::{ Vector, TOLERANCE };
use utils::compute_surfaces_for_convex_hull;

pub struct Polytope {
    pub vertices: Vec<SupportPoint>,
    pub surfaces: Vec<(Vector, [usize; 3])>,
}

impl Polytope {
    pub fn new(simplex: &Simplex) -> Polytope {
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

    pub fn expand_fully(&mut self, entities: [&ShapeEntity; 2]) {
        for _ in (0..1000) {
            if !self.expand(entities) { return }
        }

        unreachable!();
    }

    fn expand(&mut self, entities: [&ShapeEntity; 2]) -> bool {
        let new_point: Vec<SupportPoint> = self.surfaces.iter()
            .filter_map(|&(surface_normal, surface_indices)| {
                let new_support_points = Simplex::generate_support_points(surface_normal, entities);

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

