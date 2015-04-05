extern crate mithril;
extern crate rand;

use rand::Rng;
use rand::SeedableRng;

#[test]
fn falling_cubes_test() {
    let collisions = mithril::collisions::SimpleCollisions::new();
    let dynamics = mithril::dynamics::SimpleDynamics::new();
    let mut world = mithril::World::new(
        mithril::utils::log::CollisionsLogger::new(collisions),
        mithril::utils::log::DynamicsLogger::new(dynamics),
    );

    let base_shape = mithril::shapes::Cube::new(1.0, 1.0, 1.0);
    let base_material = mithril::materials::Rigid::new(1.0);
    let mut random_number_generator = rand::StdRng::from_seed(&[1, 2, 3, 4]);

    for i in (0..500) {
        if i % 50 == 0 {
            let state = mithril::core::State::new_stationary()
                .with_velocity(
                    random_number_generator.next_f32(),
                    random_number_generator.next_f32(),
                    random_number_generator.next_f32(),
                )
                .with_angular_velocity(
                    random_number_generator.next_f32() - 0.5,
                    random_number_generator.next_f32() - 0.5,
                    random_number_generator.next_f32() - 0.5,
                );
            world.create_body(base_shape.clone(), base_material.clone(), state);
        }

        world.update(0.05);
    }
}
