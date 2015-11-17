use mach::{ World, Scalar };
use mach::dynamics::{ Dynamics, SimpleDynamics };
use mach::collisions::{ CollisionSpace, SimpleCollisionSpace };

use support::{ CollisionSpaceMonitor, DynamicsMonitor };

pub struct Simulation<C: CollisionSpace, D: Dynamics> {
    world: World<CollisionSpaceMonitor<C>, DynamicsMonitor<D>>,
    did_assert: bool
}

impl<C: CollisionSpace, D: Dynamics> Simulation<C, D> {
    pub fn new_default() -> Simulation<SimpleCollisionSpace, SimpleDynamics> {
        let collisions = SimpleCollisionSpace::new();
        let dynamics = SimpleDynamics::new();
        let world = World::new(
            CollisionSpaceMonitor::new(collisions),
            DynamicsMonitor::new(dynamics)
        );

        println!("[RENDERABLE]");

        return Simulation {
            world: world,
            did_assert: false
        };
    }

    pub fn configure<F: FnOnce(&mut World<CollisionSpaceMonitor<C>, DynamicsMonitor<D>>)>(&mut self, func: F) -> &mut Simulation<C, D> {
        self.did_assert = false;

        func(&mut self.world);

        return self;
    }

    pub fn execute_multiple_steps(&mut self, count: u32, step: Scalar) -> &mut Simulation<C, D> {
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
