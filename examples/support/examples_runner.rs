extern crate mach;
extern crate glium;

use support::{ExamplesWindow, ExampleWorld, Simulation};

pub struct ExamplesRunner<S> where S: Simulation {
    pub simulation: S,
    pub desired_fps: u8,
    pub world_constructor: Box<Fn() -> Box<ExampleWorld>>,
}

impl<S> ExamplesRunner<S> where S: Simulation {
    pub fn new(simulation: S) -> ExamplesRunner<S> {
        ExamplesRunner {
            simulation: simulation,
            desired_fps: 30,
            world_constructor: Box::new(|| Box::new(mach::MachWorld::new())),
        }
    }

    pub fn with_fps(self, fps: u8) -> ExamplesRunner<S> {
        ExamplesRunner {
            desired_fps: fps,
            .. self
        }
    }

    pub fn run(self) {
        let simulation_name = self.simulation.name();
        if let Err(message) = ExamplesWindow::execute(self) {
            println!("An error occurred when running the simulation [{}]: \"{}\"", simulation_name, message);
        } else {
            println!("Example [{}] successfully exited without any errors", simulation_name);
        }
    }
}
