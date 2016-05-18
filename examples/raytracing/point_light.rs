use mach::{Scalar, Vec3D};

use raytracing::Color;

pub struct PointLight {
    color: Color,
    position: Vec3D,
}

impl PointLight {
    pub fn with_position(self, x: Scalar, y: Scalar, z: Scalar) -> PointLight {
        PointLight {
            position: Vec3D::new(x, y, z),
            .. self
        }
    }

    pub fn with_color(self, r: f32, g: f32, b: f32) -> PointLight {
        PointLight {
            color: Color::new(r, g, b),
            .. self
        }
    }
}

impl Default for PointLight {
    fn default() -> Self {
        PointLight {
            color: Color::new(1.0, 1.0, 1.0),
            position: Vec3D::zero(),
        }
    }
}
