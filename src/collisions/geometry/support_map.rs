use maths::Vec3D;

pub trait SupportMap {
    fn support_points_iter<'a>(&'a self, direction: Vec3D) -> Box<Iterator<Item=Vec3D> + 'a>;
}
