//! The `render` module is the interface with the mach browser debugger to allow
//! rendering of information. This was primarily developed to aid debugging.

use core::{ RigidBody, StaticBody };

/// Logs an event for the creation of a `RigidBody`
pub fn create_rigid_body(rigid_body: &RigidBody) {
    if cfg!(feature="debug_renderevent") {
        println!("[CREATE] {}, Shape={}", rigid_body, rigid_body.shape());
    }
}

/// Logs an event for the creation of a `StaticBody`
pub fn create_static_body(static_body: &StaticBody) {
    if cfg!(feature="debug_renderevent") {
        println!("[CREATE] {}, Shape={}", static_body, static_body.shape());
    }
}

/// Logs an event indicating the start of the update step
pub fn update_start(time_step: f32) {
    if cfg!(feature="debug_renderevent") {
        println!("[UPDATE] START step={}", time_step);
    }
}

/// Logs an event indicating the end of the update step
pub fn update_end() {
    if cfg!(feature="debug_renderevent") {
        println!("[UPDATE] END");
    }
}

/// Logs an event which shows the current state of a `RigidBody` during the update step
pub fn update_rigid_body(rigid_body: &RigidBody) {
    if cfg!(feature="debug_renderevent") {
        println!("[UPDATE] {}", rigid_body);
    }
}

/// Logs an event which shows the current state of a `StaticBody` during the update step
pub fn update_static_body(static_body: &StaticBody) {
    if cfg!(feature="debug_renderevent") {
        println!("[UPDATE] {}", static_body);
    }
}

/// Logs an event indicating a violation of some sort has occurred. Violations
/// will be highlighted by the debugger but will stop the execution of the
/// program.
pub fn violation(kind: &str, message: &str) {
    if cfg!(feature="debug_renderevent") {
        println!("[VIOLATION] <{}>: {}", kind, message);
    }
}
