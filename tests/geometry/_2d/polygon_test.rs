extern crate quickcheck;

use std::collections::HashSet;

use TOLERANCE;
use maths::_2d::Vec2D;
use geometry::PlaneLocation;
use geometry::_2d::{Plane2D, Polygon};

#[test]
fn it_cannot_be_created_with_less_than_3_points() {
    let points = vec!(Vec2D::new(1.0, 2.0));
    let polygon = Polygon::convex_hull_from(&points);

    assert!(polygon.is_err(), format!("expected the polygon to be invalid with {} points", points.len()));
}

#[test]
fn it_generates_convex_polygons_from_arbitrary_points() {
    fn validate_polygon_is_convex_hull(violations: &mut Violations, polygon: &Polygon, original_points: &Vec<Vec2D>) {
        let mut outliers = HashSet::new();
        for plane in polygon.separating_planes_iter() {
            for (index, point) in original_points.iter().enumerate() {
                let projection = plane.project_along_normal(point);

                if projection > TOLERANCE {
                    outliers.insert(index);
                }
            }
        }

        if outliers.len() != 0 {
            let edges: Vec<Plane2D> = polygon.separating_planes_iter().collect();
            let outlier_points = outliers.iter()
                .map(|&index| &original_points[index])
                .collect::<Vec<&Vec2D>>();

            violations.add(
                &format!("expected all {} points to be inside the polygon, but \
                    found {} outlier(s) for a polygon with {} point(s): {:?}, \
                    outlier(s): {:?}, {} plane(s): {:?}, this indicates the polygon is not the \
                    convex hull for the given set of points",
                    original_points.len(),
                    outliers.len(),
                    polygon.points().len(),
                    polygon.points(),
                    outlier_points,
                    edges.len(),
                    edges,
                )
            );
        }
    }

    fn validate_polygon_is_closed(violations: &mut Violations, polygon: &Polygon) {
        let num_points = polygon.points().len();
        let num_edges = polygon.separating_planes_iter().count();

        if num_points != num_edges {
            violations.add(&format!("got {} edges instead of the expected {} \
                for {} points, this indicates the polygon formed could be \
                self-intersecting and/or not closed",
                num_edges,
                num_points,
                num_points,
            ));
        } else {
            for point in polygon.points().iter() {
                let num_intersections = polygon.separating_planes_iter().fold(0, |num, plane| {
                    match plane.projection_of(point) {
                        PlaneLocation::OnPlane(_height) => num + 1,
                        _otherwise => num,
                    }
                });

                if num_intersections != 2 {
                    violations.add(&format!("expected {:?} to intersect \
                        exactly 2 polygon edge planes, but instead it \
                        intersected {} planes. This indicates that the polygon \
                        is not closed.",
                        point,
                        num_intersections,
                    ));
                }
            }
        }
    }

    fn property(point_cloud: PointCloud2D) {
        let points = point_cloud.0;
        let polygon = Polygon::convex_hull_from(&points)
            .expect(&format!("Test was setup to result in a valid convex hull, but it was invalid: {:?}", points));

        let mut violations = Violations::new();

        validate_polygon_is_convex_hull(&mut violations, &polygon, &points);
        validate_polygon_is_closed(&mut violations, &polygon);

        violations.assert_none();
    }

    quickcheck::quickcheck(property as fn(PointCloud2D));
}

pub struct Violations(Vec<String>);

impl Violations {
    pub fn new() -> Violations {
        Violations(vec![String::from("dummy")])
    }

    pub fn add(&mut self, message: &str) {
        self.0.push(String::from(message));
    }

    pub fn assert_none(mut self) {
        if self.0.len() > 1 {
            self.0[0] = format!("Found {} violations to the property:", self.0.len() - 1);
            panic!(self.0.join("\n"));
        }
    }
}

impl quickcheck::Arbitrary for Vec2D {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        Vec2D::new(
            quickcheck::Arbitrary::arbitrary(random),
            quickcheck::Arbitrary::arbitrary(random),
        )
    }
}

#[derive(Clone, Debug)]
pub struct PointCloud2D(Vec<Vec2D>);

impl quickcheck::Arbitrary for PointCloud2D {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        let length = random.gen_range(3, 9);
        let mut points = Vec::with_capacity(length);

        for _ in 0..length {
            points.push(quickcheck::Arbitrary::arbitrary(random));
        }

        PointCloud2D(points)
    }
}
