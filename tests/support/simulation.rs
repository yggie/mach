use mach::World;
use mach::dynamics::{ Dynamics, SimpleDynamics };
use mach::collisions::{ Collisions, SimpleCollisions };

use support::{ CollisionsMonitor, DynamicsMonitor };

pub struct Simulation<C: Collisions<Identifier=usize>, D: Dynamics<Identifier=usize>> {
    world: World<usize, CollisionsMonitor<C>, DynamicsMonitor<D>>,
    did_assert: bool
}

impl<C: Collisions<Identifier=usize>, D: Dynamics<Identifier=usize>> Simulation<C, D> {
    pub fn new_default() -> Simulation<SimpleCollisions, SimpleDynamics> {
        let collisions = SimpleCollisions::new();
        let dynamics = SimpleDynamics::new();
        let world = World::new(
            CollisionsMonitor::new(collisions),
            DynamicsMonitor::new(dynamics)
        );

        return Simulation {
            world: world,
            did_assert: false
        };
    }

    pub fn configure<F: FnOnce(&mut World<usize, CollisionsMonitor<C>, DynamicsMonitor<D>>)>(&mut self, func: F) -> &mut Simulation<C, D> {
        self.did_assert = false;

        func(&mut self.world);

        return self;
    }

    pub fn execute_multiple_steps(&mut self, count: u32, step: f32) -> &mut Simulation<C, D> {
        for _ in (0..count) {
            self.world.update(step);
        }

        return self;
    }

    pub fn assert_compliance(&mut self) {
        self.did_assert = true;

        if !self.did_assert {
            panic!("The simulation did not check for any violations!")
        }
    }
}
