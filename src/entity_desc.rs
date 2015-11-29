use std::rc::Rc;

use Scalar;
use maths::{State, Vector};
use entities::Material;
use shapes::{Cuboid, Shape, Sphere, TriangleMesh};

#[derive(Clone)]
pub enum ShapeDesc {
    Cuboid(Scalar, Scalar, Scalar),
    Sphere(Scalar),
    TriangleMesh(Rc<Vec<Vector>>, Vec<(usize, usize, usize)>),
}

impl ShapeDesc {
    pub fn build(&self) -> Box<Shape> {
        match self {
            &ShapeDesc::Cuboid(width, height, depth) => {
                Box::new(Cuboid::new(width, height, depth))
            },

            &ShapeDesc::Sphere(radius) => {
                Box::new(Sphere::new(radius))
            },

            &ShapeDesc::TriangleMesh(ref vertices, ref indices) => {
                Box::new(TriangleMesh::new(vertices.clone(), indices.clone()))
            },
        }
    }
}

#[derive(Clone)]
pub struct EntityDesc {
    // TODO application specified IDs?
    // pub id: Option<ID>,
    pub material: Material,
    pub shape_desc: ShapeDesc,
    pub state: State,
}

impl EntityDesc {
    pub fn as_sphere(&self, radius: Scalar) -> EntityDesc {
        EntityDesc {
            shape_desc: ShapeDesc::Sphere(radius),
            .. self.clone()
        }
    }

    pub fn as_cube(&self, size: Scalar) -> EntityDesc {
        self.as_cuboid(size, size, size)
    }

    pub fn as_cuboid(&self, width: Scalar, height: Scalar, depth: Scalar) -> EntityDesc {
        EntityDesc {
            shape_desc: ShapeDesc::Cuboid(width, height, depth),
            .. self.clone()
        }
    }

    pub fn as_triangle_mesh(&self, vertices: Rc<Vec<Vector>>, indices: Vec<(usize, usize, usize)>) -> EntityDesc {
        EntityDesc {
            shape_desc: ShapeDesc::TriangleMesh(vertices, indices),
            .. self.clone()
        }
    }

    pub fn with_density(&self, density: Scalar) -> EntityDesc {
        EntityDesc {
            material: self.material.with_density(density),
            .. self.clone()
        }
    }

    pub fn with_mass(&self, mass: Scalar) -> EntityDesc {
        EntityDesc {
            material: self.material.with_mass(mass),
            .. self.clone()
        }
    }

    pub fn with_translation(&self, x: Scalar, y: Scalar, z: Scalar) -> EntityDesc {
        EntityDesc {
            state: self.state.with_pos(x, y, z),
            .. self.clone()
        }
    }

    pub fn with_restitution_coefficient(&self, restitution_coefficient: Scalar) -> EntityDesc {
        EntityDesc {
            material: self.material.with_coefficient_of_restitution(restitution_coefficient),
            .. self.clone()
        }
    }
}

impl Default for EntityDesc {
    fn default() -> Self {
        EntityDesc {
            material: Material::default(),
            shape_desc: ShapeDesc::Cuboid(1.0, 1.0, 1.0),
            state: State::default(),
        }
    }
}
