use shapes::Shape;
use solvers::ForceSolver;
use properties::Property;
use collisions::BroadPhase;
use integrators::StateIntegrator;
use core::{ UID, Database, State };

/// A `World` is a physical world in mithril, it contains physical bodies and a
/// set of rules dictating how the bodies interact with the environment.
pub struct World<BP: BroadPhase> {
    database: Database,
    broadphase: BP,
    solver: ForceSolver,
    integrator: StateIntegrator,
}

impl<BP: BroadPhase> World<BP> {
    /// Initializes a new `World` instance with the `BroadPhase`, `ForceSolver`
    /// and `StateIntegrator` provided.
    pub fn new(broadphase: BP, solver: ForceSolver, integrator: StateIntegrator) -> World<BP> {
        World{
            database: Database::new(),
            broadphase: broadphase,
            solver: solver,
            integrator: integrator,
        }
    }

    /// Creates a new `Body` with the given `Shape`, `Property` and `State`.
    /// Returns the `UID` for the instance created.
    pub fn create_body<T: Shape, U: Property>(&mut self,
                                              shape: T,
                                              property: U,
                                              state: State) -> UID {
        self.database.create_body(shape, property, state)
    }

    /// Returns the number of `Body` instances currently in the `World`.
    pub fn num_bodies(&self) -> uint {
        self.database.size()
    }

    /// Steps the `World` forward in time by the specified amount.
    pub fn update(&mut self, time_step: f32) {
        let mut contacts = Vec::new();

        self.broadphase.reindex(&self.database);
        self.broadphase.each_contact(&self.database, |contact| contacts.push(contact));

        (self.solver)(&mut self.database, &contacts);

        // TODO possibly fixed in the future
        let integrator = self.integrator;
        self.database.each_body_mut(|body| integrator(body, time_step));
    }
}
