#[cfg(test)]
#[path="../../../tests/collisions/broadphase/brute_force_broadphase_test.rs"]
mod tests;

use ID;
use collisions::{Broadphase, CollisionData, CollisionObject, CollisionObjectSpace, MachCollisionObjectSpace, NarrowphaseData};

pub struct BruteForceBroadphase<T>(MachCollisionObjectSpace<T>) where T: NarrowphaseData;

impl<T> BruteForceBroadphase<T> where T: NarrowphaseData {
    pub fn new() -> BruteForceBroadphase<T> {
        BruteForceBroadphase(MachCollisionObjectSpace::new())
    }
}

impl<T> CollisionObjectSpace<T> for BruteForceBroadphase<T> where T: NarrowphaseData {
    fn find(&self, id: ID) -> Option<CollisionObject<T>> {
        self.0.find(id)
    }

    fn objects_iter<'a>(&'a self) -> Box<Iterator<Item=CollisionObject<T>> + 'a> {
        self.0.objects_iter()
    }

    fn foreground_objects_iter<'a>(&'a self) -> Box<Iterator<Item=CollisionObject<T>> + 'a> {
        self.0.foreground_objects_iter()
    }

    fn background_objects_iter<'a>(&'a self) -> Box<Iterator<Item=CollisionObject<T>> + 'a> {
        self.0.background_objects_iter()
    }

    fn create_foreground_object(&mut self, data: CollisionData<T>) -> CollisionObject<T> {
        self.0.create_foreground_object(data)
    }

    fn create_background_object(&mut self, data: CollisionData<T>) -> CollisionObject<T> {
        self.0.create_background_object(data)
    }
}

impl<T> Broadphase<T> for BruteForceBroadphase<T> where T: NarrowphaseData {
    fn update(&mut self) {
        // do nothing
    }

    fn possible_collision_pairs_iter(&self) -> Box<Iterator<Item=(CollisionObject<T>, CollisionObject<T>)>> {
        let mut pairs = Vec::new();

        for (index, object_0) in self.0.foreground_objects_iter().enumerate() {
            for object_1 in self.0.foreground_objects_iter().skip(index + 1) {
                pairs.push((object_0.clone(), object_1));
            }
        }

        for object_0 in self.0.foreground_objects_iter() {
            for object_1 in self.0.background_objects_iter() {
                pairs.push((object_0.clone(), object_1));
            }
        }

        return Box::new(pairs.into_iter());
    }
}
