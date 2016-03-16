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

        let entity_desc = mach::entities::BodyParams::default()
            .with_density(1.0)
            .with_restitution_coefficient(0.5);

        world.create_rigid_body(
            &entity_desc.clone()
                .as_cube(1.0)
                .with_translation(0.0, 0.0, 3.0)
                .with_velocity(0.5, 0.0, -1.0)
                .with_angular_velocity(0.3, 0.5, 0.4)
        );

        world.create_static_body(
            &entity_desc.clone()
                .as_cuboid(10.0, 10.0, 0.1)
                .with_translation(0.0, 0.0, -1.0)
        );

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
