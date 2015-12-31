use maths::{Motion, Transform, Quat, Vect};

pub trait Moveable {
    fn transform(&self) -> &Transform;
    fn transform_mut(&mut self) -> &mut Transform;
    fn motion(&self) -> &Motion;
    fn motion_mut(&mut self) -> &mut Motion;

    fn translation(&self) -> &Vect {
        &self.transform().translation
    }

    fn translation_mut(&mut self) -> &mut Vect {
        &mut self.transform_mut().translation
    }

    fn rotation(&self) -> &Quat {
        &self.transform().rotation
    }

    fn rotation_mut(&mut self) -> &mut Quat {
        &mut self.transform_mut().rotation
    }

    fn velocity(&self) -> &Vect {
        &self.motion().velocity
    }

    fn velocity_mut(&mut self) -> &mut Vect {
        &mut self.motion_mut().velocity
    }

    fn angular_velocity(&self) -> &Vect {
        &self.motion().angular_velocity
    }
}
