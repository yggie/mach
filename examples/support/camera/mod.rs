extern crate glium;
extern crate nalgebra as na;

mod camera_lens;
mod camera_view;

pub use self::camera_lens::CameraLens;
pub use self::camera_view::{CameraView, CameraViewParams};

use self::glium::glutin;

use support::{EventResponse, WindowEventHandler};

pub struct Camera {
    lens: CameraLens,
    view: CameraView,

    last_drag_point: Option<(f32, f32)>,
    did_receive_drag_event: bool,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Camera {
        Camera {
            lens: CameraLens::new(width, height),
            view: CameraView::new(CameraViewParams {
                up: na::Vec3::new(0.0, 1.0, 0.0),
                eye: na::Vec3::new(0.0, 0.0, 2.5),
                center: na::Vec3::new(0.0, 0.0, 0.0),
            }),
            last_drag_point: None,
            did_receive_drag_event: false,
        }
    }

    #[inline]
    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        self.view.view_matrix()
    }

    pub fn projection_matrix(&self) -> &[[f32; 4]; 4] {
        self.lens.projection_matrix()
    }

    fn on_mouse_wheel_line(&mut self, _x: f32, _y: f32) {
        unimplemented!();
    }

    fn on_mouse_wheel_pixel(&mut self, _x: f32, y: f32) {
        self.view.increment_zoom(y);
    }

    fn on_mouse_move(&mut self, x: f32, y: f32) {
        match self.last_drag_point {
            Some((prev_x, prev_y)) => {
                const SCALE_FACTOR: f32 = 0.015;

                let dx = (x - prev_x) * SCALE_FACTOR;
                let dy = (y - prev_y) * SCALE_FACTOR;

                self.view.increment_rotation(dx, -dy);
            },

            None => (),
        }

        self.did_receive_drag_event = true;
        self.last_drag_point = Some((x, y));
    }

    pub fn update(&mut self) {
        self.view.update();
        self.lens.update();

        if !self.did_receive_drag_event {
            self.last_drag_point = None;
        }

        self.did_receive_drag_event = false;
    }
}

impl WindowEventHandler for Camera {
    fn handle_event(&mut self, event: &glutin::Event) -> EventResponse {
        match event {
            &glutin::Event::MouseMoved((x, y)) => {
                self.on_mouse_move(x as f32, y as f32);
            },

            &glutin::Event::MouseWheel(glutin::MouseScrollDelta::LineDelta(x, y)) => {
                self.on_mouse_wheel_line(x, y);
            },

            &glutin::Event::MouseWheel(glutin::MouseScrollDelta::PixelDelta(x, y)) => {
                self.on_mouse_wheel_pixel(x, y);
            },

            _otherwise => return EventResponse::Bubble,
        }

        return EventResponse::Consumed;
    }
}
