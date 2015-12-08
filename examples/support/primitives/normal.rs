#[derive(Clone, Copy)]
pub struct Normal {
    pub normal: (f32, f32, f32),
}
implement_vertex!(Normal, normal);
