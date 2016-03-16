use Scalar;
use maths::Vect;
use entities::EntityStore;
use detection::ContactEvent;

pub trait World: EntityStore {
    fn update(&mut self, time_step: Scalar) -> Vec<ContactEvent>;
    fn set_gravity(&mut self, gravity: Vect);
}
