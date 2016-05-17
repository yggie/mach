extern crate quickcheck;

use std::collections::HashSet;

use maths::UnitQuat;
use shapes::Cuboid;
use entities::RigidBody;
use geometry::PlaneNormalProjection;
use algorithms::{Execute, PanicOnIteration};

use detection::gjkepa::GJK;
use super::super::simplex::Simplex;
use super::super::simplex_cache::SimplexCache;
use super::super::minkowski_difference::MinkowskiDifference;

fn find_origin<'a>(cache: &'a mut SimplexCache, diff: &'a MinkowskiDifference) -> Option<Simplex<'a>> {
    GJK::new(cache, diff.clone())
        .panic_on_iteration(1000, "looking for origin (in tests)")
        .execute()
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

        let (vertex, _index_pair) = simplex.support_points[index];
        match surface.projection_along_normal(vertex) {
            PlaneNormalProjection::Above(_height) =>
                panic!(format!("{:?} is degenerate, a surface is pointing in the wrong direction", cache)),

            PlaneNormalProjection::OnPlane(_height) =>
                panic!(format!("{:?} is degenerate, all points are on the same plane", cache)),

            PlaneNormalProjection::Below(_height) => (),
        }
    }
}

#[test]
fn it_can_handle_arbitrary_rotations_for_non_intersecting_bodies() {
    fn property(rot: UnitQuat) {
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

    quickcheck::quickcheck(property as fn(UnitQuat));
}

#[test]
fn it_can_handle_arbitrary_rotations_for_intersecting_bodies() {
    fn property(rot: UnitQuat) {
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

    quickcheck::quickcheck(property as fn(UnitQuat));
}
