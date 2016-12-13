use {Scalar, TOLERANCE};
use maths::DotProduct;
use maths::_2d::{UnitVec2D, Vec2D};
use collisions::geometry::{Geometry, LineProjection};
use collisions::geometry::_2d::{Plane2D, Ray2D};

#[derive(Clone, Debug)]
pub struct Line2D {
    pub start: Vec2D,
    pub end: Vec2D,
}

impl Geometry for Line2D {}

// TODO eventually replace with specialization once supported: see https://github.com/rust-lang/rust/issues/31844
#[macro_export]
macro_rules! impl_line_2d_for {
    ($struct_name:ident) => {
        pub fn start_to_end(&self) -> Vec2D {
            (&self.end as &Vec2D) - (&self.start as &Vec2D)
        }

        pub fn direction(&self) -> UnitVec2D {
            self.start_to_end().normalize()
        }

        pub fn counter_clockwise_normal(&self) -> UnitVec2D {
            self.start_to_end().rotate_90().normalize()
        }

        pub fn counter_clockwise_plane(&self) -> Plane2D {
            Plane2D::new(self.start.clone(), self.counter_clockwise_normal())
        }

        pub fn as_ray(&self) -> Ray2D {
            Ray2D::new(self.start.clone(), self.direction())
        }

        pub fn as_ray_from_end(&self) -> Ray2D {
            Ray2D::new(self.end.clone(), self.direction())
        }

        pub fn squared_length(&self) -> Scalar {
            self.start_to_end().squared_length()
        }

        pub fn length(&self) -> Scalar {
            self.start_to_end().length()
        }

        pub fn project_along_direction(&self, vec: &Vec2D) -> Scalar {
            self.direction().dot(&(vec - (&self.start as &Vec2D)))
        }

        pub fn projection_of(&self, vec: &Vec2D) -> LineProjection {
            match self.project_along_direction(vec) {
                x if x < -TOLERANCE => LineProjection::Before(x),
                x if x * x + TOLERANCE > self.squared_length() => LineProjection::After(x),
                x => LineProjection::OnLine(x),
            }
        }
    };
}

impl Line2D {
    impl_line_2d_for!(Line2D);

    pub fn new(start: Vec2D, end: Vec2D) -> Line2D {
        Line2D {
            start: start,
            end: end,
        }
    }
}
