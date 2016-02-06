extern crate mach;

// this allows code to be shared between unit and integration tests
pub use mach::*;

#[macro_use]
#[cfg(test)]
mod support;

#[cfg(test)]
mod integration_tests {
    mod cube_systems_test;
}

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
#[path="entities"]
mod entities_tests {
    mod material_test;
}

#[cfg(test)]
#[path="maths"]
mod maths_tests {
    mod quat_test;
    mod vect_test;
    mod matrix_test;
    mod transform_test;
    mod sparse_matrix_test;
}

#[cfg(test)]
#[path="shapes"]
mod shapes_tests {
    mod cuboid_test;
}

#[cfg(test)]
#[path="utils"]
mod utils_tests {
    mod compute_surfaces_for_convex_hull_test;
}
