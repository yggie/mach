use {ID, Scalar};
use maths::{Quat, Vect};
use entities::{Body, BodyParams, RigidBody, ShapeDesc};

#[derive(Clone)]
pub struct StandaloneEntityBuilder {
    params: BodyParams,
}

impl StandaloneEntityBuilder {
    inline_chainable_params_methods! {
        struct_signature: StandaloneEntityBuilder,
        struct_name: StandaloneEntityBuilder,
        field_name: params,
    }

    pub fn new() -> StandaloneEntityBuilder {
        StandaloneEntityBuilder::default()
    }

    pub fn cube(size: Scalar) -> StandaloneEntityBuilder {
        StandaloneEntityBuilder::default().as_cube(size)
    }

    pub fn build_body(self) -> Box<Body> {
        Box::new(self.build_rigid_body())
    }

    pub fn build_rigid_body(self) -> RigidBody {
        RigidBody::with_id(ID(0), &self.params)
    }
}

impl Default for StandaloneEntityBuilder {
    fn default() -> StandaloneEntityBuilder {
        StandaloneEntityBuilder {
            params: BodyParams::default(),
        }
    }
}
