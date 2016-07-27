#[macro_use]
mod assert_approx_eq;
#[macro_use]
mod assert_in_quickcheck;
#[macro_use]
mod assert_approx_matching;
#[macro_use]
mod assert_properties_for_actions;

mod result_handle;
mod arbitrary_radians;
mod variable_size_vec;
mod arbitrary_positive_scalar;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod action;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod property;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod property_checker;

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
