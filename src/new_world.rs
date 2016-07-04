use std::marker::PhantomData;

use Scalar;
use maths::Vec3D;
use utils::RefMut;
use dynamics::{ConstraintSolver, DynamicBody, DynamicBodyType, Integrator, RigidBodyRefMut};
use collisions::{Broadphase, CollisionObjectSpace, Contact, Detection, Narrowphase};

pub struct NewWorld<B, C, D, I, N, T> where
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

impl<B, C, D, I, N, T> NewWorld<B, C, D, I, N, T> where
        C: ConstraintSolver<I, N, T>,
        B: Broadphase<N, DynamicBodyType<T>>,
        D: Detection<N, DynamicBodyType<T>>,
        I: Integrator,
        N: Narrowphase,
        T: 'static {

    pub fn update(&mut self, time_step: Scalar) -> Vec<Contact<N, DynamicBodyType<T>>> {
        for mut body in self.broadphase.bodies_iter_mut() {
            if let Some(mut rigid_body) = RigidBodyRefMut::try_from(&mut body) {
                self.integrator.integrate_in_place(rigid_body.integratable(), time_step, self.gravity);
            }

            // TODO does this need to be handled by the Broadphase?
            // TODO only update if necessary?
            N::update(&mut body);
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
                N::update(&mut body);
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
