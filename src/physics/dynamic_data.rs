use Scalar;
use maths::Motion;

#[derive(Clone, Debug)]
pub struct DynamicData {
    mass: Scalar,
    motion: Motion,
}
