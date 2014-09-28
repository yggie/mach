use collisions::Contact;

/// A ContactGraph is a structure used to store and resolve collisions.
pub struct ContactGraph {
    pairs: Vec<Contact>,
}

impl ContactGraph {

    /// Constructs a new empty ContactGraph.
    pub fn new() -> ContactGraph {
        ContactGraph{ pairs: Vec::new() }
    }

    /// Adds a contact to the structure.
    pub fn add(&mut self, contact: Contact) {
        self.pairs.push(contact);
    }

    /// Solves and applies all the contacts currently contained in the
    /// structure.
    pub fn solve(&mut self) {
        while self.pairs.len() > 0 {
            self.pairs.pop();
        }
    }
}
