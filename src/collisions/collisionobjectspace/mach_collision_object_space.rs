#[cfg(test)]
#[path="../../../tests/collisions/collisionobjectspace/mach_collision_object_space_test.rs"]
mod tests;

use ID;
use utils::{Ref, RefMut};
use collisions::{Body, BodyDef, BodyHandle, CollisionGroup, CollisionObjectSpace, Narrowphase};

pub struct MachCollisionObjectSpace<N, T> where N: Narrowphase {
    foreground_bodies: Vec<BodyHandle<N, T>>,
    environment_bodies: Vec<BodyHandle<N, T>>,
}

impl<N, T> MachCollisionObjectSpace<N, T> where N: Narrowphase {
    pub fn new() -> MachCollisionObjectSpace<N, T> {
        MachCollisionObjectSpace {
            foreground_bodies: Vec::new(),
            environment_bodies: Vec::new(),
        }
    }

    fn gen_id(&self) -> ID {
        ID((self.foreground_bodies.len() + self.environment_bodies.len()) as u32)
    }
}

impl<N, T> CollisionObjectSpace<N, T> for MachCollisionObjectSpace<N, T> where N: Narrowphase {
    fn find<'a>(&'a self, id: ID) -> Option<Ref<'a, Body<N, T>>> {
        self.foreground_bodies.iter().find(|handle| handle.borrow().id() == id)
            .or_else(|| self.environment_bodies.iter().find(|handle| handle.borrow().id() == id))
            .map(|handle| handle.borrow())
    }

    fn find_handle(&self, id: ID) -> Option<&BodyHandle<N, T>> {
        self.foreground_bodies.iter().find(|handle| handle.borrow().id() == id)
            .or_else(|| self.environment_bodies.iter().find(|handle| handle.borrow().id() == id))
    }

    fn foreground_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<N, T>>> + 'a> {
        Box::new(self.foreground_bodies.iter().map(|handle| handle.borrow()))
    }

    fn foreground_bodies_mut_iter<'a>(&'a self) -> Box<Iterator<Item=RefMut<Body<N, T>>> + 'a> {
        Box::new(self.foreground_bodies.iter().map(|handle| handle.borrow_mut()))
    }

    fn foreground_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&BodyHandle<N, T>> + 'a> {
        Box::new(self.foreground_bodies.iter())
    }

    fn environment_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<N, T>>> + 'a> {
        Box::new(self.environment_bodies.iter().map(|handle| handle.borrow()))
    }

    fn environment_bodies_mut_iter<'a>(&'a self) -> Box<Iterator<Item=RefMut<Body<N, T>>> + 'a> {
        Box::new(self.environment_bodies.iter().map(|handle| handle.borrow_mut()))
    }

    fn environment_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&BodyHandle<N, T>> + 'a> {
        Box::new(self.environment_bodies.iter())
    }

    fn create_body(&mut self, def: BodyDef, extra: T) -> BodyHandle<N, T> {
        let group = def.group;
        let body = Body::new(self.gen_id(), def, extra);
        let handle = BodyHandle::new(body);

        match group {
            CollisionGroup::Environment =>
                self.environment_bodies.push(handle.clone()),

            _otherwise =>
                self.foreground_bodies.push(handle.clone()),
        }

        return handle;
    }
}
