use utils::Handle;
use collisions::CollisionObject;

pub struct CloseProximityPair<O>(pub Handle<O>, pub Handle<O>) where O: CollisionObject;
