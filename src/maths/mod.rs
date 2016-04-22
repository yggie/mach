//! The `math` module contains all the logic associated with primitive
//! mathematical operations.

#[macro_use]
mod motion;
#[macro_use]
mod transform;

mod lcp;
mod quat;
mod vec_3d;
mod matrix;
mod unit_quat;
mod integratable;
mod sparse_matrix;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod approx_eq;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod lcp_solver;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod integrator;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod dot_product;

pub mod _2d;
pub mod integrators;
pub mod lcp_solvers;

pub use self::lcp::LCP;
pub use self::quat::Quat;
pub use self::vec_3d::Vec3D;
pub use self::matrix::Matrix;
pub use self::motion::Motion;
pub use self::approx_eq::ApproxEq;
pub use self::transform::Transform;
pub use self::unit_quat::UnitQuat;
pub use self::lcp_solver::LCPSolver;
pub use self::integrator::Integrator;
pub use self::dot_product::DotProduct;
pub use self::integratable::IntegratableMut;
pub use self::sparse_matrix::SparseMatrix;
