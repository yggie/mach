use core::Database;
use collisions::Contact;

#[cfg(test)]
#[path="../../../tests/unit/solvers/force/naivesolver_test.rs"]
mod tests;

/// Naively solves for the contact forces
pub fn naive_solver(database: &mut Database, contacts: &Vec<Contact>) {
    for contact in contacts.iter() {
        match database.find_pair_mut(contact.body_ids[0], contact.body_ids[1]) {
            (Some(body_0), Some(body_1)) => {
                let masses = [body_0.mass(), body_1.mass()];
                let relative_velocity = [
                    body_0.velocity().dot(contact.normal),
                    body_1.velocity().dot(contact.normal),
                ];

                let impulse = relative_velocity[1]*masses[1] - relative_velocity[0]*masses[0];
                body_0.apply_impulse(contact.normal.scale(-impulse / masses[0]));
                body_1.apply_impulse(contact.normal.scale(impulse / masses[1]));
            },

            // TODO handle missing bodies
            _ => continue,
        }
    }
}
