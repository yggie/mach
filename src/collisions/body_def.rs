use maths::Transform;
use shapes::Shape;
use collisions::CollisionGroup;

pub struct BodyDef<D> {
    pub group: CollisionGroup,
    pub shape: Box<Shape>,
    pub transform: Transform,
    pub extra_data: D,
}
