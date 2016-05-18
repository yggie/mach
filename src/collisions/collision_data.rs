use maths::Transform;
use shapes::Shape;
use collisions::Narrowphase;

#[derive(Clone, Debug)]
pub struct CollisionData<D> {
    shape: Box<Shape>,
    transform: Transform,
    narrowphase_data: D,
}

impl<D> CollisionData<D> {
    pub fn new<N: Narrowphase<Data=D>>(strategy: &mut N, shape: Box<Shape>, transform: Transform) -> CollisionData<D> {
        let narrowphase_data = strategy.create_data(&*shape, &transform);

        CollisionData {
            shape: shape,
            transform: transform,
            narrowphase_data: narrowphase_data,
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
    pub fn narrowphase_data(&self) -> &D {
        &self.narrowphase_data
    }

    #[inline(always)]
    pub fn narrowphase_data_mut(&mut self) -> &D {
        &mut self.narrowphase_data
    }
}

#[cfg(test)]
impl CollisionData<()> {
    pub fn test_dummy(shape: Box<Shape>, transform: Transform) -> CollisionData<()> {
        CollisionData {
            shape: shape,
            transform: transform,
            narrowphase_data: (),
        }
    }
}
