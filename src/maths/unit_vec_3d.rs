#[cfg(test)]
#[path="../../tests/maths/unit_vec_3d_test.rs"]
mod tests;

#[cfg(test)]
#[path="../../tests/support/maths/arbitrary_unit_vec_3d.rs"]
mod arbitrary;

use std::ops::{Mul, Neg};

use Scalar;
use maths::{ApproxEq, CrossProduct, DotProduct, Vec3D};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UnitVec3D(Vec3D);

impl UnitVec3D {
    pub fn from_angles(polar: Scalar, azimuth: Scalar) -> UnitVec3D {
        let cp = polar.cos();
        let sp = polar.sin();
        let ca = azimuth.cos();
        let sa = azimuth.sin();

        UnitVec3D(Vec3D::new(sp * ca, sp * sa, cp))
    }

    #[inline(always)]
    pub fn squared_length(&self) -> Scalar {
        self.0.squared_length()
    }
}

impl<'a> DotProduct<&'a Vec3D> for UnitVec3D {
    #[inline(always)]
    fn dot(&self, other: &'a Vec3D) -> Scalar {
        self.0.dot(other)
    }
}

impl DotProduct<Vec3D> for UnitVec3D {
    #[inline(always)]
    fn dot(&self, other: Vec3D) -> Scalar {
        self.0.dot(other)
    }
}

impl<'a> DotProduct<&'a UnitVec3D> for UnitVec3D {
    #[inline(always)]
    fn dot(&self, other: &'a UnitVec3D) -> Scalar {
        self.0.dot(&other.0)
    }
}

impl DotProduct<UnitVec3D> for UnitVec3D {
    #[inline(always)]
    fn dot(&self, other: UnitVec3D) -> Scalar {
        self.0.dot(&other.0)
    }
}

impl<'a> DotProduct<&'a UnitVec3D> for Vec3D {
    #[inline(always)]
    fn dot(&self, other: &'a UnitVec3D) -> Scalar {
        self.dot(&other.0)
    }
}

impl DotProduct<UnitVec3D> for Vec3D {
    #[inline(always)]
    fn dot(&self, other: UnitVec3D) -> Scalar {
        self.dot(&other.0)
    }
}

impl<'a> Mul<Scalar> for &'a UnitVec3D {
    type Output = Vec3D;

    fn mul(self, scalar: Scalar) -> Self::Output {
        self.0 * scalar
    }
}

impl Mul<Scalar> for UnitVec3D {
    type Output = Vec3D;

    fn mul(self, scalar: Scalar) -> Self::Output {
        self.0 * scalar
    }
}

impl Mul<UnitVec3D> for Scalar {
    type Output = Vec3D;

    fn mul(self, unit_vec: UnitVec3D) -> Self::Output {
        unit_vec.0 * self
    }
}

impl<'a> Neg for &'a UnitVec3D {
    type Output = UnitVec3D;

    fn neg(self) -> Self::Output {
        UnitVec3D(-self.0)
    }
}

impl Neg for UnitVec3D {
    type Output = UnitVec3D;

    fn neg(self) -> Self::Output {
        UnitVec3D(-self.0)
    }
}

impl<'a, 'b> ApproxEq<&'a UnitVec3D> for &'b UnitVec3D {
    fn approx_eq(self, other: &'a UnitVec3D) -> bool {
        self.0.approx_eq(other.0)
    }
}

impl<'a> ApproxEq<UnitVec3D> for &'a UnitVec3D {
    fn approx_eq(self, other: UnitVec3D) -> bool {
        self.0.approx_eq(other.0)
    }
}

impl<'a, 'b> CrossProduct<&'a UnitVec3D> for &'b UnitVec3D {
    type Output = UnitVec3D;

    fn cross(self, other: &'a UnitVec3D) -> Self::Output {
        UnitVec3D::from(self.0.cross(&other.0))
    }
}
implement_op_overload_variants!(CrossProduct, cross, UnitVec3D, UnitVec3D, UnitVec3D);

impl<'a, 'b> CrossProduct<&'a Vec3D> for &'b UnitVec3D {
    type Output = Vec3D;

    fn cross(self, other: &'a Vec3D) -> Self::Output {
        self.0.cross(other)
    }
}
implement_op_overload_variants!(CrossProduct, cross, UnitVec3D, Vec3D, Vec3D);

impl<'a, 'b> CrossProduct<&'a UnitVec3D> for &'b Vec3D {
    type Output = Vec3D;

    fn cross(self, other: &'a UnitVec3D) -> Self::Output {
        self.cross(other.0)
    }
}
implement_op_overload_variants!(CrossProduct, cross, Vec3D, UnitVec3D, Vec3D);

impl From<UnitVec3D> for Vec3D {
    fn from(unit_vec: UnitVec3D) -> Self {
        unit_vec.0
    }
}

impl From<Vec3D> for UnitVec3D {
    fn from(vec: Vec3D) -> Self {
        UnitVec3D(vec / vec.length())
    }
}
