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
mod approx_eq;
mod lcp_solver;
mod dot_product;
mod unit_vec_3d;
mod cross_product;
mod sparse_matrix;
mod approximations;
mod coordinate_transform;

pub mod _2d;
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
pub use self::dot_product::DotProduct;
pub use self::unit_vec_3d::UnitVec3D;
pub use self::cross_product::CrossProduct;
pub use self::sparse_matrix::SparseMatrix;
pub use self::approximations::Approximations;
pub use self::coordinate_transform::CoordinateTransform;
