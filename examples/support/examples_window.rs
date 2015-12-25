extern crate mach;
extern crate glium;

use self::glium::glutin;
use self::glium::{DisplayBuild, Surface};
use self::glium::backend::glutin_backend::GlutinFacade;

use mach::dynamics::Dynamics;
use mach::detection::Space;

use support::{Camera, ExamplesRenderer, SceneEnv, WorldRenderer};

pub struct ExamplesWindow<S: Space, D: Dynamics> {
    world: WorldRenderer<S, D>,
    camera: Camera,
    display: GlutinFacade,
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

        let mut camera = Camera::default();

        let (w, h) = display.get_max_viewport_dimensions();
        camera.set_viewport_dimensions(w, h);

        let renderer = try!(ExamplesRenderer::new(&display));

        Ok(ExamplesWindow {
            world: WorldRenderer::new(world),
            camera: camera,
            display: display,
            temp_renderer: renderer,
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
        try!(self.temp_renderer.render(&mut target, &mut self.world, &SceneEnv {
            camera: &self.camera,
        }));

        return target.finish().map_err(|err| format!("{:?}", err));
    }

    fn handle_window_events(&mut self) -> Option<Result<(), String>> {
        for event in self.display.poll_events() {
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
}
