mod contact;

pub mod gjkepa;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod detection;

pub use self::contact::Contact;
pub use self::detection::Detection;
