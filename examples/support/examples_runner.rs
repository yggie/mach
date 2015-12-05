extern crate mach;
extern crate glium;

use std;

use self::glium::glutin;
use self::glium::{DisplayBuild, Surface};
use self::glium::backend::glutin_backend::GlutinFacade;

use support::{Camera, CameraDef, ExamplesRenderer, MonitoredWorld, SceneEnv, Simulation};

pub struct ExamplesRunner<S: Simulation> {
    simulation: S,
    camera: Camera,
}

impl<S> ExamplesRunner<S> where S: Simulation {
    pub fn new(simulation: S) -> ExamplesRunner<S> {
        ExamplesRunner {
            simulation: simulation,
            camera: Camera::new(CameraDef::default()),
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

        loop {
            if let Some(result) = self.handle_window_events(&display) {
                return result;
            }

            self.camera.update();
            try!(self.simulation.update(&mut world));
            try!(self.render(&display, &mut renderer, &mut world));

            std::thread::sleep_ms(12);
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
