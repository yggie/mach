use std;

use time;
use glium;
use glium::{glutin, DisplayBuild, Surface};
use glium::index::PrimitiveType;

use raytracing::{RayTracer, Renderer};

pub fn render<T>(mut renderer: Renderer<T>) where T: RayTracer {
    let display = glutin::WindowBuilder::new()
        .with_dimensions(renderer.canvas().width() as u32, renderer.canvas().height() as u32)
        .with_vsync()
        .build_glium()
        .unwrap();

    // prepare a vertex buffer containing vertices for a Quad
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
        }

        implement_vertex!(Vertex, position, tex_coords);

        glium::VertexBuffer::new(
            &display,
            &[
                Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
                Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
                Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] }
            ],
        ).unwrap()
    };

    // prepare an index buffer to describe the Quad
    let index_buffer = glium::IndexBuffer::new(
        &display,
        PrimitiveType::TriangleStrip,
        &[1 as u16, 2, 0, 3],
    ).unwrap();

    let opengl_texture = glium::texture::CompressedSrgbTexture2d::new(
        &display,
        renderer.image_buffer().clone(),
    ).unwrap();

    // basic shader suitable to draw textured polygons
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 matrix;
                in vec2 position;
                in vec2 tex_coords;
                out vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 140
                uniform sampler2D tex;
                in vec2 v_tex_coords;
                out vec4 f_color;
                void main() {
                    f_color = texture(tex, v_tex_coords);
                }
            "
        },
    ).unwrap();

    let image_rect = glium::Rect {
        left: 0,
        width: renderer.canvas().width() as u32,
        bottom: 0,
        height: renderer.canvas().height() as u32,
    };

    let mut completed = false;
    let global_start_time = time::precise_time_ns();
    let desired_fps = 10;
    let nanoseconds_per_frame = 1_000_000_000 / (desired_fps as u64);
    let nanoseconds_for_ray_tracing = nanoseconds_per_frame / 2;
    loop {
        let start_time = time::precise_time_ns();

        let mut rays_shot = 0;
        while !renderer.is_fully_renderered() &&
            time::precise_time_ns() - start_time < nanoseconds_for_ray_tracing {
            renderer.render_one_more_ray();
            rays_shot += 1;
        }

        opengl_texture.write(image_rect, renderer.image_buffer().clone());

        // orthogonal projection to display the Quad
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            tex: &opengl_texture
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        // draw the Quad using the image as a texture
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed | glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Escape)) => return,
                _otherwise => (),
            }
        }

        let time_taken = time::precise_time_ns() - start_time;
        if time_taken < nanoseconds_per_frame {
            std::thread::sleep(std::time::Duration::new(0, (nanoseconds_per_frame - time_taken) as u32));
        }

        if rays_shot != 0 {
            let time_taken = time::precise_time_ns() - start_time;
            println!("Shot {} rays in {} ms", rays_shot, time_taken / 1_000_000);
        } else if !completed {
            completed = true;
            let global_time_taken = time::precise_time_ns() - global_start_time;
            println!("Completed in {} ms", global_time_taken / 1_000_000);
        }
    }
}
