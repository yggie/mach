extern crate mach;
extern crate time;
extern crate glium;

use std;

use self::glium::glutin;
use self::glium::{DisplayBuild, Surface};
use self::glium::backend::glutin_backend::GlutinFacade;

use support::{Camera, CameraDef, ExamplesRenderer, MonitoredWorld, SceneEnv, Simulation};

pub struct ExamplesRunner<S: Simulation> {
    simulation: S,
    camera: Camera,
    desired_fps: u8,
}

impl<S> ExamplesRunner<S> where S: Simulation {
    pub fn new(simulation: S) -> ExamplesRunner<S> {
        ExamplesRunner {
            simulation: simulation,
            camera: Camera::new(CameraDef::default()),
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
        let mut world = MonitoredWorld::new(
            mach::collisions::SimpleCollisionSpace::new(),
            mach::dynamics::SimpleDynamics::new(),
        );

        let display = try!(
            glutin::WindowBuilder::new()
                .build_glium()
                .map_err(|err| format!("{}", err))
        );

        let (w, h) = display.get_max_viewport_dimensions();
        self.camera.set_viewport_dimensions(w, h);

        let mut renderer = try!(ExamplesRenderer::new(&display));

        try!(self.simulation.setup(&mut world));

        let nanoseconds_per_frame = 1_000_000_000 / (self.desired_fps as u64);
        loop {
            let start_time = time::precise_time_ns();

            if let Some(result) = self.handle_window_events(&display) {
                return result;
            }

            self.camera.update();
            try!(self.simulation.update(&mut world));
            try!(self.render(&display, &mut renderer, &mut world));

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

    fn handle_window_events(&mut self, display: &GlutinFacade) -> Option<Result<(), String>> {
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed |
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Escape)) => {
                    return Some(Ok(()));
                },

                glutin::Event::MouseMoved((x, y)) => {
                    self.camera.on_mouse_move(x as f32, y as f32);
                },

                _ => ()
            }
        }

        return None;
    }

    fn render(&self, display: &GlutinFacade, renderer: &mut ExamplesRenderer, world: &mut mach::World) -> Result<(), String> {
        let mut target = display.draw();
        try!(renderer.render(&mut target, world, &SceneEnv {
            camera: &self.camera,
        }));
        return target.finish().map_err(|err| format!("{:?}", err));
    }
}
