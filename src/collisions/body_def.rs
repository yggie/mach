use maths::{UnitQuat, Vec3D};
use collisions::CollisionGroup;
use collisions::geometry::convex_shapes::{Cuboid, ConvexShape};

#[derive(Clone, Debug)]
pub struct BodyDef {
    pub group: CollisionGroup,
    pub shape: Box<ConvexShape>,
    pub translation: Vec3D,
    pub rotation: UnitQuat,
}

impl Default for BodyDef {
    fn default() -> BodyDef {
        BodyDef {
            group: CollisionGroup::Default,
            shape: Box::new(Cuboid::cube(1.0)),
            rotation: UnitQuat::identity(),
            translation: Vec3D::zero(),
        }
    }
}
