use mach::maths::Vector;
use mach::shapes::{ Shape, Cuboid };

#[test]
fn instantiating_with_dimensions() {
    let c = Cuboid::new(5.0, 3.0, 7.5);

    let dim = c.dimensions();
    assert_eq!((dim.x, dim.y, dim.z), (5.0, 3.0, 7.5));
}

#[test]
fn computing_the_number_of_vertices() {
    let c = Cuboid::new(3.0, 2.0, 1.0);

    assert_eq!(c.vertices_len(), 8);
}

#[test]
fn computing_the_support_indices() {
    let c = Cuboid::new(2.0, 3.0, 1.0);
    let dir = Vector::new(-0.1, 1.0, 0.1);

    let indices = c.support_indices_for(dir);
    let v = c.vertex(indices[0]);

    assert_eq!(indices.len(), 1);
    assert_eq!((v.x, v.y, v.z), (-1.0, 1.5, 0.5));

    let other_indices = c.support_indices_for(Vector::new(1.0, 0.0, 0.0));
    let other_vertices: Vec<Vector> = other_indices.iter()
        .map(|&i| c.vertex(i))
        .collect();

    assert_eq!(other_indices.len(), 4);
    let v0 = other_vertices[0];
    let v1 = other_vertices[1];
    let v2 = other_vertices[2];
    let v3 = other_vertices[3];
    assert_eq!((v0.x, v0.y, v0.z), (1.0,  1.5,  0.5));
    assert_eq!((v1.x, v1.y, v1.z), (1.0, -1.5,  0.5));
    assert_eq!((v2.x, v2.y, v2.z), (1.0,  1.5, -0.5));
    assert_eq!((v3.x, v3.y, v3.z), (1.0, -1.5, -0.5));
}

#[test]
fn determining_equality() {
    let a = Cuboid::new(1.0, 2.0, 3.0);
    let b = Cuboid::new(1.0, 2.0, 3.0);

    assert_eq!(a, b);
}

#[test]
fn computing_the_volume() {
    let c = Cuboid::new(2.0, 3.0, 4.0);

    assert_eq!(c.volume(), 24.0);
}
