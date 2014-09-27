use collisions::Contact;

pub struct ContactGraph {
    pairs: Vec<Contact>,
}

impl ContactGraph {
    pub fn new() -> ContactGraph {
        ContactGraph{ pairs: Vec::new() }
    }

    pub fn add(&mut self, contact: Contact) {
        self.pairs.push(contact);
    }

    pub fn solve(&mut self) {
        while self.pairs.len() > 0 {
            self.pairs.pop();
        }
    }
}
