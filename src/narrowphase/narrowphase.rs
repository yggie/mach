use entities::BodyHandle;

pub trait Narrowphase {
    fn notify_body_created(&mut self, &BodyHandle);
    fn update(&mut self);
    // possibly could be preloaded with positional data
    fn test(&self, &BodyHandle, &BodyHandle) -> bool;
}
