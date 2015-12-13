use Scalar;
use maths::Vector;
use collisions::gjk::{MinkowskiDifference, SupportPoint};

#[derive(Debug)]
pub struct Simplex {
    support_points: [SupportPoint; 4],
}

impl Simplex {
    pub fn new(diff: MinkowskiDifference) -> Simplex {
        let relative_position = diff.center();

        let support_point_0 = diff.support_points( relative_position)[0].clone();
        let support_point_1 = diff.support_points(-relative_position)[0].clone();

        let support_point_2 = {
            let a = support_point_0.value - relative_position;
            let b = support_point_1.value - relative_position;
            let norm = Vector::cross(&a, b).normalize();

            [1.0, -1.0 as Scalar].iter()
                .flat_map(|&multiplier| {
                    diff.support_points(norm * multiplier)
                }).find(|support_point| {
                    support_point.vertex_indices != support_point_0.vertex_indices &&
                        support_point.vertex_indices != support_point_1.vertex_indices
                }).expect("should have found a match here")
        };

        let support_point_3 = {
            let a = support_point_2.value - support_point_0.value;
            let b = support_point_1.value - support_point_0.value;
            let norm = Vector::cross(&a, b).normalize();

            [1.0, -1.0].iter()
                .flat_map(|&multiplier| {
                    diff.support_points(norm * multiplier)
                }).find(|support_point| {
                    support_point.vertex_indices != support_point_0.vertex_indices &&
                        support_point.vertex_indices != support_point_1.vertex_indices &&
                        support_point.vertex_indices != support_point_2.vertex_indices
                }).expect("should have found a match here")
        };

        return Simplex {
            support_points: [
                support_point_0,
                support_point_1,
                support_point_2,
                support_point_3,
            ],
        };
    }

    pub fn vertex(&self, index: usize) -> Vector {
        self.support_points[index].value
    }
}
