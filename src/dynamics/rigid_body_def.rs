use Scalar;
use maths::{UnitQuat, Vec3D};
use dynamics::MaterialData;
use collisions::CollisionGroup;
use collisions::shapes::convex_shapes::{Cuboid, ConvexShape};

pub struct RigidBodyDef {
    pub mass: Scalar,
    pub group: CollisionGroup,
    pub shape: Box<ConvexShape>,
    pub rotation: UnitQuat,
    pub velocity: Vec3D,
    pub translation: Vec3D,
    pub angular_velocity: Vec3D,
    pub friction_coefficient: Scalar,
    pub restitution_coefficient: Scalar,
}

impl Default for RigidBodyDef {
    fn default() -> RigidBodyDef {
        let material_defaults = MaterialData::default();

        RigidBodyDef {
            mass: 1.0,
            group: CollisionGroup::Default,
            shape: Box::new(Cuboid::cube(1.0)),
            rotation: UnitQuat::identity(),
            velocity: Vec3D::zero(),
            translation: Vec3D::zero(),
            angular_velocity: Vec3D::zero(),
            friction_coefficient: material_defaults.friction_coefficient,
            restitution_coefficient: material_defaults.restitution_coefficient,
        }
    }
}
