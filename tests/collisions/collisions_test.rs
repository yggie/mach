#[cfg(test)]
pub mod helpers {
    use bodies::Body;
    use shapes::Sphere;
    use math::Transform;
    use properties::Rigid;
    use collisions::BroadPhase;

    use std::rc::Rc;

    pub fn assert_behaves_like_broadphase<T: BroadPhase>(broadphase: &mut T) {
        let s = Sphere::new(1.0);
        let p = Rigid::new(2.0);
        let t = Transform::identity();
        let b = Rc::new(Body::new(box s, box p, t));
        let initial_count = broadphase.count();

        // adding elements should work
        broadphase.add(&b);
        assert_eq!(broadphase.count(), initial_count + 1);

        // should have at least one partition to iterate
        let partitions = broadphase.partitions();
        assert!(partitions.len() > 0);

        for partition in partitions.iter() {
            assert!(true);
        }
    }
}
