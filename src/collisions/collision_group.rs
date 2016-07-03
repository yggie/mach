#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollisionGroup {
    Foreground,
    Background,
}

impl CollisionGroup {
    pub fn test(group_0: CollisionGroup, group_1: CollisionGroup) -> bool {
        match (group_0, group_1) {
            (CollisionGroup::Foreground, _) |
            (_, CollisionGroup::Foreground) => true,

            _otherwise => false,
        }
    }
}
