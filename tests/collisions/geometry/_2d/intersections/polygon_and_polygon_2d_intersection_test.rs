use maths::_2d::Vec2D;
use collisions::geometry::Intersection;
use collisions::geometry::_2d::Polygon;

#[test]
fn it_recognises_when_a_polygon_is_fully_encapsulated_by_another() {
    let polygon_0 = Polygon::convex_hull_from_points(&vec!(
        Vec2D::new(-1.0, -1.0),
        Vec2D::new( 0.0,  1.0),
        Vec2D::new( 1.0, -1.0),
    )).expect("Polygon was supposed to be hardcoded as valid");

    let polygon_1 = Polygon::convex_hull_from_points(&vec!(
        Vec2D::new(-2.0, -2.0),
        Vec2D::new( 0.0,  2.0),
        Vec2D::new( 2.0, -2.0),
    )).expect("Polygon was supposed to be hardcoded as valid");

    let polygon = polygon_0.intersection(&polygon_1)
        .expect("expected an intersection between the two polygons, but got nothing");

    assert_approx_matching!(polygon.points(), vec!(
        Vec2D::new(-1.0, -1.0),
        Vec2D::new( 0.0,  1.0),
        Vec2D::new( 1.0, -1.0),
    ));
}

#[test]
fn it_recognises_when_the_polygons_intersect_including_points_on_the_polygons() {
    let polygon_0 = Polygon::convex_hull_from_points(&vec!(
        Vec2D::new(-1.0, -1.0),
        Vec2D::new( 0.0,  1.0),
        Vec2D::new( 1.0, -1.0),
    )).expect("Polygon was supposed to be hardcoded as valid");

    let polygon_1 = Polygon::convex_hull_from_points(&vec!(
        Vec2D::new( 0.0, -1.0),
        Vec2D::new( 1.0,  1.0),
        Vec2D::new( 2.0, -1.0),
    )).expect("Polygon was supposed to be hardcoded as valid");

    let polygon = polygon_0.intersection(&polygon_1)
        .expect("expected an intersection between the two polygons, but got nothing");

    assert_approx_matching!(polygon.points(), vec!(
        Vec2D::new(0.0, -1.0),
        Vec2D::new(0.5,  0.0),
        Vec2D::new(1.0, -1.0),
    ));
}

#[test]
fn it_recognises_when_the_polygons_intersect_excluding_any_points_on_either_polygon() {
    let polygon_0 = Polygon::convex_hull_from_points(&vec!(
        Vec2D::new(-1.0, -1.0),
        Vec2D::new( 0.0,  1.0),
        Vec2D::new( 1.0, -1.0),
    )).expect("Polygon was supposed to be hardcoded as valid");

    let polygon_1 = Polygon::convex_hull_from_points(&vec!(
        Vec2D::new(-1.0,  0.8),
        Vec2D::new( 0.0, -1.2),
        Vec2D::new( 1.0,  0.8),
    )).expect("Polygon was supposed to be hardcoded as valid");

    let polygon = polygon_0.intersection(&polygon_1)
        .expect("expected an intersection between the two polygons, but got nothing");

    assert_approx_matching!(polygon.points(), vec!(
        Vec2D::new( 0.10,  0.80),
        Vec2D::new(-0.10,  0.80),
        Vec2D::new( 0.55, -0.10),
        Vec2D::new(-0.55, -0.10),
        Vec2D::new( 0.10, -1.00),
        Vec2D::new(-0.10, -1.00),
    ));
}

#[test]
fn it_recognises_when_the_polygons_do_not_intersect() {
    let polygon_0 = Polygon::convex_hull_from_points(&vec!(
        Vec2D::new( 1.0, 0.0),
        Vec2D::new( 2.0, 1.0),
        Vec2D::new( 3.0, 0.0),
    )).expect("Polygon was supposed to be hardcoded as valid");

    let polygon_1 = Polygon::convex_hull_from_points(&vec!(
        Vec2D::new(-1.0, 0.0),
        Vec2D::new(-2.0, 1.0),
        Vec2D::new(-3.0, 0.0),
    )).expect("Polygon was supposed to be hardcoded as valid");

    if let Some(polygon) = polygon_0.intersection(&polygon_1) {
        panic!(format!("expected no intersection, but got {:?} instead", polygon));
    }
}
