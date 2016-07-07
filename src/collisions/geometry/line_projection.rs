#[cfg(test)]
#[path="../../../tests/collisions/geometry/_2d/line_2d_test.rs"]
mod tests;

use Scalar;

#[derive(Clone, Debug, PartialEq)]
pub enum LineProjection {
    Before(Scalar),
    OnLine(Scalar),
    After(Scalar),
}

impl LineProjection {
    pub fn is_before_line(self) -> bool {
        match self {
            LineProjection::Before(_projection) => true,
            _otherwise => false,
        }
    }

    pub fn is_on_line(self) -> bool {
        match self {
            LineProjection::OnLine(_projection) => true,
            _otherwise => false,
        }
    }

    pub fn is_after_line(self) -> bool {
        match self {
            LineProjection::After(_projection) => true,
            _otherwise => false,
        }
    }
}
