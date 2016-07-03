use std::marker::PhantomData;

pub struct NewWorld<B, D, I, N, ND> where
        B: Broadphase,
        D: Detection,
        I: Integrator,
        N: Narrowphase,
        ND: NarrowphaseData {

    detection: D,
    broadphase: B,
    integrator: I,
    narrowphase: N,
    _narrowphase_data: PhantomData<ND>,
}

impl<B, D, I, N, ND> NewWorld where
        B: Broadphase,
        D: Detection,
        I: Integrator,
        N: Narrowphase,
        ND: NarrowphaseData {

    pub fn update(&mut self) {
        for body in self.broadphase.bodies_iter() {
            self.narrowphase.update(body.collision_data_mut());
        }

        self.detection.update();
    }

    fn compute_contacts(&mut self) -> Vec<Contact<ND>> {
        self.broadphase.close_proximity_pairs_iter()
            .filter_map(|objects| self.detection.compute_contacts(&objects.0, &objects.1))
            .collect()
    }
}
