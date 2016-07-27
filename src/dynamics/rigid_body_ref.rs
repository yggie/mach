use Scalar;
use maths::{Matrix, Vec3D};
use collisions::BodyData;
use dynamics::{DynamicBody, DynamicBodyRef, DynamicBodyRefMut, Integratable, RigidBodyData};

pub struct RigidBodyRef<'a, T>(&'a BodyData<T::Narrowphase>, &'a RigidBodyData<<T as DynamicBody>::Extension>) where T: DynamicBody;
pub struct RigidBodyRefMut<'a, T>(&'a mut BodyData<T::Narrowphase>, &'a mut RigidBodyData<<T as DynamicBody>::Extension>) where T: DynamicBody;

impl<'a, T> RigidBodyRef<'a, T> where T: DynamicBody {
    pub fn new(body_data: &'a BodyData<T::Narrowphase>, rigid_body_data: &'a RigidBodyData<<T as DynamicBody>::Extension>) -> RigidBodyRef<'a, T> {
        RigidBodyRef(body_data, rigid_body_data)
    }

    pub fn try_from(body: &'a T) -> Option<RigidBodyRef<'a, T>> {
        let dynamic_body = DynamicBodyRef::from(body);

        match dynamic_body {
            DynamicBodyRef::Rigid(rigid_body_ref) => Some(rigid_body_ref),

            _otherwise => None,
        }
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

impl<'a, T> RigidBodyRefMut<'a, T> where T: DynamicBody {
    pub fn new(body_data: &'a mut BodyData<T::Narrowphase>, rigid_body_data: &'a mut RigidBodyData<<T as DynamicBody>::Extension>) -> RigidBodyRefMut<'a, T> {
        RigidBodyRefMut(body_data, rigid_body_data)
    }

    pub fn try_from(body: &'a mut T) -> Option<RigidBodyRefMut<'a, T>> {
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
