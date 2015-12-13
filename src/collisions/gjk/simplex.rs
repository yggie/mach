use Scalar;
use maths::Vector;
use collisions::gjk::{MinkowskiDifference, SupportPoint};

#[derive(Debug)]
pub struct Simplex {
    support_points: [SupportPoint; 4],
}

impl Simplex {
    pub fn new(diff: &MinkowskiDifference) -> Simplex {
        let relative_position = diff.center();

        let support_point_0 = diff.support_points( &relative_position)[0].clone();
        let support_point_1 = diff.support_points(&-relative_position)[0].clone();

        let support_point_2 = {
            let guesses = [
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
            ];

            guesses.iter()
                .flat_map(|guess| {
                    diff.support_points(guess)
                }).find(|support_point| {
                    support_point != &support_point_0 &&
                        support_point != &support_point_1
                }).expect("should have found a match here")
        };

        let support_point_3 = {
            let datum = diff.vertex(&support_point_0);
            let a = diff.vertex(&support_point_2) - datum;
            let b = diff.vertex(&support_point_1) - datum;
            let norm = Vector::cross(&a, b).normalize();

            [1.0, -1.0 as Scalar].iter()
                .flat_map(|&multiplier| {
                    diff.support_points(&(norm * multiplier))
                }).find(|support_point| {
                    support_point != &support_point_0 &&
                        support_point != &support_point_1 &&
                        support_point != &support_point_2
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

    pub fn support_point(&self, index: usize) -> &SupportPoint {
        &self.support_points[index]
    }
}
