#[cfg(test)]
#[path="../../../tests/collisions/broadphase/brute_force_broadphase_test.rs"]
mod tests;

use ID;
use utils::Ref;
use collisions::{Body, BodyDef, BodyHandle, Broadphase, CollisionGroup, CloseProximityPair, CollisionObjectSpace, MachCollisionObjectSpace, Narrowphase};

pub struct BruteForceBroadphase<D, N>(MachCollisionObjectSpace<D, N>) where N: Narrowphase;

impl<D, N> BruteForceBroadphase<D, N> where N: Narrowphase {
    pub fn new() -> BruteForceBroadphase<D, N> {
        BruteForceBroadphase(MachCollisionObjectSpace::new())
    }
}

impl<D, N> CollisionObjectSpace<D, N> for BruteForceBroadphase<D, N> where N: Narrowphase {
    fn find(&self, id: ID) -> Option<BodyHandle<D, N>> {
        self.0.find(id)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<D, N>>> + 'a> {
        self.0.bodies_iter()
    }

    fn create_body(&mut self, data: BodyDef<D>) -> BodyHandle<D, N> {
        self.0.create_body(data)
    }

    fn foreground_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<D, N>>> + 'a> {
        self.0.foreground_bodies_iter()
    }

    fn foreground_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&BodyHandle<D, N>> + 'a> {
        self.0.foreground_handles_iter()
    }

    fn background_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<D, N>>> + 'a> {
        self.0.background_bodies_iter()
    }

    fn background_handles_iter<'a>(&'a self) -> Box<Iterator<Item=&BodyHandle<D, N>> + 'a> {
        self.0.background_handles_iter()
    }
}

impl<D, N> Broadphase<D, N> for BruteForceBroadphase<D, N> where D: 'static, N: Narrowphase {
    fn update(&mut self) {
        // do nothing
    }

    fn close_proximity_pairs_iter(&self) -> Box<Iterator<Item=CloseProximityPair<D, N>>> {
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

            for handle_1 in self.0.background_handles_iter() {
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
