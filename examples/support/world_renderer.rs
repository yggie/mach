extern crate mach;

use std::cell::Ref;

pub struct WorldRenderer<C: mach::CollisionSpace, D: mach::Dynamics>(mach::CustomWorld<C, D>);

impl<C, D> WorldRenderer<C, D>
where C: mach::CollisionSpace, D: mach::Dynamics {
    pub fn new(world: mach::CustomWorld<C, D>) -> WorldRenderer<C, D> {
        WorldRenderer(world)
    }
}

impl<C, D> mach::World for WorldRenderer<C, D>
where C: mach::CollisionSpace, D: mach::Dynamics {
    #[inline(always)]
    fn create_body(&mut self, entity_desc: &mach::EntityDesc) -> mach::ID {
        self.0.create_body(entity_desc)
    }

    fn create_static_body(&mut self, entity_desc: &mach::EntityDesc) -> mach::ID {
        self.0.create_static_body(entity_desc)
    }

    #[inline(always)]
    fn find_body(&self, id: mach::ID) -> Option<Ref<mach::RigidBody>> {
        self.0.find_body(id)
    }

    #[inline(always)]
    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<mach::RigidBody>> + 'a> {
        self.0.bodies_iter()
    }

    #[inline(always)]
    fn update(&mut self, time_step: mach::Scalar) {
        self.0.update(time_step);
    }

    #[inline(always)]
    fn gravity(&self) -> mach::Vector {
        self.0.gravity()
    }

    #[inline(always)]
    fn set_gravity(&mut self, gravity: mach::Vector) {
        self.0.set_gravity(gravity);
    }
}
