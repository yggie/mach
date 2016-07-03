use std::marker::PhantomData;

use collisions::{Broadphase, Contact, Detection, Narrowphase, NarrowphaseData};

pub struct CollisionSpace<B, D, N, ND> where
        B: Broadphase<ND>,
        D: Detection<ND>,
        N: Narrowphase<Data=ND>,
        ND: NarrowphaseData {

    detection: D,
    broadphase: B,
    narrowphase: N,
    _narrowphase_data: PhantomData<ND>,
}

impl<B, D, N, ND> CollisionSpace<B, D, N, ND> where B: Broadphase<ND>, D: Detection<ND>, N: Narrowphase<Data=ND>, ND: NarrowphaseData {
    fn update(&mut self) {
        for object in self.broadphase.objects_iter() {
            self.narrowphase.update(&mut object.data.borrow_mut());
        }
        self.detection.update();
    }

    fn compute_contacts(&mut self) -> Vec<Contact<ND>> {
        self.broadphase.possible_collision_pairs_iter()
            .filter_map(|objects| self.detection.compute_contacts(&objects.0, &objects.1))
            .collect()
    }
}
