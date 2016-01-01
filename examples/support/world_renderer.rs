extern crate mach;

use std::cell::Ref;

pub struct WorldRenderer<S: mach::Space, D: mach::Dynamics>(mach::CustomWorld<S, D>);

impl<S, D> WorldRenderer<S, D>
where S: mach::Space, D: mach::Dynamics {
    pub fn new(world: mach::CustomWorld<S, D>) -> WorldRenderer<S, D> {
        WorldRenderer(world)
    }
}

impl<S, D> mach::World for WorldRenderer<S, D>
where S: mach::Space, D: mach::Dynamics {
    #[inline(always)]
    fn create_rigid_body(&mut self, params: &mach::entities::BodyParams) -> mach::ID {
        self.0.create_rigid_body(params)
    }

    fn create_static_body(&mut self, params: &mach::entities::BodyParams) -> mach::ID {
        self.0.create_static_body(params)
    }

    #[inline(always)]
    fn find_rigid_body(&self, id: mach::ID) -> Option<Ref<mach::RigidBody>> {
        self.0.find_rigid_body(id)
    }

    #[inline(always)]
    fn rigid_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<mach::RigidBody>> + 'a> {
        self.0.rigid_bodies_iter()
    }

    #[inline(always)]
    fn update(&mut self, time_step: mach::Scalar) -> Option<Vec<mach::detection::Contact>> {
        return self.0.update(time_step);
    }

    #[inline(always)]
    fn gravity(&self) -> mach::maths::Vect {
        self.0.gravity()
    }

    #[inline(always)]
    fn set_gravity(&mut self, gravity: mach::maths::Vect) {
        self.0.set_gravity(gravity);
    }
}
