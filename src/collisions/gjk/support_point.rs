use maths::Vector;

#[derive(Clone, Debug)]
pub struct SupportPoint {
    pub vertex_indices: (usize, usize),
    pub value: Vector,
}
