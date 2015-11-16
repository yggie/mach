use Float;
use maths::{ Transform, Quat, Vector };

/// Represents a physical state. The state contains information regarding the
/// current position, rotation, velocity and rotational velocity.
#[derive(Clone, Copy, Debug)]
pub struct State {
    transform: Transform,
    vel: Vector,
    ang_vel: Vector,
}

impl State {
    /// Creates a new `State` with zero position, rotation, velocity and angular
    /// velocity.
    pub fn new_stationary() -> State {
        State {
            transform: Transform::new_identity(),
            vel: Vector::new_zero(),
            ang_vel: Vector::new_zero(),
        }
    }

    /// Creates a new `State` with a non-zero position.
    #[inline]
    pub fn new_with_pos(x: Float, y: Float, z: Float) -> State {
        State::new_stationary().with_pos(x, y, z)
    }

    /// Creates a new `State` with a non-zero rotation.
    #[inline]
    pub fn new_with_axis_angle(axis: Vector, angle_in_radians: Float) -> State {
        State::new_stationary().with_axis_angle(axis, angle_in_radians)
    }

    /// Returns the position of the `State`.
    #[inline(always)]
    pub fn pos(&self) -> Vector {
        self.transform.translation()
    }

    /// Returns the rotation of the `State` expressed as a `Quat`.
    #[inline(always)]
    pub fn rot(&self) -> Quat {
        self.transform.rotation()
    }

    /// Returns the velocity of the `State`.
    #[inline]
    pub fn vel(&self) -> Vector {
        self.vel
    }

    /// Returns the angular velocity of the `State`.
    #[inline]
    pub fn ang_vel(&self) -> Vector {
        self.ang_vel
    }

    /// Returns the associated `Transform` object.
    #[inline]
    pub fn transform(&self) -> Transform {
        self.transform
    }

    /// Sets the position using the scalar values provided.
    #[inline]
    pub fn set_pos(&mut self, values: &(Float, Float, Float)) {
        self.transform.translation_mut().set(values);
    }

    /// Returns a copy of the `State` using the inputs as components of the
    /// position `Vector`. This function can be chained.
    #[inline]
    pub fn with_pos(&self, x: Float, y: Float, z: Float) -> State {
        let mut state = self.clone();
        state.set_pos(&(x, y, z));
        return state;
    }

    /// Sets the rotation using a quaternion.
    #[inline]
    pub fn set_rot(&mut self, rot: &Quat) {
        self.transform.rotation_mut().set(rot);
    }

    /// Sets the rotation with the provided axis and angle of rotation.
    #[inline]
    pub fn set_axis_angle(&mut self, axis: Vector, angle_in_radians: Float) {
        let q = Quat::new_from_axis_angle(axis, angle_in_radians);
        self.set_rot(&q);
    }

    /// Returns a copy of the `State` using the specified angle and axis of
    /// rotation to initialize the rotation. This function can be chained.
    #[inline]
    pub fn with_axis_angle(&self, axis: Vector, angle_in_radians: Float) -> State {
        let mut state = self.clone();
        state.set_axis_angle(axis, angle_in_radians);
        return state;
    }

    /// Sets the velocity using the scalar values provided.
    #[inline]
    pub fn set_vel(&mut self, values: &(Float, Float, Float)) {
        self.vel.set(values);
    }

    /// Returns a copy of the `State` using the inputs as the components of the
    /// velocity `Vector`. This function can be chained.
    pub fn with_vel(&self, u: Float, v: Float, w: Float) -> State {
        let mut state = self.clone();
        state.set_vel(&(u, v, w));
        return state;
    }

    /// Sets the angular velocity using the specified values as components of a
    /// `Vector`.
    #[inline]
    pub fn set_ang_vel(&mut self, values: &(Float, Float, Float)) {
        self.ang_vel.set(values);
    }

    /// Returns a copy of the `State` using the inputs as components of the
    /// angular velocity `Vector`. This function can be chained.
    #[inline]
    pub fn with_ang_vel(&self, u: Float, v: Float, w: Float) -> State {
        let mut state = self.clone();
        state.set_ang_vel(&(u, v, w));
        return state;
    }

    /// Applies the `State` transformations to a `Vector`, treating the `Vector`
    /// as a point.
    pub fn transform_point(&self, point: Vector) -> Vector {
        point.rotate_by_quaternion(self.rot()) + self.pos()
    }

    /// Applies the `State` transformation to a `Vector`, treating the `Vector`
    /// as a direction.
    pub fn transform_direction(&self, direction: Vector) -> Vector {
        direction.rotate_by_quaternion(self.rot())
    }

    /// Applies the inverse `State` transformation to a `Vector`, treating the
    /// `Vector` as a direction.
    pub fn inverse_transform_direction(&self, direction: Vector) -> Vector {
        direction.rotate_by_quaternion(self.rot().inverse())
    }
}
