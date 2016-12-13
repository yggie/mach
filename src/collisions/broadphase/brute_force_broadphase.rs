#[cfg(test)]
#[path="../../../tests/collisions/broadphase/brute_force_broadphase_test.rs"]
mod tests;

use ID;
use utils::{Handle, Ref, RefMut};
use collisions::{BodyDef, Broadphase, CollisionBody, CollisionGroup, CloseProximityPair, CollisionObjectSpace, MachCollisionObjectSpace, Narrowphase};
use collisions::geometry::Ray;

pub struct BruteForceBroadphase<B>(MachCollisionObjectSpace<B>) where B: CollisionBody;

impl<B> BruteForceBroadphase<B> where B: CollisionBody {
    pub fn new() -> BruteForceBroadphase<B> {
        BruteForceBroadphase(MachCollisionObjectSpace::new())
    }
}

impl<B> CollisionObjectSpace<B> for BruteForceBroadphase<B> where B: CollisionBody {
    fn find<'a>(&'a self, id: ID) -> Option<Ref<'a, B>> {
        self.0.find(id)
    }

    fn find_handle(&self, id: ID) -> Option<&Handle<B>> {
        self.0.find_handle(id)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<B>> + 'a> {
        self.0.bodies_iter()
    }

    fn create_body(&mut self, def: BodyDef, extension: B::Extension) -> Handle<B> {
        self.0.create_body(def, extension)
    }

    fn foreground_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<B>> + 'a> {
        self.0.foreground_bodies_iter()
    }

    fn foreground_bodies_mut_iter<'a>(&'a self) -> Box<Iterator<Item=RefMut<B>> + 'a> {
        self.0.foreground_bodies_mut_iter()
    }

    fn foreground_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&Handle<B>> + 'a> {
        self.0.foreground_handles_iter()
    }

    fn environment_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<B>> + 'a> {
        self.0.environment_bodies_iter()
    }

    fn environment_bodies_mut_iter<'a>(&'a self) -> Box<Iterator<Item=RefMut<B>> + 'a> {
        self.0.environment_bodies_mut_iter()
    }

    fn environment_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&Handle<B>> + 'a> {
        self.0.environment_handles_iter()
    }
}

impl<B> Broadphase<B> for BruteForceBroadphase<B> where B: CollisionBody {
    fn update(&mut self) {
        // do nothing
    }

    fn close_proximity_pairs_iter(&self) -> Box<Iterator<Item=CloseProximityPair<B>>> {
        let mut pairs = Vec::new();

        for (index, handle_0) in self.0.foreground_handles_iter().enumerate() {
            let body_0 = handle_0.borrow();

            for handle_1 in self.0.foreground_handles_iter().skip(index + 1) {
                let body_1 = handle_0.borrow();

                if CollisionGroup::test(body_0.group(), body_1.group()) && B::Narrowphase::test(body_0.narrowphase_ref(), body_1.narrowphase_ref()) {
                    let pair = CloseProximityPair(handle_0.clone(), handle_1.clone());
                    pairs.push(pair);
                }
            }
        }

        for handle_0 in self.0.foreground_handles_iter() {
            let body_0 = handle_0.borrow();

            for handle_1 in self.0.environment_handles_iter() {
                let body_1 = handle_0.borrow();

                if CollisionGroup::test(body_0.group(), body_1.group()) && B::Narrowphase::test(body_0.narrowphase_ref(), body_1.narrowphase_ref()) {
                    let pair = CloseProximityPair(handle_0.clone(), handle_1.clone());
                    pairs.push(pair);
                }
            }
        }

        return Box::new(pairs.into_iter());
    }

    fn cast_ray<'a>(&'a self, ray: Ray) -> Box<Iterator<Item=Ref<B>> + 'a> {
        self.bodies_iter()
    }
}
