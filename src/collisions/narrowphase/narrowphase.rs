use entities::VolumetricBody;
use collisions::narrowphase::Intersection;

/// The `NarrowPhase` trait should be implemented by objects that are capable of
/// computing the intersection information between two shape entities. It is the
/// intention that to speed up collision detection, the intersection information
/// should be cached, therefore for optimal performance, individual
/// `NarrowPhase` instances should be created for each `VolumetricBody` pair.
pub trait NarrowPhase {
    /// Returns information about the intersection between two entities.
    fn find_intersection(&self, entity_0: &VolumetricBody, entity_1: &VolumetricBody) -> Option<Intersection>;
}
