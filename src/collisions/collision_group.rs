#[cfg(test)]
#[path="../../tests/collisions/collision_group_test.rs"]
mod tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollisionGroup {
    Default,
    Environment,
    A,
    B,
    C,
    D,
    E,
}

impl CollisionGroup {
    pub fn test(group_0: CollisionGroup, group_1: CollisionGroup) -> bool {
        match (group_0, group_1) {
            (CollisionGroup::Environment, CollisionGroup::Environment) => false,

            (CollisionGroup::Environment, _) |
            (_, CollisionGroup::Environment) => true,

            (a, b) if a == b => true,

            _otherwise => false,
        }
    }
}
