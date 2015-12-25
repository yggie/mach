extern crate quickcheck;

use std::collections::HashSet;

use geometries::PlaneLocation;

use super::simplex::Simplex;
use super::simplex_cache::SimplexCache;
use super::minkowski_difference::MinkowskiDifference;

use support::{inputs, EntityBuilder};

pub fn assert_valid_simplex(cache: &SimplexCache, diff: &MinkowskiDifference) {
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
        let control = EntityBuilder::new_cube(1.0).build_body();
        let body = EntityBuilder::new_cube(1.0)
            .with_translation(4.0, 4.0, 4.0)
            .with_rotation(rot)
            .build_body();
        let diff = MinkowskiDifference::new(
            control.as_ref(),
            body.as_ref(),
        );

        let mut simplex_cache = SimplexCache::new(&diff);

        assert_valid_simplex(&simplex_cache, &diff);

        if let Some(_simplex) = simplex_cache.update_to_contain_origin(diff.clone()) {
            panic!("Expected the simplex not to contain the origin, but it did");
        }

        assert_valid_simplex(&simplex_cache, &diff);
    }

    quickcheck::quickcheck(property as fn(inputs::UnitQuat));
}

#[test]
fn it_can_handle_arbitrary_rotations_for_intersecting_bodies() {
    fn property(rot: inputs::UnitQuat) {
        let control = EntityBuilder::new_cube(1.0).build_body();
        let body = EntityBuilder::new_cube(1.0)
            .with_rotation(rot)
            .build_body();
        let diff = MinkowskiDifference::new(
            control.as_ref(),
            body.as_ref(),
        );

        let mut simplex_cache = SimplexCache::new(&diff);

        assert_valid_simplex(&simplex_cache, &diff);

        if let None = simplex_cache.update_to_contain_origin(diff.clone()) {
            panic!("Expected the simplex not to contain the origin, but it did");
        }

        assert_valid_simplex(&simplex_cache, &diff);
    }

    quickcheck::quickcheck(property as fn(inputs::UnitQuat));
}
