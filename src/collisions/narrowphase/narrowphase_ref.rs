use collisions::{CollisionData, Narrowphase};

pub struct NarrowphaseRef<'a, N> where N: Narrowphase {
    pub collision_data: &'a CollisionData,
    pub narrowphase_data: &'a N,
}

pub struct NarrowphaseRefMut<'a, N> where N: Narrowphase {
    pub collision_data: &'a CollisionData,
    pub narrowphase_data: &'a mut N,
}
