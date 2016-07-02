use ID;
use utils::Ref;
use physics::{Body, FixedBody, FixedBodyData, FixedBodyHandle, PhysicsObjectSpace, RigidBody, RigidBodyData, RigidBodyHandle};
use collisions::NarrowphaseData;

pub struct MachPhysicsObjectSpace<T> where T: NarrowphaseData {
    rigid_bodies: Vec<RigidBodyHandle<T>>,
    fixed_bodies: Vec<FixedBodyHandle<T>>,
}

impl<T> MachPhysicsObjectSpace<T> where T: NarrowphaseData {
    pub fn new() -> MachPhysicsObjectSpace<T> {
        MachPhysicsObjectSpace {
            rigid_bodies: Vec::new(),
            fixed_bodies: Vec::new(),
        }
    }

    fn gen_id(&self) -> ID {
        ID((self.rigid_bodies.len() + self.fixed_bodies.len()) as u32)
    }
}

impl<T> PhysicsObjectSpace<T> for MachPhysicsObjectSpace<T> where T: NarrowphaseData {
    // fn find(&self, id: ID) -> Option<Body<T>> {
    //     self.rigid_bodies.iter().find(|rigid_body| rigid_body.id == id)
    //         .map(Body::from)
    //         .or_else(|| {
    //             self.fixed_bodies.iter()
    //                 .find(|body| body.id == id)
    //                 .map(|| {
    //                 })
    //         })
    //         .cloned()
    // }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<Body<T>>> + 'a> {
        let rigid_body_iterator = self.rigid_bodies.iter()
            .map(|handle| {
                Ref::map(handle.borrow(), |b| b) as Ref<Body<T>>
            });
        let fixed_body_iterator = self.fixed_bodies.iter()
            .map(|handle| {
                Ref::map(handle.borrow(), |b| b) as Ref<Body<T>>
            });
        let iterator = rigid_body_iterator.chain(fixed_body_iterator);

        return Box::new(iterator);
    }

    fn rigid_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<'a, RigidBody<T>>> + 'a> {
        Box::new(self.rigid_bodies.iter().map(|handle| handle.borrow()))
    }

    fn fixed_bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<'a, FixedBody<T>>> + 'a> {
        Box::new(self.fixed_bodies.iter().map(|handle| handle.borrow()))
    }

    fn create_rigid_body(&mut self, data: RigidBodyData<T>) -> RigidBodyHandle<T> {
        let rigid_body = RigidBody::new(self.gen_id(), data);
        let handle = RigidBodyHandle::new(rigid_body);
        self.rigid_bodies.push(handle.clone());

        return handle;
    }

    fn create_fixed_body(&mut self, data: FixedBodyData<T>) -> FixedBodyHandle<T> {
        let fixed_body = FixedBody::new(self.gen_id(), data);
        let handle = FixedBodyHandle::new(fixed_body);
        self.fixed_bodies.push(handle.clone());

        return handle;
    }
}
