#[macro_use]
#[cfg(test)]
mod behaviours;
#[macro_use]
#[cfg(test)]
mod assert_approx_eq;
#[cfg(test)]
mod test_shape;
#[cfg(test)]
mod test_transform;
#[cfg(test)]
mod test_volumetric_body;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod simulation;

pub use self::simulation::Simulation;
#[cfg(test)]
pub use self::test_shape::TestShape;
#[cfg(test)]
pub use self::test_transform::TestTransform;
#[cfg(test)]
pub use self::test_volumetric_body::TestVolumetricBody;
