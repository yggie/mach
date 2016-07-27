extern crate quickcheck;

use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct VariableSizeVec<D, L, U> where L: SizeBound, U: SizeBound {
    vec: Vec<D>,
    _lower: PhantomData<L>,
    _upper: PhantomData<U>,
}

impl<D, L, U> VariableSizeVec<D, L, U> where L: SizeBound, U: SizeBound {
    pub fn to_vec(self) -> Vec<D> {
        self.vec
    }
}

impl<D, L, U> quickcheck::Arbitrary for VariableSizeVec<D, L, U> where D: quickcheck::Arbitrary, L: SizeBound, U: SizeBound {
    fn arbitrary<G>(rng: &mut G) -> Self where G: quickcheck::Gen {
        VariableSizeVec {
            vec: (L::value()..U::value()).map(|_| D::arbitrary(rng)).collect::<Vec<D>>(),
            _lower: PhantomData,
            _upper: PhantomData,
        }
    }
}

pub trait SizeBound: Clone + Send + 'static {
    fn value() -> usize;
}

#[derive(Clone, Debug)]
pub struct One;
unsafe impl Send for One { }
impl SizeBound for One {
    fn value() -> usize {
        1
    }
}

#[derive(Clone, Debug)]
pub struct Four;
unsafe impl Send for Four { }
impl SizeBound for Four {
    fn value() -> usize {
        4
    }
}

#[derive(Clone, Debug)]
pub struct Ten;
unsafe impl Send for Ten { }
impl SizeBound for Ten {
    fn value() -> usize {
        10
    }
}
