extern crate mach;
#[macro_use]
extern crate glium;
extern crate rand;

use self::rand::{Rng, SeedableRng};

mod support;

use self::support::Simulation;

struct DroppedCube;

impl Simulation for DroppedCube {
    fn name(&self) -> &'static str {
        "Dropped Cube"
    }

    fn setup(&mut self, world: &mut mach::World) -> Result<(), String> {
        world.set_gravity(mach::Vec3D::new(0.0, 0.0, -0.5));
        let prototype = mach::RigidBody::default()
            .with_mass(1.0);

        let mut rng = rand::StdRng::from_seed(&[1, 2, 3, 4]);
        let upper_bound = 10.0;
        let lower_bound = -10.0;
        let range = upper_bound - lower_bound;

        let limit = 10;
        for index in 0..limit {
            let direction = mach::Vec3D::new(
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
            ).normalize();

            let offset = lower_bound + range * index as mach::Scalar / limit as mach::Scalar;
            world.add_rigid_body(prototype.clone()
                .with_shape(mach::shapes::Cuboid::cube(1.0))
                .with_translation(offset, 0.0, 0.0)
                .with_axis_angle(direction, mach::PI / 8.0)
                .with_velocity_vect(direction * 3.0));
        }

        let margin = 3.0;

        world.add_static_body(mach::StaticBody::default()
            .with_shape(mach::shapes::Cuboid::new(range + 2.0 * margin, range + 2.0 * margin, 1.0))
            .with_translation(0.0, 0.0, -margin));

        world.add_static_body(mach::StaticBody::default()
            .with_shape(mach::shapes::Cuboid::new(range + 2.0 * margin, 1.0, range + 2.0 * margin))
            .with_translation(0.0, range/2.0 + margin, 0.0));

        world.add_static_body(mach::StaticBody::default()
            .with_shape(mach::shapes::Cuboid::new(range + 2.0 * margin, 1.0, range + 2.0 * margin))
            .with_translation(0.0, -range/2.0 - margin, 0.0));

        world.add_static_body(mach::StaticBody::default()
            .with_shape(mach::shapes::Cuboid::new(1.0, range + 2.0 * margin, range + 2.0 * margin))
            .with_translation(range/2.0 + margin, 0.0, 0.0));

        world.add_static_body(mach::StaticBody::default()
            .with_shape(mach::shapes::Cuboid::new(1.0, range + 2.0 * margin, range + 2.0 * margin))
            .with_translation(-range/2.0 - margin, 0.0, 0.0));

        return Ok(());
    }

    fn update(&mut self, world: &mut mach::World) -> Result<Vec<mach::detection::ContactEvent>, String> {
        return Ok(world.update(0.05));
    }
}

fn main() {
    support::ExamplesRunner::new(DroppedCube)
        .with_fps(30)
        .run();
}
