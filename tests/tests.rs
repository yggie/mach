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
#[path="collisions"]
mod collisions_tests {
    mod simple_collision_space_test;
}

#[cfg(test)]
#[path="dynamics"]
mod dynamics_tests {
    mod simple_dynamics_test;
}

#[cfg(test)]
#[path="entities"]
mod entities_tests {
    mod material_test;
}

#[cfg(test)]
#[path="maths"]
mod maths_tests {
    mod matrix_test;
    mod quat_test;
    mod vector_test;
    mod state_test;
    mod transform_test;
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
