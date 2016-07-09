use maths::Vec3D;
use utils::compute_surfaces_for_convex_hull;

#[test]
fn it_works_with_a_cube() {
    let vertices = vec!(
        Vec3D::new( 1.0,  1.0,  1.0),
        Vec3D::new( 1.0,  1.0, -1.0),
        Vec3D::new( 1.0, -1.0,  1.0),
        Vec3D::new( 1.0, -1.0, -1.0),
        Vec3D::new(-1.0,  1.0,  1.0),
        Vec3D::new(-1.0,  1.0, -1.0),
        Vec3D::new(-1.0, -1.0,  1.0),
        Vec3D::new(-1.0, -1.0, -1.0),
    );

    let surfaces = compute_surfaces_for_convex_hull(&vertices);

    let _points_for_surfaces: Vec<(Vec3D, Vec3D, Vec3D)> = surfaces.iter().map(|surface| {
        (vertices[surface.nodes[0]], vertices[surface.nodes[1]], vertices[surface.nodes[2]])
    }).collect();
}

#[test]
fn it_works_with_a_tetrahedron() {
    let vertices = vec!(
        Vec3D::new(-1.0, -1.0,  1.0),
        Vec3D::new( 1.0,  1.0,  1.0),
        Vec3D::new(-1.0,  1.0,  1.0),
        Vec3D::new( 1.0,  1.0, -1.0),
    );

    let surfaces = compute_surfaces_for_convex_hull(&vertices);

    let _points_for_surfaces: Vec<(Vec3D, Vec3D, Vec3D)> = surfaces.iter().map(|surface| {
        (vertices[surface.nodes[0]], vertices[surface.nodes[1]], vertices[surface.nodes[2]])
    }).collect();
}
