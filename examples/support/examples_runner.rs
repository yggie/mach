extern crate mach;
extern crate time;
extern crate glium;

use std;

use self::glium::glutin;
use self::glium::{DisplayBuild, Surface};
use self::glium::backend::glutin_backend::GlutinFacade;

use support::{ExamplesRenderer, SceneEnv, Simulation, WorldRenderer};

pub struct ExamplesRunner<S: Simulation> {
    simulation: S,
    desired_fps: u8,
}

impl<S> ExamplesRunner<S> where S: Simulation {
    pub fn new(simulation: S) -> ExamplesRunner<S> {
        ExamplesRunner {
            simulation: simulation,
            desired_fps: 30,
        }
    }

    pub fn with_fps(self, fps: u8) -> ExamplesRunner<S> {
        ExamplesRunner {
            desired_fps: fps,
            .. self
        }
    }

    pub fn run(&mut self) {
        if let Err(message) = self.safe_run() {
            println!("An error occurred when running the simulation [{}]: \"{}\"", self.simulation.name(), message);
        } else {
            println!("Example [{}] successfully exited without any errors", self.simulation.name());
        }
    }

    fn safe_run(&mut self) -> Result<(), String> {
        let mut world = try!(WorldRenderer::create(
            mach::collisions::SimpleCollisionSpace::new(),
            mach::dynamics::SimpleDynamics::new(),
        ));

        try!(self.simulation.setup(&mut world));

        let nanoseconds_per_frame = 1_000_000_000 / (self.desired_fps as u64);
        loop {
            let start_time = time::precise_time_ns();

            if let Some(result) = world.update_window() {
                return result;
            }

            try!(self.simulation.update(&mut world));
            try!(world.render_frame());

            let time_taken = time::precise_time_ns() - start_time;
            if time_taken < nanoseconds_per_frame {
                std::thread::sleep_ms(((nanoseconds_per_frame - time_taken) / 1_000_000) as u32);
            }

            let time_taken = time::precise_time_ns() - start_time;
            let fps = 1_000_000_000 as f32 / time_taken as f32;
            // TODO eventually render this on screen
            println!("FPS: {}", fps);
        }
    }
}