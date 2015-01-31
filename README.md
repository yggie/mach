# Mithril Physics engine

An open source 3D physics engine. A reincarnation of my old physics engine:
[react](https://github.com/yggie/react) in Rust. Feature list is currently very
limited and the library is still in the experimentation phase.

# Documentation

The documentation can be viewed locally. Simply run the following command in the
project directory:

```
$ cargo doc --open
```

# Testing

Tests are written using the standard library tools. All are under the `tests/`
directory, and can be run using the command:

```
$ cargo test
```

# Compatibility

Rust still has not reached the 1.0 milestone at the time of writing, therefore,
the code may not compile without the appropriate Rust version. This is the
current Rust and Cargo version currently used to compile the project:

```
$ rustc --version
rustc 1.0.0-nightly (c5961ad06 2015-01-28 21:49:38 +0000)
$ cargo version
cargo 0.0.1-pre-nightly (453ae9f 2015-01-29 00:56:56 +0000)
```
