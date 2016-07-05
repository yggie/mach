use Scalar;

#[derive(Clone, Debug)]
pub struct MaterialData {
    pub friction_coefficient: Scalar,
    pub restitution_coefficient: Scalar,
}

impl Default for MaterialData {
    fn default() -> MaterialData {
        MaterialData {
            friction_coefficient: 0.7,
            restitution_coefficient: 0.7,
        }
    }
}
