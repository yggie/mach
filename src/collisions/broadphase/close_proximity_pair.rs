use collisions::{BodyHandle, Narrowphase};

pub struct CloseProximityPair<D, N>(pub BodyHandle<D, N>, pub BodyHandle<D, N>) where N: Narrowphase;
