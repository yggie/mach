use math::{ Vector, Quaternion };

#[cfg(test)]
#[path="../../tests/core/state_test.rs"]
mod tests;

/// Represents a physical state. The state contains information regarding the
/// current position, rotation, velocity and rotational velocity.
#[deriving(Clone)]
pub struct State {
    position: Vector,
    rotation: Quaternion,
    velocity: Vector,
    angular_velocity: Vector,
}

/// Safe to perform a semantic copy.
impl Copy for State { }

impl State {
    /// Creates a new `State` with zero position, rotation, velocity and angular
    /// velocity.
    pub fn new_stationary() -> State {
        State{
            position: Vector::new_zero(),
            rotation: Quaternion::new_identity(),
            velocity: Vector::new_zero(),
            angular_velocity: Vector::new_zero(),
        }
    }

    /// Creates a new `State` with a non-zero position.
    #[inline]
    pub fn new_with_position(x: f32, y: f32, z: f32) -> State {
        State::new_stationary().with_position(x, y, z)
    }

    /// Creates a new `State` with a non-zero rotation.
    #[inline]
    pub fn new_with_rotation(radians: f32, x: f32, y: f32, z: f32) -> State {
        State::new_stationary().with_rotation(radians, x, y, z)
    }

    /// Returns the position of the `State`.
    #[inline]
    pub fn position(&self) -> Vector {
        self.position
    }

    /// Returns the rotation of the `State` expressed as a `Quaternion`.
    #[inline]
    pub fn rotation(&self) -> Quaternion {
        self.rotation
    }

    /// Returns the velocity of the `State`.
    #[inline]
    pub fn velocity(&self) -> Vector {
        self.velocity
    }

    /// Returns the angular velocity of the `State`.
    #[inline]
    pub fn angular_velocity(&self) -> Vector {
        self.angular_velocity
    }

    /// Sets the position using the specified values as components of a
    /// `Vector`.
    #[inline]
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position.set(x, y, z);
    }

    /// Returns a copy of the `State` using the inputs as components of the
    /// position `Vector`. This function can be chained.
    #[inline]
    pub fn with_position(&self, x: f32, y: f32, z: f32) -> State {
        let mut s = self.clone();
        s.set_position(x, y, z);
        return s;
    }

    /// Sets the position to the `Vector` provided.
    #[inline]
    pub fn set_position_with_vector(&mut self, position: Vector) {
        self.position.set(position[0], position[1], position[2]);
    }

    /// Sets the rotation with the provided angle and axis of rotation.
    #[inline]
    pub fn set_rotation(&mut self, radians: f32, x: f32, y: f32, z: f32) {
        let q = Quaternion::new_from_rotation(radians, x, y, z);
        self.rotation.set(q[0], q[1], q[2], q[3]);
    }

    /// Returns a copy of the `State` using the specified angle and axis of
    /// rotation to initialize the rotation. This function can be chained.
    #[inline]
    pub fn with_rotation(&self, radians: f32, x: f32, y: f32, z: f32) -> State {
        let mut state = self.clone();
        state.set_rotation(radians, x, y, z);
        return state;
    }

    /// Sets the velocity using the specified values as components of a
    /// `Vector`.
    #[inline]
    pub fn set_velocity(&mut self, u: f32, v: f32, w: f32) {
        self.velocity.set(u, v, w);
    }

    /// Returns a copy of the `State` using the inputs as the components of the
    /// velocity `Vector`. This function can be chained.
    pub fn with_velocity(&self, u: f32, v: f32, w: f32) -> State {
        let mut state = self.clone();
        state.set_velocity(u, v, w);
        return state;
    }

    /// Sets the velocity to the `Vector` provided.
    #[inline]
    pub fn set_velocity_with_vector(&mut self, velocity: Vector) {
        self.velocity.set(velocity[0], velocity[1], velocity[2]);
    }

    /// Sets the angular velocity using the specified values as components of a
    /// `Vector`.
    #[inline]
    pub fn set_angular_velocity(&mut self, u: f32, v: f32, w: f32) {
        self.angular_velocity.set(u, v, w);
    }

    /// Returns a copy of the `State` using the inputs as components of the
    /// angular velocity `Vector`. This function can be chained.
    #[inline]
    pub fn with_angular_velocity(&self, u: f32, v: f32, w: f32) -> State {
        let mut state = self.clone();
        state.set_angular_velocity(u, v, w);
        return state;
    }
}
