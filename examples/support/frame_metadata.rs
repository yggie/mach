extern crate mach;

pub struct FrameMetadata {
    pub contacts: Vec<mach::maths::Vect>,
}

impl FrameMetadata {
    pub fn new() -> FrameMetadata {
        FrameMetadata {
            contacts: Vec::new(),
        }
    }
}
