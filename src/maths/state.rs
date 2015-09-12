use core::Float;
use maths::{ Transform, Quaternion, Vector };

/// Represents a physical state. The state contains information regarding the
/// current position, rotation, velocity and rotational velocity.
#[derive(Clone, Copy, Debug)]
pub struct State {
    transform: Transform,
    velocity: Vector,
    angular_velocity: Vector,
}

impl State {
    /// Creates a new `State` with zero position, rotation, velocity and angular
    /// velocity.
    pub fn new_stationary() -> State {
        State {
            transform: Transform::new_identity(),
            velocity: Vector::new_zero(),
            angular_velocity: Vector::new_zero(),
        }
    }

    /// Creates a new `State` with a non-zero position.
    #[inline]
    pub fn new_with_position(x: Float, y: Float, z: Float) -> State {
        State::new_stationary().with_position(x, y, z)
    }

    /// Creates a new `State` with a non-zero rotation.
    #[inline]
    pub fn new_with_axis_angle(axis: Vector, angle_in_radians: Float) -> State {
        State::new_stationary().with_axis_angle(axis, angle_in_radians)
    }

    /// Returns the position of the `State`.
    #[inline(always)]
    pub fn position(&self) -> Vector {
        self.transform.translation()
    }

    /// Returns the rotation of the `State` expressed as a `Quaternion`.
    #[inline(always)]
    pub fn rotation(&self) -> Quaternion {
        self.transform.rotation()
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

    /// Returns the associated `Transform` object.
    #[inline]
    pub fn transform(&self) -> Transform {
        self.transform
    }

    /// Sets the position using the specified values as components of a
    /// `Vector`.
    #[inline]
    pub fn set_position(&mut self, x: Float, y: Float, z: Float) {
        self.transform.translation_mut().set(x, y, z);
    }

    /// Sets the position to the `Vector` provided.
    #[inline]
    pub fn set_position_with_vector(&mut self, position: Vector) {
        self.set_position(position[0], position[1], position[2]);
    }

    /// Returns a copy of the `State` using the inputs as components of the
    /// position `Vector`. This function can be chained.
    #[inline]
    pub fn with_position(&self, x: Float, y: Float, z: Float) -> State {
        let mut state = self.clone();
        state.set_position(x, y, z);
        return state;
    }

    /// Sets the rotation using a quaternion.
    #[inline]
    pub fn set_rotation(&mut self, rotation: Quaternion) {
        self.transform.rotation_mut().copy(rotation);
    }

    /// Sets the rotation with the provided axis and angle of rotation.
    #[inline]
    pub fn set_axis_angle(&mut self, axis: Vector, angle_in_radians: Float) {
        let q = Quaternion::new_from_axis_angle(axis, angle_in_radians);
        self.set_rotation(q);
    }

    /// Returns a copy of the `State` using the specified angle and axis of
    /// rotation to initialize the rotation. This function can be chained.
    #[inline]
    pub fn with_axis_angle(&self, axis: Vector, angle_in_radians: Float) -> State {
        let mut state = self.clone();
        state.set_axis_angle(axis, angle_in_radians);
        return state;
    }

    /// Sets the velocity using the specified values as components of a
    /// `Vector`.
    #[inline]
    pub fn set_velocity(&mut self, u: Float, v: Float, w: Float) {
        self.velocity.set(u, v, w);
    }

    /// Returns a copy of the `State` using the inputs as the components of the
    /// velocity `Vector`. This function can be chained.
    pub fn with_velocity(&self, u: Float, v: Float, w: Float) -> State {
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
    pub fn set_angular_velocity(&mut self, u: Float, v: Float, w: Float) {
        self.angular_velocity.set(u, v, w);
    }

    /// Returns a copy of the `State` using the inputs as components of the
    /// angular velocity `Vector`. This function can be chained.
    #[inline]
    pub fn with_angular_velocity(&self, u: Float, v: Float, w: Float) -> State {
        let mut state = self.clone();
        state.set_angular_velocity(u, v, w);
        return state;
    }

    /// Applies the `State` transformations to a `Vector`, treating the `Vector`
    /// as a point.
    pub fn transform_point(&self, point: Vector) -> Vector {
        point.rotate_by_quaternion(self.rotation()) + self.position()
    }

    /// Applies the `State` transformation to a `Vector`, treating the `Vector`
    /// as a direction.
    pub fn transform_direction(&self, direction: Vector) -> Vector {
        direction.rotate_by_quaternion(self.rotation())
    }

    /// Applies the inverse `State` transformation to a `Vector`, treating the
    /// `Vector` as a direction.
    pub fn inverse_transform_direction(&self, direction: Vector) -> Vector {
        direction.rotate_by_quaternion(self.rotation().inverse())
    }
}
