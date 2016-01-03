extern crate mach;
extern crate glium;

use self::glium::glutin;
use self::glium::{DisplayBuild, Surface};
use self::glium::backend::glutin_backend::GlutinFacade;

use mach::dynamics::Dynamics;
use mach::detection::Space;

use support::{Camera, ExamplesRenderer, FrameMetadata, SceneEnv, WindowEventHandler, WorldRenderer};

pub struct ExamplesWindow<S: Space, D: Dynamics> {
    world: WorldRenderer<S, D>,
    camera: Camera,
    display: GlutinFacade,
    frame_metadata: FrameMetadata,
    temp_renderer: ExamplesRenderer,
}

impl<S, D> ExamplesWindow<S, D> where S: Space, D: Dynamics {
    pub fn create(world: mach::CustomWorld<S, D>) -> Result<ExamplesWindow<S, D>, String> {
        let display = try!(
            glutin::WindowBuilder::new()
                .with_dimensions(640, 480)
                .build_glium()
                .map_err(|err| format!("{}", err))
        );

        let (w, h) = display.get_max_viewport_dimensions();

        let renderer = try!(ExamplesRenderer::new(&display));

        Ok(ExamplesWindow {
            world: WorldRenderer::new(world),
            camera: Camera::new(w, h),
            display: display,
            temp_renderer: renderer,
            frame_metadata: FrameMetadata::new(),
        })
    }

    pub fn update(&mut self) -> Option<Result<(), String>> {
        if let Some(result) = self.handle_window_events() {
            Some(result)
        } else {
            self.camera.update();

            None
        }
    }

    pub fn world_mut(&mut self) -> &mut mach::World {
        &mut self.world
    }

    pub fn render_frame(&mut self) -> Result<(), String> {
        let mut target = self.display.draw();
        try!(self.temp_renderer.render(&mut target, &mut self.world, &self.frame_metadata, &SceneEnv {
            camera: &self.camera,
        }));

        self.frame_metadata.contacts = Vec::new();
        return target.finish().map_err(|err| format!("{:?}", err));
    }

    pub fn handle_contact_events(&mut self, events: Option<Vec<mach::detection::Contact>>) {
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
                &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Escape)) => {
                    return Some(Ok(()));
                },

                _otherwise => {
                    self.camera.handle_event(event);
                },
            }
        }

        return None;
    }
}
