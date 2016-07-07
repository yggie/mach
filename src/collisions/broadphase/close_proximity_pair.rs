use utils::Handle;
use collisions::CollisionBody;

pub struct CloseProximityPair<B>(pub Handle<B>, pub Handle<B>) where B: CollisionBody;
