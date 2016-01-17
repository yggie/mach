extern crate mach;
#[macro_use]
extern crate glium;

mod support;

use self::support::Simulation;

struct TwoCollidingCubes;

impl Simulation for TwoCollidingCubes {
    fn name(&self) -> &'static str {
        "Two Colliding Cubes"
    }

    fn setup(&mut self, world: &mut mach::World) -> Result<(), String> {
        let params = mach::entities::BodyParams::cube(1.0);

        world.create_rigid_body(
            &params.clone()
                .with_density(1.0)
                .with_translation(0.0, 0.0, 3.0)
                .with_velocity(0.0, 0.0, -1.0)
        );

        world.create_rigid_body(
            &params.clone()
                .with_density(2.0)
                .with_translation(0.0, 0.0, -3.0)
                .with_axis_angle(mach::Vect::new(1.0, 0.0, 1.0), 1.0)
                .with_velocity(0.0, 0.0, 1.0)
        );

        return Ok(());
    }

    fn update(&mut self, world: &mut mach::World) -> Result<Option<Vec<mach::detection::Contact>>, String> {
        return Ok(world.update(0.05));
    }
}

fn main() {
    support::ExamplesRunner::new(TwoCollidingCubes)
        .with_fps(30)
        .run();
}
