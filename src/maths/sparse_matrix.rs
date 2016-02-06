#[cfg(test)]
#[path="../../tests/maths/sparse_matrix_test.rs"]
mod sparse_matrix_test;

use std;

use std::ops::{Index, IndexMut};

use Scalar;

pub struct SparseMatrix {
    size: usize,
    elements: Vec<Scalar>,
}

impl SparseMatrix {
    pub fn new(size: usize) -> SparseMatrix {
        let mut elements = Vec::with_capacity(size * size);

        for _i in 0..elements.capacity() {
            elements.push(0.0);
        }

        return SparseMatrix {
            size: size,
            elements: elements,
        };
    }
}

impl Index<(usize, usize)> for SparseMatrix {
    type Output = Scalar;

    fn index(&self, index: (usize, usize)) -> &Scalar {
        &self.elements[self.size * index.0 + index.1]
    }
}

impl IndexMut<(usize, usize)> for SparseMatrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Scalar {
        &mut self.elements[self.size * index.0 + index.1]
    }
}

impl std::fmt::Debug for SparseMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        try!(write!(f, "SparseMatrix {{ "));
        for (i, element) in self.elements.iter().enumerate() {
            try!(write!(f, "({}, {}): {}, ", i / self.size, i % self.size, element));
        }
        write!(f, "}}")
    }
}
