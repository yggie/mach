use mach::{UnitVec3D, Vec3D};
use raytracing::{Color, SceneParams};

pub trait RayTracer {
    fn from_scene_params(SceneParams) -> Self;
    fn shoot_ray(&self, start: Vec3D, direction: UnitVec3D) -> Color;
}
