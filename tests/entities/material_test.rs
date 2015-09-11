use mach::shapes::Cuboid;
use mach::entities::Material;

#[test]
fn it_can_be_instantiated_with_constant_density() {
    let material = Material::new_with_density(1.5);

    assert_eq!(material.density_of(&Cuboid::new_cube(2.0)), 1.5);
    assert_eq!(material.density_of(&Cuboid::new_cube(3.0)), 1.5);
    assert_eq!(material.density_of(&Cuboid::new_cube(4.0)), 1.5);
}

#[test]
fn it_can_be_instantiated_with_constant_mass() {
    let material = Material::new_with_mass(2.5);

    assert_eq!(material.mass_of(&Cuboid::new_cube(2.0)), 2.5);
    assert_eq!(material.mass_of(&Cuboid::new_cube(3.0)), 2.5);
    assert_eq!(material.mass_of(&Cuboid::new_cube(4.0)), 2.5);
}

#[test]
fn it_computes_the_mass_correctly_with_constant_density() {
    let cuboid = Cuboid::new(1.0, 2.0, 3.0);
    let material = Material::new_with_density(2.0);

    assert_eq!(material.mass_of(&cuboid), 12.0);
}

#[test]
fn it_computes_the_density_correctly_with_constant_density() {
    let cuboid = Cuboid::new(1.0, 2.0, 3.0);
    let material = Material::new_with_mass(12.0);

    assert_eq!(material.density_of(&cuboid), 2.0);
}
