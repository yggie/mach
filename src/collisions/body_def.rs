use maths::{UnitQuat, Vec3D};
use collisions::CollisionGroup;
use collisions::geometry::shapes::{Cuboid, Shape};

#[derive(Clone, Debug)]
pub struct BodyDef {
    pub group: CollisionGroup,
    pub shape: Box<Shape>,
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
