#[cfg(test)]
#[path="../../../tests/collisions/collisionobjectspace/mach_collision_object_space_test.rs"]
mod tests;

use ID;
use utils::Ref;
use collisions::{Body, BodyDef, BodyHandle, CollisionGroup, CollisionObjectSpace, Narrowphase};

pub struct MachCollisionObjectSpace<D, N> where N: Narrowphase {
    foreground_bodies: Vec<BodyHandle<D, N>>,
    background_bodies: Vec<BodyHandle<D, N>>,
}

impl<D, N> MachCollisionObjectSpace<D, N> where N: Narrowphase {
    pub fn new() -> MachCollisionObjectSpace<D, N> {
        MachCollisionObjectSpace {
            foreground_bodies: Vec::new(),
            background_bodies: Vec::new(),
        }
    }

    fn gen_id(&self) -> ID {
        ID((self.foreground_bodies.len() + self.background_bodies.len()) as u32)
    }
}

impl<D, N> CollisionObjectSpace<D, N> for MachCollisionObjectSpace<D, N> where N: Narrowphase {
    fn find(&self, id: ID) -> Option<BodyHandle<D, N>> {
        self.foreground_bodies.iter().find(|handle| handle.borrow().id() == id)
            .or_else(|| self.background_bodies.iter().find(|handle| handle.borrow().id() == id))
            .cloned()
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<D, N>>> + 'a> {
        let iterator = self.foreground_bodies_iter()
            .chain(self.background_bodies_iter());

        return Box::new(iterator);
    }

    fn foreground_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<D, N>>> + 'a> {
        Box::new(self.foreground_bodies.iter().map(|handle| handle.borrow()))
    }

    fn foreground_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&BodyHandle<D, N>> + 'a> {
        Box::new(self.foreground_bodies.iter())
    }

    fn background_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<D, N>>> + 'a> {
        Box::new(self.background_bodies.iter().map(|handle| handle.borrow()))
    }

    fn background_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&BodyHandle<D, N>> + 'a> {
        Box::new(self.background_bodies.iter())
    }

    fn create_body(&mut self, def: BodyDef<D>) -> BodyHandle<D, N> {
        let group = def.group;
        let body = Body::new(self.gen_id(), def);
        let handle = BodyHandle::new(body);

        match group {
            CollisionGroup::Foreground =>
                self.foreground_bodies.push(handle.clone()),

            CollisionGroup::Background =>
                self.background_bodies.push(handle.clone()),
        }

        return handle;
    }
}
