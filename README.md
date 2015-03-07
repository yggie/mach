# Mithril Physics engine

An open source 3D physics engine. A reincarnation of my old physics engine:
[react](https://github.com/yggie/react) in Rust. Feature list is currently very
limited and the library is still in the experimentation phase, therefore I do
not recommend using the library. If you are looking for a physics engine in
Rust, have a look at [nphysics](https://github.com/sebcrozet/nphysics) which I
use as my reference implementation.

# Motivation

I have always wanted to build a physics engine, and with Rust maturing into an
amazing language, I wanted to put the two things together. So I came up with
this crazy idea to write a physics engine in Rust!

The project is also an exercise in software architecture and testing applied to
a high performance and unpredictable application. I am a strong believer in
test driven development and transparent software architecture, and applying
those beliefs to a physics engine is probably one of my biggest challenge yet! I
try hard to apply my experience in testing software to drive the physics engine
forward (something you don’t see very often in this field), and also to keep the
application architecture sane and not expose too much of the implementation.
With all these in mind, the project is progressing much slower than I would
like! But hopefully it will all pay off later. It is my hope that with a
transparent, solid and well tested foundation, this project could eventually be
completely self supported by the community.

# Documentation

The documentation can be viewed locally. Simply run the following command in the
project directory:

```
cargo doc --open
```

# Testing

Tests are written using the standard library tools. All are under the `tests/`
directory, and can be run using the command:

```
cargo test
```

# Compatibility

As many of us know, Rust has still not reached a stable version yet. Code that
compiled a week ago may not compile with today’s Rust version. Before trying to
compile the code, please ensure you have the most up to date Rust version. This
is the version that the library has been compiled with:

```
$ rustc --version
rustc 1.0.0-beta (9854143cb 2015-04-02) (built 2015-04-02)
$ cargo version
cargo 0.0.1-pre-nightly (84d6d2c 2015-03-31) (built 2015-03-31)
```


# License

This software is distributed under the
[MIT License](https://github.com/yggie/mithril/blob/master/LICENSE).
