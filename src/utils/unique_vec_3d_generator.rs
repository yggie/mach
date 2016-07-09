use maths::{ApproxEq, Vec3D};

pub struct UniqueVec3DGenerator<F> where F: FnMut() -> Vec3D {
    history: Vec<Vec3D>,
    next_func: F,
}

impl<F> UniqueVec3DGenerator<F> where F: FnMut() -> Vec3D {
    pub fn new(next_func: F) -> UniqueVec3DGenerator<F> {
        UniqueVec3DGenerator {
            history: Vec::new(),
            next_func: next_func,
        }
    }

    pub fn gen_next(&mut self) -> Vec3D {
        let mut counter = 0;
        while counter < 1000 {
            let guess = (self.next_func)();

            if !self.history.iter().any(|point| point.approx_eq(guess)) {
                self.history.push(guess);

                return guess;
            }

            counter += 1;
        }

        panic!("took more than 1000 iterations to compute a new unique point");
    }
}

impl<F> Iterator for UniqueVec3DGenerator<F> where F: FnMut() -> Vec3D {
    type Item = Vec3D;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.gen_next())
    }
}
