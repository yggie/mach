use std::marker::PhantomData;

use {Scalar, World};
use maths::Vec3D;
use utils::{Ref, RefMut};
use dynamics::{ConstraintSolver, DynamicBody, DynamicBodyHandle, DynamicBodyType, FixedBodyData, FixedBodyDef, Integrator, RigidBodyData, RigidBodyDef, RigidBodyRefMut};
use collisions::{BodyDef, Broadphase, CollisionGroup, CollisionObjectSpace, Contact, Detection, Narrowphase};

pub struct CustomWorld<B, C, D, I, N, T> where
        C: ConstraintSolver<I, N, T>,
        B: Broadphase<N, DynamicBodyType<T>>,
        D: Detection<N, DynamicBodyType<T>>,
        I: Integrator,
        N: Narrowphase {

    gravity: Vec3D,
    detection: D,
    integrator: I,
    broadphase: B,
    constraint_solver: C,
    _extra: PhantomData<T>,
    _narrowphase: PhantomData<N>,
}

impl<B, C, D, I, N, T> CustomWorld<B, C, D, I, N, T> where
        C: ConstraintSolver<I, N, T>,
        B: Broadphase<N, DynamicBodyType<T>>,
        D: Detection<N, DynamicBodyType<T>>,
        I: Integrator,
        N: Narrowphase,
        T: 'static {

    pub fn new(detection: D, integrator: I, broadphase: B, constraint_solver: C, gravity: Vec3D) -> CustomWorld<B, C, D, I, N, T> {
        CustomWorld {
            gravity: gravity,
            detection: detection,
            integrator: integrator,
            broadphase: broadphase,
            constraint_solver: constraint_solver,
            _extra: PhantomData,
            _narrowphase: PhantomData,
        }
    }

    pub fn update(&mut self, time_step: Scalar) -> Vec<Contact<N, DynamicBodyType<T>>> {
        for mut body in self.broadphase.bodies_iter_mut() {
            if let Some(mut rigid_body) = RigidBodyRefMut::try_from(&mut body) {
                self.integrator.integrate_in_place(&mut rigid_body.integratable(), time_step, self.gravity);
            }

            // TODO does this need to be handled by the Broadphase?
            // TODO only update if necessary?
            N::update(body.data_mut());
        }

        self.broadphase.update();
        self.detection.update();

        let contacts: Vec<Contact<N, DynamicBodyType<T>>> = self.broadphase.close_proximity_pairs_iter()
            .filter_map(|pair| self.detection.compute_contacts(&pair.0, &pair.1))
            .collect();

        if contacts.len() > 0 {
            self.constraint_solver.solve_with_contacts(&contacts, &self.integrator, time_step);

            for mut body in self.broadphase.bodies_iter_mut() {
                // TODO does this need to be handled by the Broadphase?
                // TODO only update if necessary?
                N::update(body.data_mut());
            }
            self.broadphase.update();
        }

        return contacts;
    }

    pub fn rigid_bodies_iter_mut<'a>(&'a self) -> Box<Iterator<Item=RefMut<DynamicBody<N, T>>> + 'a> {
        let iterator = self.broadphase.bodies_iter_mut()
            .filter(|body| {
                match body.extra_data() {
                    &DynamicBodyType::Rigid(_) => true,

                    _otherwise => false,
                }
            });

        return Box::new(iterator);
    }
}

impl<B, C, D, I, N, T> World<N, T> for CustomWorld<B, C, D, I, N, T> where
        C: ConstraintSolver<I, N, T>,
        B: Broadphase<N, DynamicBodyType<T>>,
        D: Detection<N, DynamicBodyType<T>>,
        I: Integrator,
        N: Narrowphase,
        T: 'static {

    fn update(&mut self, time_step: Scalar) -> Vec<Contact<N, DynamicBodyType<T>>> {
        CustomWorld::update(self, time_step)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<DynamicBody<N, T>>> + 'a> {
        self.broadphase.bodies_iter()
    }

    fn set_gravity(&mut self, gravity: Vec3D) {
        self.gravity = gravity;
    }

    fn create_rigid_body(&mut self, def: RigidBodyDef, extra: T) -> DynamicBodyHandle<N, T> {
        let rigid_body_data = RigidBodyData::new(&def, extra);

        self.broadphase.create_body(BodyDef {
            group: def.group,
            shape: def.shape,
            rotation: def.rotation,
            translation: def.translation,
        }, DynamicBodyType::Rigid(Box::new(rigid_body_data)))
    }

    fn create_fixed_body(&mut self, def: FixedBodyDef, extra: T) -> DynamicBodyHandle<N, T> {
        let fixed_body_data = FixedBodyData::new(&def, extra);

        self.broadphase.create_body(BodyDef {
            group: CollisionGroup::Environment,
            shape: def.shape,
            rotation: def.rotation,
            translation: def.translation,
        }, DynamicBodyType::Fixed(Box::new(fixed_body_data)))
    }
}
