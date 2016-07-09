use Scalar;
use maths::UnitVec3D;

static POLAR_STEP: Scalar = 2.23725224525;
static AZIMUTH_STEP: Scalar = 1.01862612263;

pub struct UnitVec3DGenerator {
    polar: Scalar,
    azimuth: Scalar,
}

impl UnitVec3DGenerator {
    pub fn new() -> UnitVec3DGenerator {
        UnitVec3DGenerator {
            polar: -POLAR_STEP,
            azimuth: -AZIMUTH_STEP,
        }
    }

    pub fn gen_next(&mut self) -> UnitVec3D {
        self.polar += POLAR_STEP;
        self.azimuth += AZIMUTH_STEP;

        UnitVec3D::from_angles(self.polar, self.azimuth)
    }
}

impl Iterator for UnitVec3DGenerator {
    type Item = UnitVec3D;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.gen_next())
    }
}
