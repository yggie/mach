use std::rc::Rc;

use Scalar;
use maths::{State, Vect};
use entities::Material;
use shapes::{Cuboid, Shape, Sphere, TriangleMesh};

#[derive(Clone)]
pub enum ShapeDesc {
    Cuboid(Scalar, Scalar, Scalar),
    Sphere(Scalar),
    TriangleMesh(Rc<Vec<Vect>>, Vec<(usize, usize, usize)>),
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
    pub fn as_sphere(self, radius: Scalar) -> EntityDesc {
        EntityDesc {
            shape_desc: ShapeDesc::Sphere(radius),
            .. self
        }
    }

    pub fn as_cube(self, size: Scalar) -> EntityDesc {
        self.as_cuboid(size, size, size)
    }

    pub fn as_cuboid(self, width: Scalar, height: Scalar, depth: Scalar) -> EntityDesc {
        EntityDesc {
            shape_desc: ShapeDesc::Cuboid(width, height, depth),
            .. self
        }
    }

    pub fn as_triangle_mesh(self, vertices: Rc<Vec<Vect>>, indices: Vec<(usize, usize, usize)>) -> EntityDesc {
        EntityDesc {
            shape_desc: ShapeDesc::TriangleMesh(vertices, indices),
            .. self
        }
    }

    pub fn as_stationary(self) -> EntityDesc {
        EntityDesc {
            state: State::new_stationary(),
            .. self
        }
    }

    pub fn with_density(self, density: Scalar) -> EntityDesc {
        EntityDesc {
            material: self.material.with_density(density),
            .. self
        }
    }

    pub fn with_mass(self, mass: Scalar) -> EntityDesc {
        EntityDesc {
            material: self.material.with_mass(mass),
            .. self
        }
    }

    pub fn with_pos(self, x: Scalar, y: Scalar, z: Scalar) -> EntityDesc {
        EntityDesc {
            state: self.state.with_pos(x, y, z),
            .. self
        }
    }

    pub fn with_vel(self, vx: Scalar, vy: Scalar, vz: Scalar) -> EntityDesc {
        EntityDesc {
            state: self.state.with_vel(vx, vy, vz),
            .. self
        }
    }

    pub fn with_axis_angle(self, axis: Vect, angle: Scalar) -> EntityDesc {
        EntityDesc {
            state: self.state.with_axis_angle(axis, angle),
            .. self
        }
    }

    pub fn with_ang_vel(self, vx: Scalar, vy: Scalar, vz: Scalar) -> EntityDesc {
        EntityDesc {
            state: self.state.with_ang_vel(vx, vy, vz),
            .. self
        }
    }

    pub fn with_restitution_coefficient(self, restitution_coefficient: Scalar) -> EntityDesc {
        EntityDesc {
            material: self.material.with_coefficient_of_restitution(restitution_coefficient),
            .. self
        }
    }

    pub fn with_friction_coefficient(self, friction_coefficient: Scalar) -> EntityDesc {
        EntityDesc {
            material: self.material.with_friction_coefficient(friction_coefficient),
            .. self
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
