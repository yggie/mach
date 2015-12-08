extern crate glium;

use support::{Normal, Vertex};

pub struct PolygonModel {
    pub vertices: glium::VertexBuffer<Vertex>,
    pub normals: glium::VertexBuffer<Normal>,
    pub indices: glium::IndexBuffer<u16>,
}
