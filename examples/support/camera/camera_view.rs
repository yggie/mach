extern crate nalgebra as na;

use std::mem;
use std::f32::consts::PI;

use self::na::{Dot, Norm};

pub struct CameraViewParams {
    pub up: na::Vector3<f32>,
    pub eye: na::Vector3<f32>,
    pub center: na::Vector3<f32>,
}

pub struct CameraView {
    polar_axis: na::Vector3<f32>,
    zoom_level: f32,
    zenith_axis: na::Vector3<f32>,
    anchor_point: na::Vector3<f32>,
    polar_radians: f32,
    azimuth_radians: f32,
}

impl CameraView {
    pub fn new(params: CameraViewParams) -> CameraView {
        let center_to_eye = params.eye - params.center;
        let direction = center_to_eye.normalize();
        let zenith_axis = -params.up.normalize();
        let polar_axis = na::cross(&direction, &zenith_axis);
        let polar_radians = 0.5 * PI - f32::acos(na::dot(&direction, &zenith_axis));

        return CameraView {
            polar_axis: polar_axis,
            zoom_level: center_to_eye.norm(),
            zenith_axis: zenith_axis,
            anchor_point: na::Vector3::new(0.0, 0.0, 0.0),
            polar_radians: CameraView::constrain_polar_radians(polar_radians),
            azimuth_radians: 0.0,
        };
    }

    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        let z_axis = -self.camera_direction();
        let x_axis = na::cross(&self.zenith_axis, &z_axis).normalize();
        let y_axis = -na::cross(&z_axis, &x_axis).normalize();
        let camera_position = self.anchor_point + z_axis * self.zoom_level;

        unsafe {
            mem::transmute(na::Matrix4::new(
                x_axis.x, -x_axis.y, x_axis.z, -x_axis.dot(&camera_position),
                y_axis.x, -y_axis.y, y_axis.z, -y_axis.dot(&camera_position),
                z_axis.x, -z_axis.y, z_axis.z, -z_axis.dot(&camera_position),
                     0.0,       0.0,      0.0,                           1.0,
            ))
        }
    }

    pub fn increment_zoom(&mut self, zoom_change: f32) {
        const SCALE_FACTOR: f32 = 10.0;

        self.zoom_level = self.zoom_level * f32::powf(2.0, -zoom_change / SCALE_FACTOR);
    }

    pub fn increment_rotation(&mut self, azimuth_change: f32, polar_change: f32) {
        self.polar_radians = CameraView::constrain_polar_radians(self.polar_radians + polar_change);
        self.azimuth_radians = self.azimuth_radians + azimuth_change;
    }

    pub fn update(&mut self) {
        // no-op
    }

    fn camera_direction(&self) -> na::Vector3<f32> {
        let polar_rotation = na::Rotation3::new(self.polar_axis * (0.5 * PI - self.polar_radians));
        let azimuth_rotation = na::Rotation3::new(self.zenith_axis * self.azimuth_radians);

        return azimuth_rotation * polar_rotation * self.zenith_axis;
    }

    fn constrain_polar_radians(polar_radians: f32) -> f32 {
        const BOUND: f32 = 0.9 * (0.5 * PI);

        if polar_radians < -BOUND {
            -BOUND
        } else if polar_radians > BOUND {
            BOUND
        } else {
            polar_radians
        }
    }
}
