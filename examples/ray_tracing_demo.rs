#[macro_use]
extern crate glium;
extern crate mach;
extern crate time;

mod raytracing;

use mach::{CollisionObjectSpace, MachWorld, UnitVec3D, Vec3D, World};
use raytracing::{Color, DirectionalLight, Importable, PointLight, RayTracer, RayTracingRenderer, SceneGeometry, SceneParams};

fn main() {
    let renderer = RayTracingRenderer::<RayTracingDemo>::import_from("examples/assets/scene6.txt").unwrap();

    raytracing::render(renderer);
}

struct RayTracingDemo {
    world: MachWorld<()>,
    point_lights: Vec<PointLight>,
    max_ray_bounces: usize,
    directional_lights: Vec<DirectionalLight>,
}

impl RayTracer for RayTracingDemo {
    fn from_scene_params(params: SceneParams) -> Self {
        let mut world = MachWorld::new();

        for object in params.objects.iter() {
            match object.geometry {
                SceneGeometry::Ellipse(x, y, z) => {
                    // TODO not ideal!
                    let average = (x + y + z) / 3.0;

                    world.create_fixed_body(mach::dynamics::FixedBodyDef {
                        shape: Box::new(mach::collisions::geometry::shapes::Sphere::new(average)),
                        rotation: object.rotation,
                        translation: object.position,
                        .. mach::dynamics::FixedBodyDef::default()
                    }, ());
                }
            }
        }

        RayTracingDemo {
            world: world,
            point_lights: params.point_lights,
            max_ray_bounces: params.max_ray_bounces,
            directional_lights: params.directional_lights,
        }
    }

    fn shoot_ray(&self, source: Vec3D, direction: UnitVec3D) -> Color {
        let ray = mach::collisions::geometry::Ray::new(source, direction);
        match self.world.cast_ray(ray) {
            Some(body) => {
                let d = Vec3D::from(direction);
                Color::new(d.x.abs() as f32, d.y.abs() as f32, d.z.abs() as f32)
            },

            // TODO take from background?
            _otherwise => Color::new(0.0, 0.0, 0.0),
        }
    }
}
