use maths::LCP;

pub trait LCPSolver {
    fn solve_in_place(&self, &mut LCP);
}
