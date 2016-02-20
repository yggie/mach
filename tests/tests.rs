extern crate mach;

// this allows code to be shared between unit and integration tests
pub use mach::*;

#[macro_use]
#[cfg(test)]
mod support;

#[cfg(test)]
#[path="detection"]
mod detection_tests {
    mod mach_space_test;
}

#[cfg(test)]
#[path="dynamics"]
mod dynamics_tests {
    mod mach_dynamics_test;
}

#[cfg(test)]
#[path="utils"]
mod utils_tests {
    mod compute_surfaces_for_convex_hull_test;
}
