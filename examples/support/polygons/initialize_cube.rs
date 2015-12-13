extern crate glium;

use self::glium::backend::glutin_backend::GlutinFacade;

use support::{Normal, PolygonModel, Vertex};

pub fn initialize_cube(display: &GlutinFacade) -> PolygonModel {
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

    PolygonModel {
        vertices: vertex_buffer,
        normals: normal_buffer,
        indices: indices,
    }
}
