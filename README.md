# Mach Physics engine

[![Build Status](https://travis-ci.org/yggie/mach.svg?branch=master)](https://travis-ci.org/yggie/mach)
[![Clippy Linting Result](https://clippy.bashy.io/github/yggie/mach/master/badge.svg)](https://clippy.bashy.io/github/yggie/mach/master/log)
[![License](https://img.shields.io/badge/license-MIT-yellow.svg)](#license)

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

Tests are compiled and run using `Cargo`:

```
cargo test
```

# License

This software is distributed under the [MIT License](LICENSE).
