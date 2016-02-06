extern crate mach;
extern crate time;
extern crate glium;

use std;

use self::glium::glutin;
use self::glium::{DisplayBuild, Surface};
use self::glium::backend::glutin_backend::GlutinFacade;

use support::{Camera, ExamplesRenderer, ExamplesRunner, FrameMetadata, SceneEnv, Simulation, WindowEventHandler};

macro_rules! key_released {
    ($key:path) => {
        glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some($key))
    };
}

pub struct ExamplesWindow<S: Simulation> {
    world: Box<mach::World>,
    camera: Camera,
    display: GlutinFacade,
    renderer: ExamplesRenderer,
    simulation: S,
    desired_fps: u8,
    frame_metadata: FrameMetadata,
    world_constructor: Box<Fn() -> Box<mach::World>>,
}

impl<S> ExamplesWindow<S> where S: Simulation {
    pub fn execute(runner: ExamplesRunner<S>) -> Result<(), String> {
        try!(ExamplesWindow::create(runner)).run()
    }

    fn create(mut runner: ExamplesRunner<S>) -> Result<ExamplesWindow<S>, String> {
        let display = try!(
            glutin::WindowBuilder::new()
                .with_dimensions(640, 480)
                .build_glium()
                .map_err(|err| format!("{}", err))
        );

        let (w, h) = display.get_max_viewport_dimensions();

        let renderer = try!(ExamplesRenderer::new(&display));
        let mut world = (*runner.world_constructor)();
        try!(runner.simulation.setup(&mut *world));

        Ok(ExamplesWindow {
            world: world,
            camera: Camera::new(w, h),
            display: display,
            renderer: renderer,
            simulation: runner.simulation,
            desired_fps: runner.desired_fps,
            frame_metadata: FrameMetadata::new(),
            world_constructor: runner.world_constructor,
        })
    }

    fn run(mut self) -> Result<(), String> {
        loop {
            let nanoseconds_per_frame = 1_000_000_000 / (self.desired_fps as u64);
            let start_time = time::precise_time_ns();

            if let Some(result) = self.handle_window_events() {
                return result;
            }

            let contacts_option = try!(self.simulation.update(&mut *self.world));
            self.handle_contact_events(contacts_option);

            self.camera.update();
            try!(self.render_frame());

            let time_taken = time::precise_time_ns() - start_time;
            if time_taken < nanoseconds_per_frame {
                std::thread::sleep(std::time::Duration::new(0, (nanoseconds_per_frame - time_taken) as u32));
            }

            let time_taken = time::precise_time_ns() - start_time;
            let fps = 1_000_000_000 as f32 / time_taken as f32;
            // TODO eventually render this on screen
            println!("FPS: {}", fps);
        }
    }

    fn render_frame(&mut self) -> Result<(), String> {
        let mut target = self.display.draw();
        try!(self.renderer.render(&mut target, &mut *self.world, &self.frame_metadata, &SceneEnv {
            camera: &self.camera,
        }));

        self.frame_metadata.contacts = Vec::new();
        return target.finish().map_err(|err| format!("{:?}", err));
    }

    fn handle_contact_events(&mut self, events: Option<Vec<mach::detection::Contact>>) {
        self.frame_metadata.contacts = events.map_or(Vec::new(), |contacts: Vec<mach::detection::Contact>| -> Vec<mach::maths::Vect> {
            return contacts.into_iter()
                .map(|contact| contact.center.clone())
                .collect();
        });
    }

    fn handle_window_events(&mut self) -> Option<Result<(), String>> {
        for ref event in self.display.poll_events() {
            match event {
                &glutin::Event::Closed |
                &key_released!(glutin::VirtualKeyCode::Escape) => {
                    return Some(Ok(()));
                },

                &key_released!(glutin::VirtualKeyCode::R) => {
                    self.world = (*self.world_constructor)();

                    if let Err(message) = self.simulation.setup(&mut *self.world) {
                        return Some(Err(message));
                    }
                },

                &key_released!(glutin::VirtualKeyCode::Up) => {
                    self.desired_fps = self.desired_fps + 1;
                },

                &key_released!(glutin::VirtualKeyCode::Down) => {
                    self.desired_fps = self.desired_fps - 1;
                },

                _otherwise => {
                    self.camera.handle_event(event);
                },
            }
        }

        return None;
    }
}
