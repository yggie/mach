use entities::VolumetricBody;
use collisions::Intersection;

pub trait ContactDetector {
    fn compute_contacts(&mut self, &VolumetricBody, &VolumetricBody) -> Option<Intersection>;
}
