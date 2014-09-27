use bodies::Body;

pub struct ContactGraph {
    pairs: uint,
}

impl ContactGraph {
    pub fn new() -> ContactGraph {
        ContactGraph{ pairs: 0 }
    }

    pub fn add_pair<'a>(&self, a: &'a Body, b: &'a Body) {
    }

    pub fn solve(&self) {
    }
}
