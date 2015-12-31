use maths::Vect;

#[derive(Clone, Debug)]
pub struct Motion {
    pub velocity: Vect,
    pub angular_velocity: Vect,
}

impl Motion {
    pub fn new(velocity: Vect, angular_velocity: Vect) -> Motion {
        Motion {
            velocity: velocity,
            angular_velocity: angular_velocity,
        }
    }
}
