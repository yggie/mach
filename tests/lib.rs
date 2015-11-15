extern crate mach;

#[macro_use]
#[cfg(test)]
mod support;

#[cfg(test)]
mod integration_tests {
    mod cube_systems_test;
}

#[cfg(test)]
mod collisions {
    mod simple_collision_space_test;

    mod narrowphase {
        mod gjk_epa_implementation_test;
    }
}

#[cfg(test)]
mod dynamics {
    mod simple_dynamics_test;
}

#[cfg(test)]
mod entities {
    mod material_test;
}

#[cfg(test)]
mod maths {
    mod matrix_test;
    mod quat_test;
    mod vector_test;
    mod state_test;
    mod transform_test;
}

#[cfg(test)]
mod shapes {
    mod cuboid_test;
}

#[cfg(test)]
mod utils {
    mod compute_surfaces_for_convex_hull_test;
}
