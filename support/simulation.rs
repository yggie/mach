extern crate mach;

pub trait Simulation {
    fn name(&self) -> &'static str;

    fn setup<N>(&mut self, _world: &mut mach::World<N, ()>) -> Result<(), String> where N: mach::collisions::Narrowphase {
        Ok(())
    }

    fn update<N>(&mut self, _world: &mut mach::World<N, ()>) -> Result<Vec<mach::collisions::Contact<N, mach::dynamics::DynamicBodyType<()>>>, String> where N: mach::collisions::Narrowphase;
}
