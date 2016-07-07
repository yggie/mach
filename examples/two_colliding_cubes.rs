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

    fn setup(&mut self, world: &mut mach::World<mach::MachBody<()>>) -> Result<(), String> {
        world.create_rigid_body(mach::dynamics::RigidBodyDef {
            mass: 1.0,
            shape: Box::new(mach::shapes::Cuboid::cube(1.0)),
            velocity: mach::Vec3D::new(0.0, 0.0, -1.0),
            translation: mach::Vec3D::new(0.0, 0.0, 3.0),
            .. mach::dynamics::RigidBodyDef::default()
        }, ());

        world.create_rigid_body(mach::dynamics::RigidBodyDef {
            mass: 2.0,
            shape: Box::new(mach::shapes::Cuboid::cube(1.0)),
            rotation: mach::UnitQuat::from_axis_angle(mach::Vec3D::new(1.0, 0.0, 1.0).normalize(), 1.0),
            velocity: mach::Vec3D::new(0.0, 0.0, 1.0),
            translation: mach::Vec3D::new(0.0, 0.0, -3.0),
            .. mach::dynamics::RigidBodyDef::default()
        }, ());

        return Ok(());
    }

    fn update(&mut self, world: &mut mach::World<mach::MachBody<()>>) -> Result<Vec<mach::collisions::Contact<mach::MachBody<()>>>, String> {
        // let total_kinetic_energy = world.rigid_bodies_iter().fold(0.0, |total, rigid_body| {
        //     total + rigid_body.mass() * rigid_body.velocity().length_sq()
        // });
        //
        // println!("TOTAL KE: {}", total_kinetic_energy);

        return Ok(world.update(0.05));
    }
}

fn main() {
    support::ExamplesRunner::new(TwoCollidingCubes)
        .with_fps(30)
        .run();
}
