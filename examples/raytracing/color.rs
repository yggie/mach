use glium::texture::{ClientFormat, PixelValue};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
        }
    }
}

unsafe impl Send for Color { }

unsafe impl PixelValue for Color {
    fn get_format() -> ClientFormat {
        ClientFormat::F32F32F32
    }
}
