use collisions::Contact;

/// A ContactGraph is a structure used to store and resolve collisions.
pub struct ContactGraph<'a> {
    contacts: Vec<Contact<'a>>,
}

impl<'a> ContactGraph<'a> {

    /// Constructs a new empty ContactGraph.
    pub fn new() -> ContactGraph<'a> {
        ContactGraph{ contacts: Vec::new() }
    }

    /// Adds a contact to the structure.
    pub fn add(&mut self, contact: Contact<'a>) {
        self.contacts.push(contact);
    }

    /// Solves for all the contacts currently present in the structure.
    pub fn solve(&mut self) {
        loop {
            let mut contact = match self.contacts.pop() {
                None => break,
                Some(c) => c,
            };

            let masses = [contact.bodies[0].mass(), contact.bodies[1].mass()];
            let relative_velocity = [
                contact.bodies[0].velocity().dot(contact.normal),
                contact.bodies[1].velocity().dot(contact.normal),
            ];

            let impulse = relative_velocity[1]*masses[1] + relative_velocity[0]*masses[0];
            // contact.bodies[0].apply_impulse(contact.normal.scale(impulse));
            // contact.bodies[1].apply_impulse(contact.normal.scale(-impulse));
        }
    }
}
