use Scalar;
use maths::Vec3D;
use entities::EntityStore;
use detection::ContactEvent;

pub trait World: EntityStore {
    fn update(&mut self, time_step: Scalar) -> Vec<ContactEvent>;
    fn set_gravity(&mut self, gravity: Vec3D);
}
