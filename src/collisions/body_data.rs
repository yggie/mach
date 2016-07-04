use ID;
use maths::{Transform, Vec3D};
use shapes::Shape;
use collisions::{CollisionData, CollisionGroup, Narrowphase};

#[derive(Clone, Debug)]
pub struct BodyData<N> where N: Narrowphase {
    id: ID,
    group: CollisionGroup,
    collision_data: CollisionData,
    narrowphase_data: N,
}

impl<N> BodyData<N> where N: Narrowphase {
    pub fn new(id: ID, group: CollisionGroup, shape: Box<Shape>, transform: Transform) -> BodyData<N> {
        let collision_data = CollisionData::new(shape, transform);
        let narrowphase_data = N::new(&collision_data);

        BodyData {
            id: id,
            group: group,
            collision_data: collision_data,
            narrowphase_data: narrowphase_data,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> ID {
        self.id
    }

    #[inline(always)]
    pub fn group(&self) -> CollisionGroup {
        self.group
    }

    #[inline(always)]
    pub fn collision_data(&self) -> &CollisionData {
        &self.collision_data
    }

    #[inline(always)]
    pub fn shape(&self) -> &Shape {
        self.collision_data.shape()
    }

    #[inline(always)]
    pub fn narrowphase_data(&self) -> &N {
        &self.narrowphase_data
    }

    #[inline(always)]
    pub fn narrowphase_data_mut(&mut self) -> &mut N {
        &mut self.narrowphase_data
    }

    #[inline(always)]
    pub fn transform_mut(&mut self) -> &mut Transform {
        self.collision_data.transform_mut()
    }

    #[inline(always)]
    pub fn translation(&self) -> &Vec3D {
        self.collision_data.translation()
    }

    #[inline(always)]
    pub fn translation_mut(&mut self) -> &mut Vec3D {
        self.collision_data.translation_mut()
    }
}
