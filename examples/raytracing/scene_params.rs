use mach::Scalar;

use raytracing::{DirectionalLight, PointLight, SceneObject};

pub struct SceneParams {
    pub objects: Vec<SceneObject>,
    pub point_lights: Vec<PointLight>,
    pub max_ray_bounces: usize,
    pub directional_lights: Vec<DirectionalLight>,
    pub linear_attenuation: Scalar,
    pub constant_attenuation: Scalar,
    pub quadratic_attenuation: Scalar,
}

impl Default for SceneParams {
    fn default() -> Self {
        SceneParams {
            objects: Vec::new(),
            point_lights: Vec::new(),
            max_ray_bounces: 5,
            directional_lights: Vec::new(),
            linear_attenuation: 0.0,
            constant_attenuation: 1.0,
            quadratic_attenuation: 0.0,
        }
    }
}
