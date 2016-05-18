use mach::{Scalar, UnitVec3D, Vec3D};

use raytracing::Color;

pub struct DirectionalLight {
    color: Color,
    direction: UnitVec3D,
}

impl DirectionalLight {
    pub fn with_direction(self, x: Scalar, y: Scalar, z: Scalar) -> DirectionalLight {
        DirectionalLight {
            direction: Vec3D::new(x, y, z).normalize(),
            .. self
        }
    }

    pub fn with_color(self, r: f32, g: f32, b: f32) -> DirectionalLight {
        DirectionalLight {
            color: Color::new(r, g, b),
            .. self
        }
    }
}

impl Default for DirectionalLight {
    fn default() -> Self {
        DirectionalLight {
            color: Color::new(1.0, 1.0, 1.0),
            direction: Vec3D::new(0.0, 0.0, 1.0).normalize(),
        }
    }
}
