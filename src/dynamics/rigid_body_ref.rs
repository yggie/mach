use Scalar;
use maths::{Matrix, Vec3D};
use collisions::{BodyData, Narrowphase};
use dynamics::{DynamicBody, DynamicBodyRefMut, Integratable, RigidBodyData};

pub struct RigidBodyRef<'a, N, T>(&'a BodyData<N>, &'a RigidBodyData<T>) where T: 'static, N: Narrowphase;
pub struct RigidBodyRefMut<'a, N, T>(&'a mut BodyData<N>, &'a mut RigidBodyData<T>) where T: 'static, N: Narrowphase;

impl<'a, N, T> RigidBodyRef<'a, N, T> where N: Narrowphase {
    pub fn new(body_data: &'a BodyData<N>, extra_data: &'a RigidBodyData<T>) -> RigidBodyRef<'a, N, T> {
        RigidBodyRef(body_data, extra_data)
    }

    #[inline(always)]
    pub fn translation(&self) -> &Vec3D {
        self.0.translation()
    }

    #[inline(always)]
    pub fn friction_coefficient(&self) -> Scalar {
        self.1.friction_coefficient()
    }

    #[inline(always)]
    pub fn restitution_coefficient(&self) -> Scalar {
        self.1.restitution_coefficient()
    }

    #[inline(always)]
    pub fn velocity(&self) -> &Vec3D {
        self.1.velocity()
    }

    #[inline(always)]
    pub fn angular_velocity(&self) -> &Vec3D {
        self.1.angular_velocity()
    }

    #[inline(always)]
    pub fn mass_inverse(&self) -> Scalar {
        self.1.mass_inverse()
    }

    pub fn inertia(&self) -> Matrix {
        self.0.shape().inertia() * self.mass()
    }

    pub fn inertia_inverse(&self) -> Matrix {
        self.inertia().inverse()
    }

    #[inline(always)]
    pub fn mass(&self) -> Scalar {
        self.1.mass()
    }
}

impl<'a, N, T> RigidBodyRefMut<'a, N, T> where N: Narrowphase {
    pub fn new(body_data: &'a mut BodyData<N>, extra_data: &'a mut RigidBodyData<T>) -> RigidBodyRefMut<'a, N, T> {
        RigidBodyRefMut(body_data, extra_data)
    }

    pub fn try_from<'b>(body: &'b mut DynamicBody<N, T>) -> Option<RigidBodyRefMut<'b, N, T>> {
        let dynamic_body = DynamicBodyRefMut::from(body);

        match dynamic_body {
            DynamicBodyRefMut::Rigid(rigid_body_ref) => Some(rigid_body_ref),

            _otherwise => None,
        }
    }

    pub fn integratable<'b>(&'b mut self) -> Integratable<'b> {
        Integratable::new(self.0.transform_mut(), self.1.motion_mut())
    }

    #[inline(always)]
    pub fn translation(&self) -> &Vec3D {
        self.0.translation()
    }

    #[inline(always)]
    pub fn translation_mut(&mut self) -> &mut Vec3D {
        self.0.translation_mut()
    }

    #[inline(always)]
    pub fn velocity_mut(&mut self) -> &mut Vec3D {
        self.1.velocity_mut()
    }

    #[inline(always)]
    pub fn angular_velocity_mut(&mut self) -> &mut Vec3D {
        self.1.angular_velocity_mut()
    }

    #[inline(always)]
    pub fn friction_coefficient(&self) -> Scalar {
        self.1.friction_coefficient()
    }

    #[inline(always)]
    pub fn mass(&self) -> Scalar {
        self.1.mass()
    }

    pub fn inertia(&self) -> Matrix {
        self.0.shape().inertia() * self.mass()
    }

    pub fn inertia_inverse(&self) -> Matrix {
        self.inertia().inverse()
    }
}
