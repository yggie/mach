use ID;
use maths::{Transform, Vec3D};
use shapes::Shape;
use collisions::{BodyData, BodyDef, CollisionData, CollisionGroup, Narrowphase};
use collisions::narrowphase::{NarrowphaseRef, NarrowphaseRefMut};

pub trait CollisionBody: 'static {
    type Extension: 'static;
    type Narrowphase: Narrowphase;

    fn new(id: ID, def: BodyDef, extra: Self::Extension) -> Self;

    fn id(&self) -> ID;
    fn data(&self) -> &BodyData<Self::Narrowphase>;
    fn data_mut(&mut self) -> &mut BodyData<Self::Narrowphase>;
    fn group(&self) -> CollisionGroup;
    fn collision_data(&self) -> &CollisionData;
    fn shape(&self) -> &Shape;
    fn translation(&self) -> &Vec3D;
    fn transform(&self) -> &Transform;
    fn extension_data(&self) -> &Self::Extension;
    fn extension_data_mut(&mut self) -> &mut Self::Extension;

    fn narrowphase_ref(&self) -> NarrowphaseRef<Self::Narrowphase>;
    fn narrowphase_ref_mut(&mut self) -> NarrowphaseRefMut<Self::Narrowphase>;
    fn split_data_mut(&mut self) -> (&mut BodyData<Self::Narrowphase>, &mut Self::Extension);
}
