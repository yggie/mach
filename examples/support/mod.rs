mod camera;
mod instance;
mod examples_runner;
mod examples_renderer;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
#[path="../../tests/support/mod.rs"]
pub mod tests_support;

pub use self::camera::{Camera, CameraDef};
pub use self::instance::{Instance, InstanceFactory};
pub use self::tests_support::{CollisionSpaceMonitor, DynamicsMonitor, MonitoredWorld, Simulation};
pub use self::examples_runner::ExamplesRunner;
pub use self::examples_renderer::ExamplesRenderer;

pub struct SceneEnv<'a> {
    camera: &'a Camera,
}
