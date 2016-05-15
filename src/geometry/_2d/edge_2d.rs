use {Scalar, TOLERANCE};
use maths::DotProduct;
use maths::_2d::{UnitVec2D, Vec2D};
use geometry::LineProjection;
use geometry::_2d::{Line2D, Plane2D, Ray2D};

// TODO experimental approach using reference structs, is it worth the
// complexity of having something which is basically a Line2D? Can
// specialization save the day?
#[derive(Clone, Debug)]
pub struct Edge2D<'a> {
    pub start: &'a Vec2D,
    pub end: &'a Vec2D,
}

impl<'a> Edge2D<'a> {
    impl_line_2d_for!(Edge2D);

    pub fn new(start: &'a Vec2D, end: &'a Vec2D) -> Edge2D<'a> {
        Edge2D {
            start: start,
            end: end,
        }
    }

    #[inline]
    pub fn reversed(self) -> Edge2D<'a> {
        Edge2D::new(self.end, self.start)
    }

    pub fn as_line(&self) -> Line2D {
        Line2D::new(self.start.clone(), self.end.clone())
    }
}

impl<'a> From<Edge2D<'a>> for Line2D {
    fn from(edge: Edge2D<'a>) -> Line2D {
        Line2D::new(edge.start.clone(), edge.end.clone())
    }
}
