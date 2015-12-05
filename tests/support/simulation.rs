extern crate mach;

pub trait Simulation {
    fn name(&self) -> &'static str;

    fn setup(&mut self, _world: &mut mach::World) -> Result<(), String> {
        Ok(())
    }

    fn update(&mut self, _world: &mut mach::World) -> Result<(), String>;
}
