#[cfg(test)]
#[path="../../../tests/collisions/collisionobjectspace/mach_collision_object_space_test.rs"]
mod tests;

use ID;
use utils::{Handle, Ref, RefMut};
use collisions::{BodyDef, CollisionGroup, CollisionObject, CollisionObjectSpace};

pub struct MachCollisionObjectSpace<O> where O: CollisionObject {
    foreground_bodies: Vec<Handle<O>>,
    environment_bodies: Vec<Handle<O>>,
}

impl<O> MachCollisionObjectSpace<O> where O: CollisionObject {
    pub fn new() -> MachCollisionObjectSpace<O> {
        // TODO let narrowphase_data = N::new(&collision_data); ??

        MachCollisionObjectSpace {
            foreground_bodies: Vec::new(),
            environment_bodies: Vec::new(),
        }
    }

    fn gen_id(&self) -> ID {
        ID((self.foreground_bodies.len() + self.environment_bodies.len()) as u32)
    }
}

impl<O> CollisionObjectSpace<O> for MachCollisionObjectSpace<O> where O: CollisionObject {
    fn find<'a>(&'a self, id: ID) -> Option<Ref<'a, O>> {
        self.foreground_bodies.iter().find(|handle| handle.borrow().id() == id)
            .or_else(|| self.environment_bodies.iter().find(|handle| handle.borrow().id() == id))
            .map(|handle| handle.borrow())
    }

    fn find_handle(&self, id: ID) -> Option<&Handle<O>> {
        self.foreground_bodies.iter().find(|handle| handle.borrow().id() == id)
            .or_else(|| self.environment_bodies.iter().find(|handle| handle.borrow().id() == id))
    }

    fn foreground_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<O>> + 'a> {
        Box::new(self.foreground_bodies.iter().map(|handle| handle.borrow()))
    }

    fn foreground_bodies_mut_iter<'a>(&'a self) -> Box<Iterator<Item=RefMut<O>> + 'a> {
        Box::new(self.foreground_bodies.iter().map(|handle| handle.borrow_mut()))
    }

    fn foreground_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&Handle<O>> + 'a> {
        Box::new(self.foreground_bodies.iter())
    }

    fn environment_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<O>> + 'a> {
        Box::new(self.environment_bodies.iter().map(|handle| handle.borrow()))
    }

    fn environment_bodies_mut_iter<'a>(&'a self) -> Box<Iterator<Item=RefMut<O>> + 'a> {
        Box::new(self.environment_bodies.iter().map(|handle| handle.borrow_mut()))
    }

    fn environment_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&Handle<O>> + 'a> {
        Box::new(self.environment_bodies.iter())
    }

    fn create_body(&mut self, def: BodyDef, extension: O::Extension) -> Handle<O> {
        let group = def.group;
        let body = O::new(self.gen_id(), def, extension);
        let handle = Handle::new(body);

        match group {
            CollisionGroup::Environment =>
                self.environment_bodies.push(handle.clone()),

            _otherwise =>
                self.foreground_bodies.push(handle.clone()),
        }

        return handle;
    }
}
