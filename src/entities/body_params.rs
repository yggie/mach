use std::rc::Rc;

use Scalar;
use maths::{Motion, Transform, Quat, Vect};
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
pub struct BodyParams {
    pub motion: Motion,
    pub material: Material,
    pub transform: Transform,
    pub shape_desc: ShapeDesc,
}

impl BodyParams {
    pub fn cube(size: Scalar) -> BodyParams {
        BodyParams::default().as_cube(size)
    }

    pub fn cuboid(x: Scalar, y: Scalar, z: Scalar) -> BodyParams {
        BodyParams::default().as_cuboid(x, y, z)
    }

    pub fn shape(shape_desc: ShapeDesc) -> BodyParams {
        BodyParams::default().as_shape(shape_desc)
    }

    pub fn as_shape(self, shape_desc: ShapeDesc) -> BodyParams {
        BodyParams {
            shape_desc: shape_desc,
            .. self
        }
    }

    pub fn as_sphere(self, radius: Scalar) -> BodyParams {
        BodyParams {
            shape_desc: ShapeDesc::Sphere(radius),
            .. self
        }
    }

    pub fn as_cube(self, size: Scalar) -> BodyParams {
        self.as_cuboid(size, size, size)
    }

    pub fn as_cuboid(self, x: Scalar, y: Scalar, z: Scalar) -> BodyParams {
        BodyParams {
            shape_desc: ShapeDesc::Cuboid(x, y, z),
            .. self
        }
    }

    pub fn as_triangle_mesh(self, vertices: Rc<Vec<Vect>>, indices: Vec<(usize, usize, usize)>) -> BodyParams {
        BodyParams {
            shape_desc: ShapeDesc::TriangleMesh(vertices, indices),
            .. self
        }
    }

    pub fn as_stationary(self) -> BodyParams {
        BodyParams {
            motion: Motion::stationary(),
            .. self
        }
    }

    pub fn with_density(self, density: Scalar) -> BodyParams {
        BodyParams {
            material: self.material.with_density(density),
            .. self
        }
    }

    pub fn with_mass(self, mass: Scalar) -> BodyParams {
        BodyParams {
            material: self.material.with_mass(mass),
            .. self
        }
    }

    pub fn with_translation(self, x: Scalar, y: Scalar, z: Scalar) -> BodyParams {
        BodyParams {
            transform: Transform {
                translation: Vect::new(x, y, z),
                .. self.transform
            },
            .. self
        }
    }

    pub fn with_rotation(self, rotation: Quat) -> BodyParams {
        BodyParams {
            transform: Transform {
                rotation: rotation,
                .. self.transform
            },
            .. self
        }
    }

    pub fn with_axis_angle(self, axis: Vect, angle: Scalar) -> BodyParams {
        BodyParams {
            transform: Transform {
                rotation: Quat::from_axis_angle(axis, angle),
                .. self.transform
            },
            .. self
        }
    }

    pub fn with_material(self, material: Material) -> BodyParams {
        BodyParams {
            material: material,
            .. self
        }
    }

    pub fn with_velocity(self, vx: Scalar, vy: Scalar, vz: Scalar) -> BodyParams {
        BodyParams {
            motion: Motion {
                velocity: Vect::new(vx, vy, vz),
                .. self.motion
            },
            .. self
        }
    }

    pub fn with_angular_velocity(self, vx: Scalar, vy: Scalar, vz: Scalar) -> BodyParams {
        BodyParams {
            motion: Motion {
                angular_velocity: Vect::new(vx, vy, vz),
                .. self.motion
            },
            .. self
        }
    }

    pub fn with_restitution_coefficient(self, restitution_coefficient: Scalar) -> BodyParams {
        BodyParams {
            material: self.material.with_coefficient_of_restitution(restitution_coefficient),
            .. self
        }
    }

    pub fn with_friction_coefficient(self, friction_coefficient: Scalar) -> BodyParams {
        BodyParams {
            material: self.material.with_friction_coefficient(friction_coefficient),
            .. self
        }
    }
}

impl Default for BodyParams {
    fn default() -> Self {
        BodyParams {
            motion: Motion::stationary(),
            material: Material::default(),
            transform: Transform::identity(),
            shape_desc: ShapeDesc::Cuboid(1.0, 1.0, 1.0),
        }
    }
}
