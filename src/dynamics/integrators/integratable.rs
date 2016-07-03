use maths::{Motion, UnitQuat, Transform, Vec3D};

pub struct Integratable<'a> {
    transform: &'a mut Transform,
    motion: &'a mut Motion,
}

impl<'a> Integratable<'a> {
    pub fn new(transform: &'a mut Transform, motion: &'a mut Motion) -> Integratable<'a> {
        Integratable {
            transform: transform,
            motion: motion,
        }
    }

    #[inline]
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    #[inline]
    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn translation(&self) -> &Vec3D {
        &self.transform().translation
    }

    pub fn translation_mut(&mut self) -> &mut Vec3D {
        &mut self.transform_mut().translation
    }

    pub fn rotation(&self) -> UnitQuat {
        self.transform().rotation
    }

    pub fn rotation_mut(&mut self) -> &mut UnitQuat {
        &mut self.transform_mut().rotation
    }

    pub fn motion(&self) -> &Motion {
        &self.motion
    }

    pub fn motion_mut(&mut self) -> &mut Motion {
        &mut self.motion
    }

    pub fn velocity(&self) -> &Vec3D {
        &self.motion().velocity
    }

    pub fn velocity_mut(&mut self) -> &mut Vec3D {
        &mut self.motion_mut().velocity
    }

    pub fn angular_velocity(&self) -> &Vec3D {
        &self.motion().angular_velocity
    }
}
