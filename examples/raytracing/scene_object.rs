use mach::{UnitQuat, Vec3D};

use raytracing::{Color, SceneGeometry};

#[derive(Clone, Debug)]
pub struct SceneObject {
    pub geometry: SceneGeometry,
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
    pub emission: Color,
    pub position: Vec3D,
    pub rotation: UnitQuat,
    pub shininess: f32,
}

impl Default for SceneObject {
    fn default() -> SceneObject {
        SceneObject {
            geometry: SceneGeometry::Ellipse(1.0, 1.0, 1.0),
            ambient: Color::new(0.2, 0.2, 0.2),
            diffuse: Color::new(0.0, 0.0, 0.0),
            specular: Color::new(0.0, 0.0, 0.0),
            emission: Color::new(0.0, 0.0, 0.0),
            position: Vec3D::zero(),
            rotation: UnitQuat::identity(),
            shininess: 0.0,
        }
    }
}
