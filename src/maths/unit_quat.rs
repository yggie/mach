#[cfg(test)]
#[path="../../tests/maths/unit_quat_test.rs"]
mod tests;

#[cfg(test)]
#[path="../../tests/support/maths/arbitrary_unit_quat.rs"]
mod arbitrary;

use std::ops::Mul;

use Scalar;
use maths::{ApproxEq, Quat, UnitVec3D, Vec3D};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnitQuat(Quat);

impl UnitQuat {
    #[inline(always)]
    pub fn identity() -> UnitQuat {
        UnitQuat(Quat::new(1.0, 0.0, 0.0, 0.0))
    }

    pub fn from_quat(quat: Quat) -> UnitQuat {
        UnitQuat(quat / quat.length())
    }

    pub fn from_axis_angle(axis: UnitVec3D, angle_in_radians: Scalar) -> UnitQuat {
        let axis = Vec3D::from(axis);
        let half_radians = angle_in_radians / 2.0;
        let s = half_radians.sin();
        let c = half_radians.cos();

        return UnitQuat(Quat::new(c, s*axis.x, s*axis.y, s*axis.z));
    }

    pub fn rotate(&self, vect: Vec3D) -> Vec3D {
        let result = self * Quat::new(0.0, vect.x, vect.y, vect.z) * self.inverse();
        return Vec3D::new(result.i, result.j, result.k);
    }

    #[inline]
    pub fn inverse(&self) -> UnitQuat {
        UnitQuat(self.0.inverse())
    }
}

impl From<UnitQuat> for Quat {
    #[inline(always)]
    fn from(unit_quat: UnitQuat) -> Quat {
        unit_quat.0
    }
}

impl<'a, 'b> ApproxEq<&'a UnitQuat> for &'b UnitQuat {
    fn approx_eq(self, other: &'a UnitQuat) -> bool {
        self.0.approx_eq(&other.0)
    }
}

impl<'a, 'b> ApproxEq<UnitQuat> for &'b UnitQuat {
    fn approx_eq(self, other: UnitQuat) -> bool {
        self.0.approx_eq(&other.0)
    }
}

impl<'a, 'b> Mul<&'a UnitQuat> for &'b UnitQuat {
    type Output = UnitQuat;

    fn mul(self, other: &'a UnitQuat) -> Self::Output {
        UnitQuat::from_quat(self.0 * other.0)
    }
}
implement_op_overload_variants!(Mul, mul, UnitQuat, UnitQuat, UnitQuat);

impl<'a, 'b> Mul<&'a Quat> for &'b UnitQuat {
    type Output = Quat;

    fn mul(self, other: &'a Quat) -> Self::Output {
        self.0 * other
    }
}
implement_op_overload_variants!(Mul, mul, UnitQuat, Quat, Quat);

impl<'a, 'b> Mul<&'a UnitQuat> for &'b Quat {
    type Output = Quat;

    fn mul(self, other: &'a UnitQuat) -> Self::Output {
        self * other.0
    }
}
implement_op_overload_variants!(Mul, mul, Quat, UnitQuat, Quat);
