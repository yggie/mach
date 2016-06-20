#[cfg(test)]
#[path="../../../tests/collisions/collisionobjectspace/mach_collision_object_space_test.rs"]
mod tests;

use ID;
use collisions::{CollisionData, CollisionDataHandle, CollisionObject, CollisionObjectSpace, NarrowphaseData};

pub struct MachCollisionObjectSpace<T> where T: NarrowphaseData {
    foreground_objects: Vec<CollisionObject<T>>,
    background_objects: Vec<CollisionObject<T>>,
}

impl<T> MachCollisionObjectSpace<T> where T: NarrowphaseData {
    pub fn new() -> MachCollisionObjectSpace<T> {
        MachCollisionObjectSpace {
            foreground_objects: Vec::new(),
            background_objects: Vec::new(),
        }
    }

    fn gen_id(&self) -> ID {
        ID((self.foreground_objects.len() + self.background_objects.len()) as u32)
    }
}

impl<T> CollisionObjectSpace<T> for MachCollisionObjectSpace<T> where T: NarrowphaseData {
    fn find(&self, id: ID) -> Option<CollisionObject<T>> {
        self.foreground_objects.iter().find(|obj| obj.id == id)
            .or_else(|| self.background_objects.iter().find(|obj| obj.id == id))
            .cloned()
    }

    fn objects_iter<'a>(&'a self) -> Box<Iterator<Item=CollisionObject<T>> + 'a> {
        let iterator = self.foreground_objects_iter()
            .chain(self.background_objects_iter());

        return Box::new(iterator);
    }

    fn foreground_objects_iter<'a>(&'a self) -> Box<Iterator<Item=CollisionObject<T>> + 'a> {
        Box::new(self.foreground_objects.iter().cloned())
    }

    fn background_objects_iter<'a>(&'a self) -> Box<Iterator<Item=CollisionObject<T>> + 'a> {
        Box::new(self.background_objects.iter().cloned())
    }

    fn create_foreground_object(&mut self, data: CollisionData<T>) -> CollisionObject<T> {
        let object = CollisionObject {
            id: self.gen_id(),
            is_background: false,
            data: CollisionDataHandle::new(data),
        };
        self.foreground_objects.push(object.clone());

        return object;
    }

    fn create_background_object(&mut self, data: CollisionData<T>) -> CollisionObject<T> {
        let object = CollisionObject {
            id: self.gen_id(),
            is_background: true,
            data: CollisionDataHandle::new(data),
        };
        self.background_objects.push(object.clone());

        return object;
    }
}
