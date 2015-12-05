extern crate nalgebra as na;

use self::na::{Cross, Dot, Norm};

pub struct CameraDef {
    pub up: na::Vec3<f32>,
    pub eye: na::Vec3<f32>,
    pub center: na::Vec3<f32>,

    pub far: f32,
    pub near: f32,
    pub aspect_ratio: f32,
    pub field_of_view: f32,
    pub viewport_width: u32,
    pub viewport_height: u32,
}

impl Default for CameraDef {
    fn default() -> Self {
        CameraDef {
            up: na::Vec3::new(0.0, 1.0, 0.0),
            eye: na::Vec3::new(0.0, -3.0, 8.0),
            center: na::Vec3::new(0.0, 3.0, 0.0),

            far: 128.0,
            near: 0.1,
            field_of_view: 3.141592 / 4.0,
            aspect_ratio: 4.0 / 3.0,
            viewport_width: 640,
            viewport_height: 480,
        }
    }
}

pub struct Camera {
    up: na::Vec3<f32>,
    position: na::Vec3<f32>,
    direction: na::Vec3<f32>,

    far: f32,
    near: f32,
    aspect_ratio: f32,
    field_of_view: f32,
    projection_matrix: [[f32; 4]; 4],

    last_drag_point: Option<(f32, f32)>,
    did_receive_drag_event: bool,
}

impl Camera {
    pub fn new(camera_def: CameraDef) -> Camera {
        let up = camera_def.up;
        let eye = camera_def.eye;
        let center = camera_def.center;

        let eye_to_center = center - eye;

        let new_up = up.normalize();
        let direction = eye_to_center.normalize();
        let x_axis = direction.cross(&new_up).normalize();

        let mut camera = Camera {
            up: new_up,
            position: eye,
            direction: direction,

            far: camera_def.far,
            near: camera_def.near,
            aspect_ratio: camera_def.aspect_ratio,
            field_of_view: camera_def.field_of_view,
            projection_matrix: [[0.0; 4]; 4],

            last_drag_point: None,
            did_receive_drag_event: false,
        };

        camera.set_viewport_dimensions(camera_def.viewport_width, camera_def.viewport_height);

        return camera;
    }

    pub fn view_matrix(&self) -> na::Mat4<f32> {
        let z_axis = -self.direction;
        let x_axis = na::cross(&self.up, &z_axis).normalize();
        let y_axis = -na::cross(&z_axis, &x_axis).normalize();

        na::Mat4::new(
            x_axis.x, -x_axis.y, x_axis.z, -x_axis.dot(&self.position),
            y_axis.x, -y_axis.y, y_axis.z, -y_axis.dot(&self.position),
            z_axis.x, -z_axis.y, z_axis.z, -z_axis.dot(&self.position),
                 0.0,       0.0,      0.0,                        1.0,
        )
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.projection_matrix = Camera::compute_projection_matrix(self.near, self.far, self.field_of_view, self.aspect_ratio);
    }

    pub fn set_viewport_dimensions(&mut self, width: u32, height: u32) {
        self.set_aspect_ratio(width as f32 / height as f32);
    }

    pub fn projection_matrix(&self) -> &[[f32; 4]; 4] {
        &self.projection_matrix
    }

    pub fn on_mouse_move(&mut self, x: f32, y: f32) {
        match self.last_drag_point {
            Some((prev_x, prev_y)) => {
                let (dx, dy) = (x - prev_x, y - prev_y);
                let rot_mag = f32::sqrt(dx*dx + dy*dy) / 1000.0;

                let z_view = -self.direction;
                let x_view = na::cross(&self.up, &z_view).normalize();
                let y_view = -na::cross(&z_view, &x_view).normalize();

                let axis = (x_view * -dy + y_view * -dx).normalize();

                self.direction = -(na::Rot3::new(axis * rot_mag) * z_view).normalize();
                self.up = -y_view;
            },

            None => (),
        }

        self.did_receive_drag_event = true;
        self.last_drag_point = Some((x, y));
    }

    pub fn update(&mut self) {
        if !self.did_receive_drag_event {
            self.last_drag_point = None;
        }

        self.did_receive_drag_event = false;
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

