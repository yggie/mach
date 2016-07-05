use Scalar;
use maths::{UnitQuat, Vec3D};
use shapes::{Cuboid, Shape};
use dynamics::MaterialData;
use collisions::CollisionGroup;

pub struct FixedBodyDef {
    pub group: CollisionGroup,
    pub shape: Box<Shape>,
    pub rotation: UnitQuat,
    pub translation: Vec3D,
    pub friction_coefficient: Scalar,
    pub restitution_coefficient: Scalar,
}

impl Default for FixedBodyDef {
    fn default() -> FixedBodyDef {
        let material_defaults = MaterialData::default();

        FixedBodyDef {
            group: CollisionGroup::Default,
            shape: Box::new(Cuboid::cube(1.0)),
            rotation: UnitQuat::identity(),
            translation: Vec3D::zero(),
            friction_coefficient: material_defaults.friction_coefficient,
            restitution_coefficient: material_defaults.restitution_coefficient,
        }
    }
}
