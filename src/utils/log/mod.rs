//! The `utils::log` submodule contains utility functions to aid troubleshooting
//! of the engine.

use core::{ Body, Handle };

pub use self::dynamics_logger::DynamicsLogger;
pub use self::collisions_logger::CollisionsLogger;

mod dynamics_logger;
mod collisions_logger;

fn verbose_format_body<H: Handle>(body: &Body<H>) -> String {
    format!("{}, Shape={}", body, body.shape())
}
