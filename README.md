# hotcode

Library for hot-reloading functions in Rust.

[![Build Status](https://github.com/alordash/hotcode/actions/workflows/ci.yml/badge.svg)](https://github.com/alordash/hotcode/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/hotcode.svg)](https://crates.io/crates/hotcode)
[![Documentation](https://docs.rs/hotcode/badge.svg)](https://docs.rs/hotcode)

## Overview

This library exposes `hotreload` attribute that can be applied to standalone or implementation functions. This attribute
makes function hot-reloadable: it's code and behavior can be changed on the fly while the application is still running.

💻 Works on **Linux**, **macOS** and **Windows**.

## Usage

Add `hotcode` to your `dependencies`:

```toml
[dependencies]
hotcode = "0.1.1"
```

Add `cdylib` to your library crate types:

```toml
[lib]
crate-type = ["cdylib", "lib"]
```

Add `hotcode::hotreload` attribute to your function:

```rust
#[hotcode::hotreload]
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Run your application, change the code of this function and rebuild your library using `cargo build --lib`. The function
behavior will change without application restart.

For more information refer to [crate documentation](https://docs.rs/hotcode).

## Examples

Examples are located in the [`examples`](examples) folder. The simplest example is the
[`single_crate`](examples/single_crate) project.

# Minimum Supported Rust Version (MSRV)

`hotcode` is supported on Rust 1.89.0 and higher. `hotcode`'s MSRV will not be changed in the future without bumping the
major or minor version.

# License

`hotcode` is distributed under the terms of MIT license. See [license.txt](license.txt) for details.
