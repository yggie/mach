use ID;
use collisions::{BodyDef, CollisionData, CollisionGroup, Narrowphase};

#[derive(Clone, Debug)]
pub struct Body<D, N> where N: Narrowphase {
    id: ID,
    group: CollisionGroup,
    extra_data: D,
    collision_data: CollisionData,
    narrowphase_data: N,
}

impl<D, N> Body<D, N> where N: Narrowphase {
    pub fn new(id: ID, def: BodyDef<D>) -> Body<D, N> {
        let collision_data = CollisionData::new(def.shape, def.transform);
        let narrowphase_data = N::new(&collision_data);

        Body {
            id: id,
            group: def.group,
            extra_data: def.extra_data,
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
}
