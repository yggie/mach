//! The `math` module contains all the logic associated with primitive
//! mathematical operations.

mod lcp;
mod quat;
mod vect;
mod matrix;
#[macro_use]
mod motion;
mod transform;
mod integratable;
mod sparse_matrix;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod approx_eq;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod lcp_solver;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod integrator;

pub mod integrators;
pub mod lcp_solvers;

pub use self::lcp::LCP;
pub use self::quat::Quat;
pub use self::vect::Vect;
pub use self::matrix::Matrix;
pub use self::motion::Motion;
pub use self::approx_eq::ApproxEq;
pub use self::transform::Transform;
pub use self::lcp_solver::LCPSolver;
pub use self::integrator::Integrator;
pub use self::integratable::IntegratableMut;
pub use self::sparse_matrix::SparseMatrix;
