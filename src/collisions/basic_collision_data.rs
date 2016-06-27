use maths::{Transform, Vec3D};
use shapes::Shape;
use collisions::SupportMap;

#[derive(Clone, Debug)]
pub struct BasicCollisionData {
    shape: Box<Shape>,
    transform: Transform,
}

impl BasicCollisionData {
    pub fn new(shape: Box<Shape>, transform: Transform) -> BasicCollisionData {
        BasicCollisionData {
            shape: shape,
            transform: transform,
        }
    }

    #[inline(always)]
    pub fn shape(&self) -> &Shape {
        &*self.shape
    }

    #[inline(always)]
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    #[inline(always)]
    pub fn translation(&self) -> Vec3D {
        self.transform.translation()
    }

    pub fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vec3D> + 'a> {
        let vec = self.shape.vertices_iter()
            .map(|vertex| self.transform.apply_to_point(vertex))
            .collect::<Vec<Vec3D>>();

        Box::new(vec.into_iter())
    }
}

impl SupportMap for BasicCollisionData {
    fn support_points_iter<'a>(&'a self, direction: Vec3D) -> Box<Iterator<Item=Vec3D> + 'a> {
        let transform = self.transform;
        let new_direction = self.transform.apply_inverse_to_direction(direction);
        let iterator = self.shape.support_points_iter(new_direction)
            .map(move |vertex| transform.apply_to_point(vertex));

        return Box::new(iterator);
    }
}
