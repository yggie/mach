extern crate quickcheck;

use std::collections::HashSet;

use shapes::Cuboid;
use entities::RigidBody;
use geometry::PlaneLocation;
use algorithms::{IterativeAlgorithmExecutor, PanicOnIteration};

use detection::gjkepa::GJK;
use super::super::simplex::Simplex;
use super::super::simplex_cache::SimplexCache;
use super::super::minkowski_difference::MinkowskiDifference;

use support::inputs;

fn find_origin<'a>(cache: &'a mut SimplexCache, diff: &'a MinkowskiDifference) -> Option<Simplex<'a>> {
    let algorithm = PanicOnIteration::new(
        GJK::new(cache, diff.clone()),
        1000,
        "looking for origin (in tests)",
    );

    return IterativeAlgorithmExecutor::execute(algorithm);
}

fn assert_valid_simplex(cache: &SimplexCache, diff: &MinkowskiDifference) {
    let simplex = Simplex::new(cache, diff.clone());

    let indices = {
        let mut set = HashSet::with_capacity(4);
        set.insert(0);
        set.insert(1);
        set.insert(2);
        set.insert(3);

        set
    };

    for (ref surface, vertex_indices) in simplex.surfaces_iter() {
        let mut set = indices.clone();
        assert!(set.remove(&vertex_indices.0));
        assert!(set.remove(&vertex_indices.1));
        assert!(set.remove(&vertex_indices.2));

        let index = (0..4usize).find(|i| set.contains(&i))
            .map(|i| i.clone())
            .unwrap();

        let (ref vertex, _index_pair) = simplex.support_points[index];
        match surface.location_of(vertex) {
            PlaneLocation::Above =>
                panic!(format!("{:?} is degenerate, a surface is pointing in the wrong direction", cache)),

            PlaneLocation::Plane =>
                panic!(format!("{:?} is degenerate, all points are on the same plane", cache)),

            PlaneLocation::Below => (),
        }
    }
}

#[test]
fn it_can_handle_arbitrary_rotations_for_non_intersecting_bodies() {
    fn property(rot: inputs::UnitQuat) {
        let control = RigidBody::default().with_shape(Cuboid::cube(1.0));
        let rigid_body = RigidBody::default()
            .with_shape(Cuboid::cube(1.0))
            .with_translation(4.0, 4.0, 4.0)
            .with_rotation(rot.into());
        let diff = MinkowskiDifference(control.form(), rigid_body.form());

        let mut simplex_cache = SimplexCache::new(&diff);

        assert_valid_simplex(&simplex_cache, &diff);

        if let Some(_simplex) = find_origin(&mut simplex_cache, &diff) {
            panic!("Expected the simplex not to contain the origin, but it did");
        }

        assert_valid_simplex(&simplex_cache, &diff);
    }

    quickcheck::quickcheck(property as fn(inputs::UnitQuat));
}

#[test]
fn it_can_handle_arbitrary_rotations_for_intersecting_bodies() {
    fn property(rot: inputs::UnitQuat) {
        let control = RigidBody::default().with_shape(Cuboid::cube(1.0));
        let rigid_body = RigidBody::default()
            .with_shape(Cuboid::cube(1.0))
            .with_rotation(rot.into());
        let diff = MinkowskiDifference(control.form(), rigid_body.form());

        let mut simplex_cache = SimplexCache::new(&diff);

        assert_valid_simplex(&simplex_cache, &diff);

        if let None = find_origin(&mut simplex_cache, &diff) {
            panic!("Expected the simplex not to contain the origin, but it did");
        }

        assert_valid_simplex(&simplex_cache, &diff);
    }

    quickcheck::quickcheck(property as fn(inputs::UnitQuat));
}
