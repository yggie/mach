//! The `render` module is the interface with the mach browser debugger to allow
//! rendering of information. This was primarily developed to aid debugging.

use core::Float;
use maths::Vector;
use entities::{ RigidBody, StaticBody };
use collisions::Contact;

/// Logs an event for the creation of a `RigidBody`
pub fn create_rigid_body(rigid_body: &RigidBody) {
    if cfg!(feature="debug_renderevent") {
        println!("[NEW] {}, Shape={}", rigid_body, rigid_body.shape());
    }
}

/// Logs an event for the creation of a `StaticBody`
pub fn create_static_body(static_body: &StaticBody) {
    if cfg!(feature="debug_renderevent") {
        println!("[NEW] {}, Shape={}", static_body, static_body.shape());
    }
}

/// Logs an event indicating the start of the update step
pub fn update_start(time_step: Float) {
    if cfg!(feature="debug_renderevent") {
        println!("[FRAME] NEW step={}", time_step);
    }
}

/// Logs an event indicating the end of the update step
pub fn update_end() { }

/// Logs an event which shows the current state of a `RigidBody` during the update step
pub fn update_rigid_body(rigid_body: &RigidBody) {
    if cfg!(feature="debug_renderevent") {
        println!("[FRAME] {}", rigid_body);
    }
}

/// Logs an event which shows the current state of a `StaticBody` during the update step
pub fn update_static_body(static_body: &StaticBody) {
    if cfg!(feature="debug_renderevent") {
        println!("[FRAME] {}", static_body);
    }
}

/// Logs an event which shows the `Contact` found.
pub fn contact_found(contact: &Contact) {
    if cfg!(feature="debug_renderevent") {
        println!("[FRAME] {}", contact);
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

/// Serializes a point cloud to be rendered in the current frame. An ID should
/// be specified to help the test browser identify the same point cloud between
/// frames.
pub fn point_cloud(id: usize, points: &Vec<Vector>) {
    if cfg!(feature="debug_renderevent") {
        for point in points.iter() {
            println!("[FRAME] PointCloud[{}]: {}", id, point);
        }
    }
}

// TODO not yet supported in the test browser
// pub fn arrow(id: usize, points: (Vector, Vector)) {
//     if cfg!(feature="debug_renderevent") {
//         println!("[FRAME] Arrow[{}]: {} -> {}", id, points.0, points.1);
//     }
// }

/// Serializes a triangle mesh to be rendered in the current frame. An ID should
/// be specified to help the test browser identify the same triangle mesh
/// between frames.
pub fn triangle_mesh(id: usize, triangle_mesh: &Vec<(Vector, Vector, Vector)>) {
    if cfg!(feature="debug_renderevent") {
        for &(point_0, point_1, point_2) in triangle_mesh.iter() {
            println!("[FRAME] TriangleMesh[{}]: {} -> {} -> {}", id, point_0, point_1, point_2);
        }
    }
}
