use maths::Vector;

#[derive(Clone, Debug)]
pub struct Surface {
    pub normal: Vector,
    pub indices: (usize, usize, usize),
}
