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

    fn setup(&mut self, world: &mut mach::World<mach::MachBody<()>>) -> Result<(), String> {
        world.set_gravity(mach::Vec3D::new(0.0, 0.0, -0.5));

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
            world.create_rigid_body(mach::dynamics::RigidBodyDef {
                shape: Box::new(mach::collisions::geometry::shapes::Cuboid::cube(1.0)),
                rotation: mach::UnitQuat::from_axis_angle(direction, mach::PI / 8.0),
                velocity: direction * 3.0,
                translation: mach::Vec3D::new(offset, 0.0, 0.0),
                .. mach::dynamics::RigidBodyDef::default()
            }, ());
        }

        let margin = 3.0;

        world.create_fixed_body(mach::dynamics::FixedBodyDef {
            shape: Box::new(mach::collisions::geometry::shapes::Cuboid::new(range + 2.0 * margin, range + 2.0 * margin, 1.0)),
            translation: mach::Vec3D::new(0.0, 0.0, -margin),
            .. mach::dynamics::FixedBodyDef::default()
        }, ());

        world.create_fixed_body(mach::dynamics::FixedBodyDef {
            shape: Box::new(mach::collisions::geometry::shapes::Cuboid::new(range + 2.0 * margin, 1.0, range + 2.0 * margin)),
            translation: mach::Vec3D::new(0.0, range/2.0 + margin, 0.0),
            .. mach::dynamics::FixedBodyDef::default()
        }, ());

        world.create_fixed_body(mach::dynamics::FixedBodyDef {
            shape: Box::new(mach::collisions::geometry::shapes::Cuboid::new(range + 2.0 * margin, 1.0, range + 2.0 * margin)),
            translation: mach::Vec3D::new(0.0, -range/2.0 - margin, 0.0),
            .. mach::dynamics::FixedBodyDef::default()
        }, ());

        world.create_fixed_body(mach::dynamics::FixedBodyDef {
            shape: Box::new(mach::collisions::geometry::shapes::Cuboid::new(1.0, range + 2.0 * margin, range + 2.0 * margin)),
            translation: mach::Vec3D::new(range/2.0 + margin, 0.0, 0.0),
            .. mach::dynamics::FixedBodyDef::default()
        }, ());

        world.create_fixed_body(mach::dynamics::FixedBodyDef {
            shape: Box::new(mach::collisions::geometry::shapes::Cuboid::new(1.0, range + 2.0 * margin, range + 2.0 * margin)),
            translation: mach::Vec3D::new(-range/2.0 - margin, 0.0, 0.0),
            .. mach::dynamics::FixedBodyDef::default()
        }, ());

        return Ok(());
    }

    fn update(&mut self, world: &mut mach::World<mach::MachBody<()>>) -> Result<Vec<mach::collisions::Contact<mach::MachBody<()>>>, String> {
        return Ok(world.update(0.05));
    }
}

fn main() {
    support::ExamplesRunner::new(DroppedCube)
        .with_fps(30)
        .run();
}
