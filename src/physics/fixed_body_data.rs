use physics::{MaterialData};
use collisions::{CollisionData, NarrowphaseData};

#[derive(Clone, Debug)]
pub struct FixedBodyData<T> where T: NarrowphaseData {
    pub collision_data: CollisionData<T>,
    pub material_data: MaterialData,
}
