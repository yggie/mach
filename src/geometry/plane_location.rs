use Scalar;

#[derive(Clone, Debug, PartialEq)]
pub enum PlaneLocation {
    Above(Scalar),
    OnPlane(Scalar),
    Below(Scalar),
}

impl PlaneLocation {
    pub fn is_above_plane(self) -> bool {
        match self {
            PlaneLocation::Above(_height) => true,
            _otherwise => false,
        }
    }

    pub fn is_on_plane(self) -> bool {
        match self {
            PlaneLocation::OnPlane(_height) => true,
            _otherwise => false,
        }
    }

    pub fn is_below_plane(self) -> bool {
        match self {
            PlaneLocation::Below(_height) => true,
            _otherwise => false,
        }
    }
}
