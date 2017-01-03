#[cfg(test)]
#[path="../../../tests/collisions/broadphase/brute_force_broadphase_test.rs"]
mod tests;

use ID;
use utils::{Handle, Ref, RefMut};
use collisions::{BodyDef, Broadphase, CollisionGroup, CloseProximityPair, CollisionObject, CollisionObjectSpace, MachCollisionObjectSpace, Narrowphase};
use collisions::shapes::Ray;

pub struct BruteForceBroadphase<O>(MachCollisionObjectSpace<O>) where O: CollisionObject;

impl<O> BruteForceBroadphase<O> where O: CollisionObject {
    pub fn new() -> BruteForceBroadphase<O> {
        BruteForceBroadphase(MachCollisionObjectSpace::new())
    }
}

impl<O> CollisionObjectSpace<O> for BruteForceBroadphase<O> where O: CollisionObject {
    fn find<'a>(&'a self, id: ID) -> Option<Ref<'a, O>> {
        self.0.find(id)
    }

    fn find_handle(&self, id: ID) -> Option<&Handle<O>> {
        self.0.find_handle(id)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<O>> + 'a> {
        self.0.bodies_iter()
    }

    fn create_body(&mut self, def: BodyDef, extension: O::Extension) -> Handle<O> {
        self.0.create_body(def, extension)
    }

    fn foreground_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<O>> + 'a> {
        self.0.foreground_bodies_iter()
    }

    fn foreground_bodies_mut_iter<'a>(&'a self) -> Box<Iterator<Item=RefMut<O>> + 'a> {
        self.0.foreground_bodies_mut_iter()
    }

    fn foreground_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&Handle<O>> + 'a> {
        self.0.foreground_handles_iter()
    }

    fn environment_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<O>> + 'a> {
        self.0.environment_bodies_iter()
    }

    fn environment_bodies_mut_iter<'a>(&'a self) -> Box<Iterator<Item=RefMut<O>> + 'a> {
        self.0.environment_bodies_mut_iter()
    }

    fn environment_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&Handle<O>> + 'a> {
        self.0.environment_handles_iter()
    }
}

impl<O> Broadphase<O> for BruteForceBroadphase<O> where O: CollisionObject {
    fn update(&mut self) {
        // do nothing
    }

    fn close_proximity_pairs_iter(&self) -> Box<Iterator<Item=CloseProximityPair<O>>> {
        let mut pairs = Vec::new();

        for (index, handle_0) in self.0.foreground_handles_iter().enumerate() {
            let body_0 = handle_0.borrow();

            for handle_1 in self.0.foreground_handles_iter().skip(index + 1) {
                let body_1 = handle_0.borrow();

                if CollisionGroup::test(body_0.group(), body_1.group()) && O::Narrowphase::test(body_0.narrowphase_ref(), body_1.narrowphase_ref()) {
                    let pair = CloseProximityPair(handle_0.clone(), handle_1.clone());
                    pairs.push(pair);
                }
            }
        }

        for handle_0 in self.0.foreground_handles_iter() {
            let body_0 = handle_0.borrow();

            for handle_1 in self.0.environment_handles_iter() {
                let body_1 = handle_0.borrow();

                if CollisionGroup::test(body_0.group(), body_1.group()) && O::Narrowphase::test(body_0.narrowphase_ref(), body_1.narrowphase_ref()) {
                    let pair = CloseProximityPair(handle_0.clone(), handle_1.clone());
                    pairs.push(pair);
                }
            }
        }

        return Box::new(pairs.into_iter());
    }

    fn cast_ray<'a>(&'a self, ray: &Ray) -> Box<Iterator<Item=Ref<O>> + 'a> {
        self.bodies_iter()
    }
}
