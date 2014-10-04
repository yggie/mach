#[cfg(test)]
pub mod helpers {
    use bodies::Body;
    use shapes::Sphere;
    use math::Transform;
    use properties::Rigid;
    use collisions::Space;

    use std::rc::Rc;

    pub fn assert_behaves_like_a_space<'a, T: Space<'a>>(space: &mut T) {
        let s = Sphere::new(1.0);
        let p = Rigid::new(2.0);
        let t = Transform::new_identity();
        let b = Rc::new(Body::new(box s, box p, t, t));
        let initial_count = space.size();

        // adding elements should work
        space.add(b);
        assert_eq!(space.size(), initial_count + 1);
    }
}
