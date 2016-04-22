#[cfg(test)]
#[path="../../../tests/maths/_2d/vec_2d_test.rs"]
mod tests;

#[cfg(test)]
#[path="../../../tests/support/maths/_2d/arbitrary_vec_2d.rs"]
mod arbitrary;

use std::ops::{Add, Mul, Neg, Sub};

use {Scalar, TOLERANCE};
use maths::{ApproxEq, DotProduct};
use maths::_2d::UnitVec2D;

#[derive(Clone, Debug, PartialEq)]
pub struct Vec2D {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vec2D {
    pub fn new(x: Scalar, y: Scalar) -> Vec2D {
        Vec2D {
            x: x,
            y: y,
        }
    }

    pub fn zero() -> Vec2D {
        Vec2D::new(0.0, 0.0)
    }

    pub fn rotate_90(&self) -> Vec2D {
        Vec2D::new(-self.y, self.x)
    }

    #[inline]
    pub fn squared_length(&self) -> Scalar {
        self.dot(self)
    }

    #[inline]
    pub fn length(&self) -> Scalar {
        self.squared_length().sqrt()
    }

    #[inline]
    pub fn normalize(&self) -> UnitVec2D {
        UnitVec2D::from_vec(self)
    }
}

impl<'a, 'b> Add<&'b Vec2D> for &'a Vec2D {
    type Output = Vec2D;

    #[inline]
    fn add(self, other: &'b Vec2D) -> Self::Output {
        Vec2D::new(self.x + other.x, self.y + other.y)
    }
}
implement_op_overload_variants!(Add, add, Vec2D, Vec2D, Vec2D);

impl<'a, 'b> Sub<&'b Vec2D> for &'a Vec2D {
    type Output = Vec2D;

    #[inline]
    fn sub(self, other: &'b Vec2D) -> Self::Output {
        Vec2D::new(self.x - other.x, self.y - other.y)
    }
}
implement_op_overload_variants!(Sub, sub, Vec2D, Vec2D, Vec2D);

impl<'a, 'b> Mul<&'a Scalar> for &'b Vec2D {
    type Output = Vec2D;

    #[inline]
    fn mul(self, scalar: &'a Scalar) -> Self::Output {
        Vec2D::new(self.x * scalar, self.y * scalar)
    }
}
implement_op_overload_variants!(Mul, mul, Vec2D, Scalar, Vec2D);

impl<'a, 'b> Mul<&'a Vec2D> for &'b Scalar {
    type Output = Vec2D;

    #[inline]
    fn mul(self, vec: &'a Vec2D) -> Self::Output {
        vec * self
    }
}
implement_op_overload_variants!(Mul, mul, Scalar, Vec2D, Vec2D);

impl<'a> Neg for &'a Vec2D {
    type Output = Vec2D;

    #[inline]
    fn neg(self) -> Vec2D {
        Vec2D::new(-self.x, -self.y)
    }
}

impl Neg for Vec2D {
    type Output = Vec2D;

    #[inline]
    fn neg(self) -> Vec2D {
        -&self
    }
}

impl<'a> DotProduct<&'a Vec2D> for Vec2D {
    #[inline(always)]
    fn dot(&self, other: &'a Vec2D) -> Scalar {
        self.x * other.x + self.y * other.y
    }
}

impl DotProduct<Vec2D> for Vec2D {
    #[inline(always)]
    fn dot(&self, other: Vec2D) -> Scalar {
        self.dot(&other)
    }
}

impl<'a> ApproxEq for &'a Vec2D {
    fn approx_eq(self, other: Self) -> bool {
        (self.squared_length() - other.squared_length()).abs() < TOLERANCE
    }
}
