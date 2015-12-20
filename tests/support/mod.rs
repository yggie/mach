#[macro_use]
mod behaviours;
mod entity_builder;
#[macro_use]
mod assert_approx_eq;

pub mod inputs;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod simulation;

pub use self::simulation::Simulation;
pub use self::entity_builder::EntityBuilder;
