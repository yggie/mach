#[cfg(test)]
#[path="../../../tests/collisions/broadphase/brute_force_broadphase_test.rs"]
mod tests;

use ID;
use utils::{Ref, RefMut};
use collisions::{Body, BodyDef, BodyHandle, Broadphase, CollisionGroup, CloseProximityPair, CollisionObjectSpace, MachCollisionObjectSpace, Narrowphase};

pub struct BruteForceBroadphase<N, T>(MachCollisionObjectSpace<N, T>) where N: Narrowphase;

impl<N, T> BruteForceBroadphase<N, T> where N: Narrowphase {
    pub fn new() -> BruteForceBroadphase<N, T> {
        BruteForceBroadphase(MachCollisionObjectSpace::new())
    }
}

impl<N, T> CollisionObjectSpace<N, T> for BruteForceBroadphase<N, T> where N: Narrowphase {
    fn find<'a>(&'a self, id: ID) -> Option<Ref<'a, Body<N, T>>> {
        self.0.find(id)
    }

    fn find_handle(&self, id: ID) -> Option<&BodyHandle<N, T>> {
        self.0.find_handle(id)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<N, T>>> + 'a> {
        self.0.bodies_iter()
    }

    fn create_body(&mut self, data: BodyDef<T>) -> BodyHandle<N, T> {
        self.0.create_body(data)
    }

    fn foreground_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<N, T>>> + 'a> {
        self.0.foreground_bodies_iter()
    }

    fn foreground_bodies_mut_iter<'a>(&'a self) -> Box<Iterator<Item=RefMut<Body<N, T>>> + 'a> {
        self.0.foreground_bodies_mut_iter()
    }

    fn foreground_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&BodyHandle<N, T>> + 'a> {
        self.0.foreground_handles_iter()
    }

    fn environment_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<N, T>>> + 'a> {
        self.0.environment_bodies_iter()
    }

    fn environment_bodies_mut_iter<'a>(&'a self) -> Box<Iterator<Item=RefMut<Body<N, T>>> + 'a> {
        self.0.environment_bodies_mut_iter()
    }

    fn environment_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&BodyHandle<N, T>> + 'a> {
        self.0.environment_handles_iter()
    }
}

impl<N, T> Broadphase<N, T> for BruteForceBroadphase<N, T> where T: 'static, N: Narrowphase {
    fn update(&mut self) {
        // do nothing
    }

    fn close_proximity_pairs_iter(&self) -> Box<Iterator<Item=CloseProximityPair<N, T>>> {
        let mut pairs = Vec::new();

        for (index, handle_0) in self.0.foreground_handles_iter().enumerate() {
            let body_0 = handle_0.borrow();

            for handle_1 in self.0.foreground_handles_iter().skip(index + 1) {
                let body_1 = handle_0.borrow();

                if CollisionGroup::test(body_0.group(), body_1.group()) && N::test(&body_0, &body_1) {
                    let pair = CloseProximityPair(handle_0.clone(), handle_1.clone());
                    pairs.push(pair);
                }
            }
        }

        for handle_0 in self.0.foreground_handles_iter() {
            let body_0 = handle_0.borrow();

            for handle_1 in self.0.environment_handles_iter() {
                let body_1 = handle_0.borrow();

                if CollisionGroup::test(body_0.group(), body_1.group()) && N::test(&body_0, &body_1) {
                    let pair = CloseProximityPair(handle_0.clone(), handle_1.clone());
                    pairs.push(pair);
                }
            }
        }

        return Box::new(pairs.into_iter());
    }
}
