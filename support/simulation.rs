extern crate mach;

pub trait Simulation {
    fn name(&self) -> &'static str;

    fn setup(&mut self, _world: &mut mach::World<mach::MachBody<()>>) -> Result<(), String> {
        Ok(())
    }

    fn update(&mut self, _world: &mut mach::World<mach::MachBody<()>>) -> Result<Vec<mach::collisions::Contact<mach::MachBody<()>>>, String>;
}
