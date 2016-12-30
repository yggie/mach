extern crate mach;

mod camera;
mod instance;
mod primitives;
#[path="../../support/simulation.rs"]
mod simulation;
mod polygon_model;
mod frame_metadata;
mod examples_window;
mod examples_runner;
mod examples_renderer;
mod window_event_handler;

pub mod polygons;


pub use self::camera::Camera;
pub use self::instance::{Instance, InstanceFactory};
pub use self::primitives::{Normal, Vertex};
pub use self::simulation::Simulation;
pub use self::polygon_model::PolygonModel;
pub use self::frame_metadata::FrameMetadata;
pub use self::examples_window::ExamplesWindow;
pub use self::examples_runner::ExamplesRunner;
pub use self::examples_renderer::ExamplesRenderer;
pub use self::window_event_handler::{EventResponse, WindowEventHandler};

pub struct SceneEnv<'a> {
    camera: &'a Camera,
}

pub type ExampleWorld = mach::World<mach::MachBody<()>>;
