#[cfg(test)]
#[path="../../../tests/maths/_2d/unit_vec_2d_test.rs"]
mod tests;

#[cfg(test)]
#[path="../../../tests/support/maths/_2d/arbitrary_unit_vec_2d.rs"]
mod arbitrary;

use std::ops::{Mul, Neg};

use Scalar;
use maths::DotProduct;
use maths::_2d::Vec2D;

#[derive(Copy, Clone, Debug)]
pub struct UnitVec2D(Vec2D);

impl UnitVec2D {
    #[inline(always)]
    pub fn vec(&self) -> &Vec2D {
        &self.0
    }

    pub fn from_radians(radians: Scalar) -> UnitVec2D {
        UnitVec2D(Vec2D::new(radians.cos(), radians.sin()))
    }

    pub fn rotate_90(&self) -> UnitVec2D {
        self.0.rotate_90().normalize()
    }

    pub fn rotate_by(&self, radians: Scalar) -> UnitVec2D {
        let x = radians.cos();
        let y = radians.sin();

        UnitVec2D(Vec2D::new(x * self.0.x - y * self.0.y, x * self.0.y + y * self.0.x))
    }

    pub fn reversed(self) -> UnitVec2D {
        UnitVec2D(-self.0)
    }
}

impl From<Vec2D> for UnitVec2D {
    fn from(vec: Vec2D) -> UnitVec2D {
        let length = vec.length();
        UnitVec2D(Vec2D::new(vec.x / length, vec.y / length))
    }
}

impl From<UnitVec2D> for Vec2D {
    fn from(unit_vec: UnitVec2D) -> Vec2D {
        unit_vec.0
    }
}

impl<'a> DotProduct<&'a Vec2D> for UnitVec2D {
    #[inline(always)]
    fn dot(&self, other: &'a Vec2D) -> Scalar {
        self.0.dot(other)
    }
}

impl DotProduct<Vec2D> for UnitVec2D {
    #[inline(always)]
    fn dot(&self, other: Vec2D) -> Scalar {
        self.dot(&other)
    }
}

impl<'a> DotProduct<&'a UnitVec2D> for UnitVec2D {
    #[inline(always)]
    fn dot(&self, other: &'a UnitVec2D) -> Scalar {
        self.dot(&other.0)
    }
}

impl DotProduct<UnitVec2D> for UnitVec2D {
    #[inline(always)]
    fn dot(&self, other: UnitVec2D) -> Scalar {
        self.dot(&other.0)
    }
}

impl<'a> DotProduct<&'a UnitVec2D> for Vec2D {
    #[inline(always)]
    fn dot(&self, other: &'a UnitVec2D) -> Scalar {
        self.dot(&other.0)
    }
}

impl DotProduct<UnitVec2D> for Vec2D {
    #[inline(always)]
    fn dot(&self, other: UnitVec2D) -> Scalar {
        self.dot(&other.0)
    }
}

impl<'a, 'b> Mul<&'a Scalar> for &'b UnitVec2D {
    type Output = Vec2D;

    #[inline]
    fn mul(self, scalar: &'a Scalar) -> Self::Output {
        scalar * (&self.0)
    }
}
implement_op_overload_variants!(Mul, mul, UnitVec2D, Scalar, Vec2D);

impl<'a, 'b> Mul<&'a UnitVec2D> for &'b Scalar {
    type Output = Vec2D;

    #[inline]
    fn mul(self, vec: &'a UnitVec2D) -> Self::Output {
        vec * self
    }
}
implement_op_overload_variants!(Mul, mul, Scalar, UnitVec2D, Vec2D);

impl<'a> Neg for &'a UnitVec2D {
    type Output = UnitVec2D;

    #[inline]
    fn neg(self) -> UnitVec2D {
        UnitVec2D(-&self.0)
    }
}

impl Neg for UnitVec2D {
    type Output = UnitVec2D;

    #[inline]
    fn neg(self) -> UnitVec2D {
        -&self
    }
}
