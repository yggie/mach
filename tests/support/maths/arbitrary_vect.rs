extern crate quickcheck;

use maths::Vect;

impl quickcheck::Arbitrary for Vect {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        Vect::new(
            random.gen_range(-100.0, 100.0),
            random.gen_range(-100.0, 100.0),
            random.gen_range(-100.0, 100.0),
        )
    }
}
