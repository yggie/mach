extern crate mach;

// this allows code to be shared between unit and integration tests
pub use mach::*;

#[macro_use]
#[cfg(test)]
mod support;

#[cfg(test)]
#[path="utils"]
mod utils_tests {
    mod compute_surfaces_for_convex_hull_test;
}
