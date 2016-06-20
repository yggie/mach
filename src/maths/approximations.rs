use TOLERANCE;

pub trait Approximations {
    fn is_strictly_positive(self) -> bool;
    fn is_strictly_negative(self) -> bool;
    fn is_approximately_zero(self) -> bool;
}

impl Approximations for f32 {
    #[inline(always)]
    fn is_strictly_positive(self) -> bool {
        self > TOLERANCE as f32
    }

    #[inline(always)]
    fn is_strictly_negative(self) -> bool {
        self < -TOLERANCE as f32
    }

    #[inline(always)]
    fn is_approximately_zero(self) -> bool {
        self.abs() < TOLERANCE as f32
    }
}

impl Approximations for f64 {
    #[inline(always)]
    fn is_strictly_positive(self) -> bool {
        self > TOLERANCE as f64
    }

    #[inline(always)]
    fn is_strictly_negative(self) -> bool {
        self < -TOLERANCE as f64
    }

    #[inline(always)]
    fn is_approximately_zero(self) -> bool {
        self.abs() < TOLERANCE as f64
    }
}
