use std::marker::PhantomData;

use {Scalar, World};
use maths::Vec3D;
use utils::{Ref, RefMut, Handle};
use dynamics::{ConstraintSolver, DynamicBody, DynamicBodyExtension, FixedBodyData, FixedBodyDef, Integrator, RigidBodyData, RigidBodyDef, RigidBodyRefMut};
use collisions::{BodyDef, Broadphase, CollisionObject, CollisionGroup, Contact, Detection};
use collisions::shapes::{Intersection, Ray};
use collisions::shapes::convex_shapes::ShapeRef;

pub struct CustomWorld<B, C, D, E, I, O> where
        B: Broadphase<O>,
        C: ConstraintSolver<I, O>,
        D: Detection<O>,
        E: 'static,
        I: Integrator,
        O: CollisionObject<Extension=DynamicBodyExtension<E>> {

    gravity: Vec3D,
    detection: D,
    integrator: I,
    broadphase: B,
    constraint_solver: C,
    _extra: PhantomData<O>,
}

impl<B, C, D, E, I, O> CustomWorld<B, C, D, E, I, O> where
        B: Broadphase<O>,
        C: ConstraintSolver<I, O>,
        D: Detection<O>,
        E: 'static,
        I: Integrator,
        O: CollisionObject<Extension=DynamicBodyExtension<E>> {

    pub fn new(detection: D, integrator: I, broadphase: B, constraint_solver: C, gravity: Vec3D) -> CustomWorld<B, C, D, E, I, O> {
        CustomWorld {
            gravity: gravity,
            detection: detection,
            integrator: integrator,
            broadphase: broadphase,
            constraint_solver: constraint_solver,
            _extra: PhantomData,
        }
    }

    pub fn update(&mut self, time_step: Scalar) -> Vec<Contact<O>> {
        for mut body in self.broadphase.bodies_iter_mut() {
            if let Some(mut rigid_body) = RigidBodyRefMut::try_from(&mut *body) {
                self.integrator.integrate_in_place(&mut rigid_body.integratable(), time_step, self.gravity);
            }
        }

        self.broadphase.update();
        self.detection.update();

        let contacts: Vec<Contact<O>> = self.broadphase.close_proximity_pairs_iter()
            .filter_map(|pair| self.detection.compute_contacts(&pair.0, &pair.1))
            .collect();

        if contacts.len() > 0 {
            self.constraint_solver.solve_with_contacts(&contacts, &self.integrator, time_step);

            self.broadphase.update();
        }

        return contacts;
    }

    pub fn rigid_bodies_iter_mut<'a>(&'a self) -> Box<Iterator<Item=RefMut<O>> + 'a> {
        let iterator = self.broadphase.bodies_iter_mut()
            .filter(|body| {
                match O::extension_data(body) {
                    &DynamicBodyExtension::Rigid(_) => true,

                    _otherwise => false,
                }
            });

        return Box::new(iterator);
    }
}

impl<B, C, D, E, I, O> World<O> for CustomWorld<B, C, D, E, I, O> where
        B: Broadphase<O>,
        C: ConstraintSolver<I, O>,
        D: Detection<O>,
        E: 'static,
        I: Integrator,
        O: CollisionObject<Extension=DynamicBodyExtension<E>> {

    fn update(&mut self, time_step: Scalar) -> Vec<Contact<O>> {
        CustomWorld::update(self, time_step)
    }

    fn bodies_iter<'a>(&'a self) -> Box<Iterator<Item=Ref<O>> + 'a> {
        self.broadphase.bodies_iter()
    }

    fn set_gravity(&mut self, gravity: Vec3D) {
        self.gravity = gravity;
    }

    fn create_rigid_body(&mut self, def: RigidBodyDef, extension: <O as DynamicBody>::Extension) -> Handle<O> {
        let rigid_body_data = RigidBodyData::new(&def, extension);

        self.broadphase.create_body(BodyDef {
            group: def.group,
            shape: def.shape,
            rotation: def.rotation,
            translation: def.translation,
        }, DynamicBodyExtension::Rigid(Box::new(rigid_body_data)))
    }

    fn create_fixed_body(&mut self, def: FixedBodyDef, extension: <O as DynamicBody>::Extension) -> Handle<O> {
        let fixed_body_data = FixedBodyData::new(&def, extension);

        self.broadphase.create_body(BodyDef {
            group: CollisionGroup::Environment,
            shape: def.shape,
            rotation: def.rotation,
            translation: def.translation,
        }, DynamicBodyExtension::Fixed(Box::new(fixed_body_data)))
    }

    fn cast_ray<'a>(&'a self, ray: &Ray) -> Option<Ref<'a, O>> {
        self.broadphase.cast_ray(ray).filter(move |body| {
            match body.shape().downcast() {
                ShapeRef::Sphere(sphere) => {
                    sphere.fast_intersection(ray)
                },

                _otherwise => panic!("Unhandled shape-ray cast intersection"),
            }
        }).next()
    }
}
