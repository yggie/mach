use std;

use Scalar;
use maths::SparseMatrix;

pub type ValueConstraint = Fn(&LCP, Scalar) -> Scalar;

pub struct LCP {
    bias: Vec<Scalar>,
    matrix: SparseMatrix,
    solution: Vec<Scalar>,
    value_constraints: Vec<Box<ValueConstraint>>,
}

impl LCP {
    pub fn new(size: usize) -> LCP {
        let mut constraints: Vec<Box<ValueConstraint>> = Vec::with_capacity(size);
        let mut bias = Vec::with_capacity(size);
        let mut solution = Vec::with_capacity(size);

        for _i in 0..size {
            bias.push(0.0);
            solution.push(0.0);
            constraints.push(Box::new(|_problem, value| value));
        }

        return LCP {
            bias: bias,
            matrix: SparseMatrix::new(size),
            solution: solution,
            value_constraints: constraints,
        };
    }

    #[inline]
    pub fn matrix(&self, row: usize, column: usize) -> Scalar {
        self.matrix[(row, column)]
    }

    #[inline]
    pub fn matrix_mut(&mut self, row: usize, column: usize) -> &mut Scalar {
        &mut self.matrix[(row, column)]
    }

    #[inline]
    pub fn solution(&self, index: usize) -> Scalar {
        self.solution[index]
    }

    #[inline]
    pub fn unknown_mut(&mut self, index: usize) -> &mut Scalar {
        &mut self.solution[index]
    }

    #[inline]
    pub fn bias(&self, index: usize) -> Scalar {
        self.bias[index]
    }

    #[inline]
    pub fn bias_mut(&mut self, index: usize) -> &mut Scalar {
        &mut self.bias[index]
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.bias.len()
    }

    #[inline]
    pub fn add_value_constraint(&mut self, index: usize, constraint: Box<ValueConstraint>) {
        self.value_constraints[index] = constraint;
    }

    #[inline]
    pub fn apply_constraints(&self, index: usize, value: Scalar) -> Scalar {
        self.value_constraints[index](self, value)
    }
}

impl std::fmt::Debug for LCP {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        try!(write!(f, "LCP {{ matrix: {:?}, ", self.matrix));
        try!(write!(f, "bias: {:?}, ", self.bias));
        return write!(f, "solution: {:?} }}", self.solution);
    }
}
