extern crate mach;

pub struct FrameMetadata {
    pub contacts: Vec<mach::maths::Vec3D>,
}

impl FrameMetadata {
    pub fn new() -> FrameMetadata {
        FrameMetadata {
            contacts: Vec::new(),
        }
    }
}
