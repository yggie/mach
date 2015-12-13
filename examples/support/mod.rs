mod camera;
mod instance;
mod primitives;
mod polygon_model;
mod world_renderer;
mod examples_runner;
mod examples_renderer;

pub mod polygons;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
#[path="../../tests/support/mod.rs"]
pub mod tests_support;

pub use self::camera::{Camera, CameraDef};
pub use self::instance::{Instance, InstanceFactory};
pub use self::primitives::{Normal, Vertex};
pub use self::polygon_model::PolygonModel;
pub use self::tests_support::{MonitoredWorld, Simulation};
pub use self::world_renderer::WorldRenderer;
pub use self::examples_runner::ExamplesRunner;
pub use self::examples_renderer::ExamplesRenderer;

pub struct SceneEnv<'a> {
    camera: &'a Camera,
}
