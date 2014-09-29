use collisions::Contact;

/// A ContactGraph is a structure used to store and resolve collisions.
pub struct ContactGraph<'a> {
    pairs: Vec<Contact<'a>>,
}

impl<'a> ContactGraph<'a> {

    /// Constructs a new empty ContactGraph.
    pub fn new() -> ContactGraph<'a> {
        ContactGraph{ pairs: Vec::new() }
    }

    /// Adds a contact to the structure.
    pub fn add(&mut self, contact: Contact<'a>) {
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
