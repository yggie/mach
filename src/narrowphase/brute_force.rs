#[cfg(test)]
#[path="../../tests/narrowphase/brute_force_test.rs"]
mod tests;

use entities::BodyHandle;
use narrowphase::Narrowphase;

pub struct BruteForce;

impl BruteForce {
    pub fn new() -> BruteForce {
        BruteForce
    }
}

impl Narrowphase for BruteForce {
    fn notify_body_created(&mut self, _body: &BodyHandle) {
        // do nothing
    }

    fn update(&mut self) {
        // do nothing
    }

    fn test(&self, _handle_0: &BodyHandle, _handle_1: &BodyHandle) -> bool {
        true
    }
}
