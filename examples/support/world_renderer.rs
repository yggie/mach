extern crate mach;
extern crate glium;

use std::cell::Ref;

use self::glium::glutin;
use self::glium::{DisplayBuild, Surface};
use self::glium::backend::glutin_backend::GlutinFacade;

use mach::dynamics::Dynamics;
use mach::collisions::CollisionSpace;

use support::{Camera, ExamplesRenderer, SceneEnv};

pub struct WorldRenderer<C: CollisionSpace, D: Dynamics> {
    world: mach::CustomWorld<C, D>,
    camera: Camera,
    display: GlutinFacade,
    temp_renderer: ExamplesRenderer,
}

impl<C, D> WorldRenderer<C, D> where C: CollisionSpace, D: Dynamics {
    pub fn create(collision_space: C, dynamics: D) -> Result<WorldRenderer<C, D>, String> {
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

        Ok(WorldRenderer {
            world: mach::CustomWorld::new(
                collision_space,
                dynamics,
            ),
            camera: camera,
            display: display,
            temp_renderer: renderer,
        })
    }

    pub fn update_window(&mut self) -> Option<Result<(), String>> {
        if let Some(result) = self.handle_window_events() {
            Some(result)
        } else {
            self.camera.update();

            None
        }
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

impl<C, D> mach::World for WorldRenderer<C, D> where C: CollisionSpace, D: Dynamics {
    #[inline(always)]
    fn create_body(&mut self, entity_desc: &mach::EntityDesc) -> mach::ID {
        self.world.create_body(entity_desc)
    }

    fn create_static_body(&mut self, entity_desc: &mach::EntityDesc) -> mach::ID {
        self.world.create_static_body(entity_desc)
    }

    #[inline(always)]
    fn find_body(&self, id: mach::ID) -> Option<Ref<mach::RigidBody>> {
        self.world.find_body(id)
    }

    #[inline(always)]
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<mach::RigidBody>> + 'a> {
        self.world.bodies_iter()
    }

    #[inline(always)]
    fn update(&mut self, time_step: mach::Scalar) {
        self.world.update(time_step);
    }

    #[inline(always)]
    fn gravity(&self) -> mach::Vector {
        self.world.gravity()
    }

    #[inline(always)]
    fn set_gravity(&mut self, gravity: mach::Vector) {
        self.world.set_gravity(gravity);
    }
}
