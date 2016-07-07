use ID;
use maths::{Transform, Vec3D};
use shapes::Shape;
use collisions::{BodyDef, CollisionData, CollisionGroup, Narrowphase};
use collisions::narrowphase::{NarrowphaseRef, NarrowphaseRefMut};

#[derive(Clone, Debug)]
pub struct BodyData<N> where N: Narrowphase {
    id: ID,
    group: CollisionGroup,
    narrowphase_data: N,
    collision_data: CollisionData,
}

impl<N> BodyData<N> where N: Narrowphase {
    pub fn new(id: ID, def: BodyDef) -> BodyData<N> {
        let collision_data = CollisionData::new(def.shape, Transform {
            rotation: def.rotation,
            translation: def.translation,
        });
        let narrowphase_data = N::new(&collision_data);

        BodyData {
            id: id,
            group: def.group,
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
    pub fn transform(&self) -> &Transform {
        self.collision_data.transform()
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

    pub fn narrowphase_ref(&self) -> NarrowphaseRef<N> {
        NarrowphaseRef {
            collision_data: &self.collision_data,
            narrowphase_data: &self.narrowphase_data,
        }
    }

    pub fn narrowphase_ref_mut(&mut self) -> NarrowphaseRefMut<N> {
        NarrowphaseRefMut {
            collision_data: &self.collision_data,
            narrowphase_data: &mut self.narrowphase_data,
        }
    }
}
