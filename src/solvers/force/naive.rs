use core::Database;
use collisions::Contact;

/// Naively solves for the contact forces
pub fn naive_solver(database: &Database, contacts: &Vec<Contact>) {
    for contact in contacts.iter() {
        match database.find_pair(contact.body_ids) {
            (Some(body_0), Some(body_1)) => {
                let masses = [body_0.mass(), body_1.mass()];
                let relative_velocity = [
                    body_0.velocity().dot(contact.normal),
                    body_1.velocity().dot(contact.normal),
                ];

                let impulse = relative_velocity[1]*masses[1] + relative_velocity[0]*masses[0];
                // contact.bodies[0].apply_impulse(contact.normal.scale(impulse));
                // contact.bodies[1].apply_impulse(contact.normal.scale(-impulse));
            },

            // TODO handle missing bodies
            _ => continue,
        }
    }
}
