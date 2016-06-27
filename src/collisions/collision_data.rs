use maths::{Transform, Vec3D};
use shapes::Shape;
use collisions::{BasicCollisionData, Narrowphase, NarrowphaseData};

#[derive(Clone, Debug)]
pub struct CollisionData<T> where T: NarrowphaseData {
    basic_data: BasicCollisionData,
    narrowphase_data: T,
}

impl<T> CollisionData<T> where T: NarrowphaseData {
    pub fn new<N: Narrowphase<Data=T>>(strategy: &mut N, shape: Box<Shape>, transform: Transform) -> CollisionData<T> {
        let narrowphase_data = strategy.create_data(&*shape, &transform);

        CollisionData {
            basic_data: BasicCollisionData::new(shape, transform),
            narrowphase_data: narrowphase_data,
        }
    }

    #[inline(always)]
    pub fn basic_data(&self) -> &BasicCollisionData {
        &self.basic_data
    }

    #[inline(always)]
    pub fn shape(&self) -> &Shape {
        self.basic_data.shape()
    }

    #[inline(always)]
    pub fn translation(&self) -> Vec3D {
        self.basic_data.translation()
    }

    #[inline(always)]
    pub fn transform(&self) -> &Transform {
        self.basic_data.transform()
    }

    #[inline(always)]
    pub fn narrowphase_data(&self) -> &T {
        &self.narrowphase_data
    }

    #[inline(always)]
    pub fn narrowphase_data_mut(&mut self) -> &T {
        &mut self.narrowphase_data
    }
}

#[cfg(test)]
impl CollisionData<()> {
    pub fn test_dummy(shape: Box<Shape>, transform: Transform) -> CollisionData<()> {
        CollisionData {
            basic_data: BasicCollisionData::new(shape, transform),
            narrowphase_data: (),
        }
    }
}
