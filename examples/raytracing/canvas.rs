use mach::{Scalar, UnitVec3D, Vec3D};
use mach::maths::CrossProduct;

struct Camera {
    eye: Vec3D,
    center: Vec3D,
    x_unit: Vec3D,
    y_unit: Vec3D,
}

pub struct Canvas {
    size: (usize, usize),
    camera: Camera,
}

impl Canvas {
    pub fn set_size(&mut self, width: usize, height: usize) {
        self.size = (width, height);

        self.camera.x_unit = self.camera.x_unit.normalize() * self.camera.y_unit.length() * self.aspect_ratio();
    }

    pub fn set_camera_params(&mut self, eye: Vec3D, center: Vec3D, up: Vec3D, field_of_view: Scalar) {
        self.camera.eye = eye;
        self.camera.center = center;

        let z_axis = eye - center;
        let x_axis = -z_axis.cross(up).normalize();
        let y_axis = z_axis.cross(x_axis).normalize();

        self.camera.y_unit = y_axis * (field_of_view / 2.0).tan() * z_axis.length();
        self.camera.x_unit = x_axis * self.camera.y_unit.length() * self.aspect_ratio();
    }

    pub fn width(&self) -> usize {
        self.size.0
    }

    pub fn height(&self) -> usize {
        self.size.1
    }

    pub fn aspect_ratio(&self) -> Scalar {
        self.size.0 as Scalar / self.size.1 as Scalar
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> (Vec3D, UnitVec3D) {
        let x =  self.camera.x_unit * (x as Scalar - self.size.0 as Scalar / 2.0);
        let y = -self.camera.y_unit * (y as Scalar + self.size.1 as Scalar / 2.0);

        let direction = (x + y + self.camera.center - self.camera.eye).normalize();

        return (self.camera.eye, direction);
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Canvas {
            size: (320, 240),
            camera: Camera {
                eye: Vec3D::zero(),
                center: Vec3D::new(0.0, 0.0, 10.0),
                x_unit: Vec3D::new(1.0, 0.0, 0.0),
                y_unit: Vec3D::new(0.0, 1.0, 0.0),
            },
        }
    }
}
