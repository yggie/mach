extern crate mach;
#[macro_use]
extern crate glium;

mod support;

use self::support::Simulation;

struct DroppedCube;

impl Simulation for DroppedCube {
    fn name(&self) -> &'static str {
        "Dropped Cube"
    }

    fn setup(&mut self, world: &mut mach::World) -> Result<(), String> {
        world.set_gravity(mach::Vect::new(0.0, 0.0, -0.5));

        world.add_rigid_body(mach::RigidBody::default()
            .with_shape(mach::shapes::Cuboid::cube(1.0))
            .with_mass(1.0)
            .with_restitution_coefficient(0.5)
            .with_translation(0.0, 0.0, 3.0)
            .with_velocity(0.5, 0.0, -1.0)
            .with_angular_velocity(0.3, 0.5, 0.4));

        world.add_static_body(mach::StaticBody::default()
            .with_shape(mach::shapes::Cuboid::new(10.0, 10.0, 0.1))
            .with_restitution_coefficient(0.5)
            .with_translation(0.0, 0.0, -1.0));

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
