use maths::Vec3D;

#[derive(Clone, Debug)]
pub struct ConvexHull3D(Vec<Vec3D>);

impl ConvexHull3D {
    pub fn new(vertices: Vec<Vec3D>) -> ConvexHull3D {
        ConvexHull3D(vertices)
    }

    #[inline(always)]
    pub fn vertices(&self) -> &Vec<Vec3D> {
        &self.0
    }
}

impl From<ConvexHull3D> for Vec<Vec3D> {
    fn from(convex_hull: ConvexHull3D) -> Vec<Vec3D> {
        convex_hull.0
    }
}
