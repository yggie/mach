extern crate mach;
extern crate glium;

use std::rc::Rc;
use std::mem;
use std::collections::HashMap;

use self::glium::backend::glutin_backend::GlutinFacade;

use support::{SceneEnv, Instance, InstanceFactory, FrameMetadata, PolygonModel};
use support::polygons;

pub struct ExamplesRenderer {
    program: glium::Program,
    cube: Rc<PolygonModel>,
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

        Ok(ExamplesRenderer {
            instances: HashMap::new(),
            cube: Rc::new(polygons::initialize_cube(display)),
            program: program,
            factory: InstanceFactory::new(),
        })
    }

    pub fn render<S: glium::Surface>(&mut self, surface: &mut S, world: &mach::World, frame_metadata: &FrameMetadata, env: &SceneEnv) -> Result<(), String> {
        surface.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let mut old_instances = HashMap::new();

        mem::swap(&mut old_instances, &mut self.instances);

        for body in world.rigid_bodies_iter() {
            if let Some(instance) = old_instances.remove(&body.id()) {
                self.render_and_save(surface, instance, body.transform(), env);
            } else {
                let instance = self.generate_new_instance(body.id(), body.shape());

                self.render_and_save(surface, instance, body.transform(), env);
            }
        }

        try!(self.render_contacts(surface, &frame_metadata.contacts, env));
        return Ok(());
    }

    fn render_contacts<S: glium::Surface>(&mut self, surface: &mut S, contacts: &Vec<mach::maths::Vect>, env: &SceneEnv) -> Result<(), String> {
        let mut model_matrix: [[f32; 4]; 4] = [
            [0.1, 0.0, 0.0, 0.0],
            [0.0, 0.1, 0.0, 0.0],
            [0.0, 0.0, 0.1, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        for contact in contacts.iter() {
            model_matrix[3][0] = contact.x;
            model_matrix[3][1] = contact.y;
            model_matrix[3][2] = contact.z;

            surface.draw(
                (&self.cube.vertices, &self.cube.normals),
                &self.cube.indices,
                &self.program,
                &uniform! {
                    projection_matrix: *env.camera.projection_matrix(),
                    view_matrix: env.camera.view_matrix(),
                    model_matrix: model_matrix,
                    light_direction: [1.0, 2.0, 1.0f32],
                    surface_color: [1.0, 1.0, 0.0, 1.0f32],
                },
                &glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::Overwrite,
                        write: false,
                        .. Default::default()
                    },
                    .. Default::default()
                },
            ).unwrap();
        }

        return Ok(());
    }

    fn render_and_save<S: glium::Surface>(&mut self, surface: &mut S, instance: Instance, transform: &mach::maths::Transform, env: &SceneEnv) {
        let quat = transform.rotation().normalize();
        let r11 = 1.0 - 2.0*quat.j*quat.j - 2.0*quat.k*quat.k;
        let r12 = 2.0*quat.i*quat.j - 2.0*quat.r*quat.k;
        let r13 = 2.0*quat.i*quat.k + 2.0*quat.r*quat.j;
        let r21 = 2.0*quat.i*quat.j + 2.0*quat.r*quat.k;
        let r22 = 1.0 - 2.0*quat.i*quat.i - 2.0*quat.k*quat.k;
        let r23 = 2.0*quat.j*quat.k - 2.0*quat.r*quat.i;
        let r31 = 2.0*quat.i*quat.k - 2.0*quat.r*quat.j;
        let r32 = 2.0*quat.j*quat.k + 2.0*quat.r*quat.i;
        let r33 = 1.0 - 2.0*quat.i*quat.i - 2.0*quat.j*quat.j;

        let m11 = instance.scale.0 * r11;
        let m12 = instance.scale.0 * r12;
        let m13 = instance.scale.0 * r13;
        let m14 = instance.scale.0 * transform.translation().x as f32;
        let m21 = instance.scale.1 * r21;
        let m22 = instance.scale.1 * r22;
        let m23 = instance.scale.1 * r23;
        let m24 = instance.scale.1 * transform.translation().y as f32;
        let m31 = instance.scale.2 * r31;
        let m32 = instance.scale.2 * r32;
        let m33 = instance.scale.2 * r33;
        let m34 = instance.scale.2 * transform.translation().z as f32;

        let model_matrix: [[f32; 4]; 4] = [
            [m11, m21, m31, 0.0],
            [m12, m22, m32, 0.0],
            [m13, m23, m33, 0.0],
            [m14, m24, m34, 1.0],
        ];

        surface.draw(
            (&instance.polygon_model.vertices, &instance.polygon_model.normals),
            &instance.polygon_model.indices,
            &self.program,
            &uniform! {
                projection_matrix: *env.camera.projection_matrix(),
                view_matrix: env.camera.view_matrix(),
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

        self.instances.insert(instance.id, instance);
    }

    fn generate_new_instance(&mut self, id: mach::ID, shape: &mach::Shape) -> Instance {
        match shape.spec() {
            mach::ShapeSpec::Cuboid(x, y, z) => {
                self.factory.generate(id, (x as f32, y as f32, z as f32), self.cube.clone())
            },

            _ => unimplemented!(),
        }
    }
}
