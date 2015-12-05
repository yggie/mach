extern crate mach;
extern crate glium;

use std;
use std::mem;
use std::collections::HashMap;

use self::glium::backend::glutin_backend::GlutinFacade;

use support::{Camera, SceneEnv, Instance, InstanceFactory};

#[derive(Clone, Copy)]
pub struct Vertex {
    position: (f32, f32, f32),
}
implement_vertex!(Vertex, position);

#[derive(Clone, Copy)]
pub struct Normal {
    normal: (f32, f32, f32),
}
implement_vertex!(Normal, normal);

pub struct ExamplesRenderer {
    program: glium::Program,
    cube: (glium::VertexBuffer<Vertex>, glium::VertexBuffer<Normal>, glium::IndexBuffer<u16>),
    instances: HashMap<mach::ID, Instance>,
    factory: InstanceFactory,
}

impl ExamplesRenderer {
    pub fn new(display: &GlutinFacade) -> Result<ExamplesRenderer, String> {
        let program = try!(
            glium::Program::from_source(
                display,
                include_str!("shaders/foreground.vert.glsl"),
                include_str!("shaders/foreground.frag.glsl"),
                None,
            ).map_err(|err| format!("{}", err))
        );

        let cube = {
            let vertices: [Vertex; 8] = [
                Vertex { position: (-1.0, -1.0, -1.0) },
                Vertex { position: ( 1.0, -1.0, -1.0) },
                Vertex { position: ( 1.0,  1.0, -1.0) },
                Vertex { position: (-1.0,  1.0, -1.0) },
                Vertex { position: (-1.0, -1.0,  1.0) },
                Vertex { position: ( 1.0, -1.0,  1.0) },
                Vertex { position: ( 1.0,  1.0,  1.0) },
                Vertex { position: (-1.0,  1.0,  1.0) },
            ];

            let normals: [Normal; 8] = [
                Normal { normal: (-1.0, -1.0, -1.0) },
                Normal { normal: ( 1.0, -1.0, -1.0) },
                Normal { normal: ( 1.0,  1.0, -1.0) },
                Normal { normal: (-1.0,  1.0, -1.0) },
                Normal { normal: (-1.0, -1.0,  1.0) },
                Normal { normal: ( 1.0, -1.0,  1.0) },
                Normal { normal: ( 1.0,  1.0,  1.0) },
                Normal { normal: (-1.0,  1.0,  1.0) },
            ];

            let indices: [u16; 36] = [
                0, 5, 1,
                0, 4, 5,
                3, 6, 7,
                3, 2, 6,
                1, 6, 2,
                1, 5, 6,
                0, 3, 7,
                0, 7, 4,
                0, 2, 3,
                0, 1, 2,
                4, 6, 5,
                4, 7, 6,
            ];

            let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
            let normal_buffer = glium::VertexBuffer::new(display, &normals).unwrap();
            let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

            (vertex_buffer, normal_buffer, indices)
        };

        Ok(ExamplesRenderer {
            instances: HashMap::new(),
            cube: cube,
            program: program,
            factory: InstanceFactory::new(),
        })
    }

    pub fn render<S: glium::Surface>(&mut self, surface: &mut S, world: &mach::World, env: &SceneEnv) -> Result<(), String> {
        surface.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let mut old_instances = HashMap::new();

        std::mem::swap(&mut old_instances, &mut self.instances);

        for body in world.bodies_iter() {
            if let Some(instance) = old_instances.remove(&body.id()) {
                self.render_and_save(surface, instance, body.state().transform(), env);
            } else {
                let instance = self.factory.generate(body.id(), body.shape());

                self.render_and_save(surface, instance, body.state().transform(), env);
            }
        }

        return Ok(());
    }

    fn render_and_save<S: glium::Surface>(&mut self, surface: &mut S, instance: Instance, transform: &mach::maths::Transform, env: &SceneEnv) {
        match instance.shape_spec {
            mach::ShapeSpec::Cuboid { width, height, depth } => {
                let model_matrix: [[f32; 4]; 4] = [
                    [width, 0.0, 0.0, 0.0],
                    [0.0, height, 0.0, 0.0],
                    [0.0, 0.0, depth, 0.0],
                    [transform.translation().x, transform.translation().y, transform.translation().z, 1.0],
                ];

                let view_matrix: [[f32; 4]; 4] = unsafe {
                    mem::transmute(env.camera.view_matrix())
                };

                surface.draw(
                    (&self.cube.0, &self.cube.1),
                    &self.cube.2,
                    &self.program,
                    &uniform! {
                        projection_matrix: *env.camera.projection_matrix(),
                        view_matrix: view_matrix,
                        model_matrix: model_matrix,
                        light_direction: [1.0, 2.0, 1.0f32],
                        surface_color: instance.color.clone(),
                    },
                    &glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::draw_parameters::DepthTest::IfLess,
                            write: true,
                            .. Default::default()
                        },
                        .. Default::default()
                    },
                ).unwrap();
            },

            _ => (),
        }
        self.instances.insert(instance.id, instance);
    }
}
