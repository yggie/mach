pub struct CameraLens {
    far: f32,
    near: f32,
    aspect_ratio: f32,
    field_of_view: f32,
    projection_matrix: [[f32; 4]; 4],
}

impl CameraLens {
    pub fn new(width: u32, height: u32) -> CameraLens {
        let mut lens = CameraLens {
            far: 128.0,
            near: 0.1,
            aspect_ratio: 4.0 / 3.0,
            field_of_view: 3.141592 / 4.0,
            projection_matrix: [[0.0; 4]; 4],
        };

        lens.set_viewport_dimensions(width, height);

        return lens;
    }

    pub fn set_viewport_dimensions(&mut self, width: u32, height: u32) {
        self.set_aspect_ratio(width as f32 / height as f32);
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.projection_matrix = CameraLens::compute_projection_matrix(self.near, self.far, self.field_of_view, self.aspect_ratio);
    }

    pub fn projection_matrix(&self) -> &[[f32; 4]; 4] {
        &self.projection_matrix
    }

    pub fn update(&mut self) {
        // no-op
    }

    fn compute_projection_matrix(near: f32, far: f32, field_of_view: f32, aspect_ratio: f32) -> [[f32; 4]; 4] {
        let fov = field_of_view;
        let a = 1.0 / fov.tan();
        let dist = far - near;

        [
            [a / aspect_ratio, 0.0,                          0.0,  0.0],
            [             0.0,   a,                          0.0,  0.0],
            [             0.0, 0.0,     -(far + near) / dist, -1.0],
            [             0.0, 0.0, -2.0 * far * near / dist,  0.0],
        ]
    }
}
