use collisions::{BodyHandle, Narrowphase};

pub struct CloseProximityPair<N, T>(pub BodyHandle<N, T>, pub BodyHandle<N, T>) where N: Narrowphase;
