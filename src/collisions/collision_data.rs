use maths::{Transform, Vec3D};
use shapes::Shape;
use collisions::geometry::SupportMap;

#[derive(Clone, Debug)]
pub struct CollisionData {
    shape: Box<Shape>,
    transform: Transform,
}

impl CollisionData {
    pub fn new(shape: Box<Shape>, transform: Transform) -> CollisionData {
        CollisionData {
            shape: shape,
            transform: transform,
        }
    }

    #[inline(always)]
    pub fn shape(&self) -> &Shape {
        &*self.shape
    }

    pub fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vec3D> + 'a> {
        let vec = self.shape.vertices_iter()
            .map(|vertex| self.transform.apply_to_point(vertex))
            .collect::<Vec<Vec3D>>();

        Box::new(vec.into_iter())
    }
}

impl SupportMap for CollisionData {
    fn support_points_iter<'a>(&'a self, direction: Vec3D) -> Box<Iterator<Item=Vec3D> + 'a> {
        let transform = self.transform;
        let new_direction = self.transform.apply_inverse_to_direction(direction);
        let iterator = self.shape.support_points_iter(new_direction)
            .map(move |vertex| transform.apply_to_point(vertex));

        return Box::new(iterator);
    }
}
