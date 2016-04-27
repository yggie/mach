use Scalar;

#[derive(Clone, Debug, PartialEq)]
pub enum PlaneNormalProjection {
    Above(Scalar),
    OnPlane(Scalar),
    Below(Scalar),
}

impl PlaneNormalProjection {
    pub fn is_above_plane(self) -> bool {
        match self {
            PlaneNormalProjection::Above(_height) => true,
            _otherwise => false,
        }
    }

    pub fn is_on_plane(self) -> bool {
        match self {
            PlaneNormalProjection::OnPlane(_height) => true,
            _otherwise => false,
        }
    }

    pub fn is_below_plane(self) -> bool {
        match self {
            PlaneNormalProjection::Below(_height) => true,
            _otherwise => false,
        }
    }
}
