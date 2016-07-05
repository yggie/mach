use collisions::CollisionGroup;

#[test]
fn it_allows_non_environment_groups_to_collide_within_the_group() {
    assert!(CollisionGroup::test(CollisionGroup::Default, CollisionGroup::Default));
    assert!(CollisionGroup::test(CollisionGroup::A, CollisionGroup::A));
    assert!(CollisionGroup::test(CollisionGroup::B, CollisionGroup::B));
    assert!(CollisionGroup::test(CollisionGroup::C, CollisionGroup::C));
    assert!(CollisionGroup::test(CollisionGroup::D, CollisionGroup::D));
    assert!(CollisionGroup::test(CollisionGroup::E, CollisionGroup::E));
}

#[test]
fn it_does_not_allow_non_environment_groups_to_collide_across_groups() {
    assert!(!CollisionGroup::test(CollisionGroup::Default, CollisionGroup::A));
}

#[test]
fn it_allows_environment_group_to_collide_with_all_other_groups() {
    assert!(CollisionGroup::test(CollisionGroup::Default, CollisionGroup::Environment));
    assert!(CollisionGroup::test(CollisionGroup::A, CollisionGroup::Environment));
    assert!(CollisionGroup::test(CollisionGroup::B, CollisionGroup::Environment));
    assert!(CollisionGroup::test(CollisionGroup::C, CollisionGroup::Environment));
    assert!(CollisionGroup::test(CollisionGroup::D, CollisionGroup::Environment));
    assert!(CollisionGroup::test(CollisionGroup::E, CollisionGroup::Environment));
}

#[test]
fn it_does_not_allow_environment_group_to_collide_with_itself() {
    assert!(!CollisionGroup::test(CollisionGroup::Environment, CollisionGroup::Environment));
}
