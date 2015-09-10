use mach::shapes::Cube;
use mach::materials::{ Material, Rigid };

#[test]
fn instantiating_with_density() {
    let r = Rigid::new(1.5);

    assert_eq!(r.density(), 1.5);
}

#[test]
fn instantiating_with_coefficient_of_restitution() {
    let r = Rigid::new(1.5).with_coefficient_of_restitution(0.75);

    assert_eq!(r.coefficient_of_restitution(), 0.75);
}

#[test]
fn determining_equality() {
    let a = Rigid::new(1.0);
    let b = Rigid::new(1.0);

    assert_eq!(a, b);
}

#[test]
fn computing_the_mass() {
    let c = Cube::new(1.0, 2.0, 3.0);
    let p = Rigid::new(2.0);

    assert_eq!(p.mass_of(&c), 12.0);
}

#[test]
fn computing_the_density() {
    let c = Cube::new(1.0, 2.0, 3.0);
    let p = Rigid::new(1.5);

    assert_eq!(p.density_of(&c), 1.5);
}
