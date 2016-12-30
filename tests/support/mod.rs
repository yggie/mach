#[macro_use]
mod assert_approx_eq;
#[macro_use]
mod quickcheck_assert;
#[macro_use]
mod quickcheck_expect;
#[macro_use]
mod assert_approx_matching;
#[macro_use]
mod assert_properties_for_actions;

mod action;
mod property;
mod result_handle;
mod property_checker;
mod arbitrary_radians;
mod variable_size_vec;
mod arbitrary_positive_scalar;

pub use self::action::Action;
pub use self::property::Property;
pub use self::result_handle::ResultHandle;
pub use self::property_checker::PropertyCheck;
pub use self::arbitrary_radians::Radians;
pub use self::arbitrary_positive_scalar::PositiveScalar;
pub use self::assert_properties_for_actions::assert_properties_for_actions;
pub use self::variable_size_vec::{Four, One, Ten, VariableSizeVec};

use Scalar;
use collisions::Body;
use collisions::narrowphase::NullNarrowphase;

pub static TEST_SCALAR_BOUNDS: Scalar = 1e2;
pub type TestBody = Body<(), NullNarrowphase>;
