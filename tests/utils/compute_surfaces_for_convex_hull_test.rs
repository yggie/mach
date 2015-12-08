use mach::maths::Vector;
use mach::utils::compute_surfaces_for_convex_hull;

// TODO very much a work in progress
#[test]
fn with_a_cube() {
    let vertices = vec!(
        Vector::new( 1.0,  1.0,  1.0),
        Vector::new( 1.0,  1.0, -1.0),
        Vector::new( 1.0, -1.0,  1.0),
        Vector::new( 1.0, -1.0, -1.0),
        Vector::new(-1.0,  1.0,  1.0),
        Vector::new(-1.0,  1.0, -1.0),
        Vector::new(-1.0, -1.0,  1.0),
        Vector::new(-1.0, -1.0, -1.0),
    );

    let surfaces = compute_surfaces_for_convex_hull(&vertices);

    let points_for_surfaces: Vec<(Vector, Vector, Vector)> = surfaces.iter().map(|surface| {
        (vertices[surface.nodes[0]], vertices[surface.nodes[1]], vertices[surface.nodes[2]])
    }).collect();
}
