//! Library for hot-reloading functions in Rust.
//!
//! # Usage
//!
//! `hotcode` works only inside library crates. Typical supported project consists of a library
//! crate with runnable binaries, or an extra binary crate that uses your library.
//!
//! First, you need to add `hotcode` as dependency to your library crate.  
//! Then, add `cdylib` and `lib` to your library crate's types:
//! ```toml
//! // Cargo.toml
//! [lib]
//! crate-type = ["cdylib", "lib"]
//! ```
//!
//! Finally, just apply `#[hotreload]` attribute to function that you want to reload:
//! ```
//! // source code
//! #[hotcode::hotreload]
//! fn get() -> i32 {
//!     1
//! }
//! ```
//!
//! Now you can run your binary, apply changes to your library code, run `cargo build` and your
//! application will use updated library code without restarting.
//!
//! # Hotreload in `release` mode
//!
//! By default, `#[hotreload]` attribute works only with `debug_assertions` compiler flag so you can
//! keep this attribute even when building your application for production - it will do nothing in
//! this case. However, if you want to use hotreload even in `release` build, then you can pass
//! `always` argument to attribute like this: `#[hotreload(always)]`. This forces `hotcode` to
//! reload your library code even in `release` mode.
//! ```
//! #[hotcode::hotreload(always)]
//! fn get() -> i32 { 1 }
//! ```
//!
//! # How it works
//!
//! `hotcode` adds an `if` block at the beginning of affected functions. It checks whether currently
//! invoked function is loaded from dynamic library or is statically linked (`cdylib` and `lib`
//! parts of your `crate-types` config, respectively). If it is located in dynamic module, then the
//! body of your function is invoked, otherwise `hotcode` tries to lazily load your function from
//! dynamic module. On the first call it requires performing multiple initialization steps:
//! 1. perform sys-call to check whether entered function is from a dynamic module or is statically
//!    linked, cache result;
//! 2. create a copy of your crate's dynamic library file (so that it don't block source file for
//!    consequent rebuilds);
//! 3. start file watcher over copy of your dynamic library file in separate thread;
//! 4. load your crate as dynamic library from its copy on disk, store it to static cache;
//! 5. find your function's symbol in your dynamic library, store it to static cache;
//! 6. invoke your function loaded from dynamic library.
//!
//! All these steps can add milliseconds delay. However, as soon as library is loaded and pointer to
//! your function is cached, any subsequent calls will have very little overhead, because all that's
//! left to do is:
//! 1. read already initialized `LazyLock` bool static variable to determine whether entered
//!    function is in dynamically or statically linked module (no sys-calls performed)
//! 2. find your library in `IndexMap` cache by its `&str` name;
//! 3. find your function's pointer in `IndexMap` cache by its `&str` name;
//! 4. invoke your function loaded from dynamic library.
//!
//! This adds an approximately 100ns flat overhead on modern CPUs.
//!
//! When you rebuild your library, system notifies `hotcode`'s file watcher, and it reloads your
//! library in static cache from the newly copied version of your rebuilt library. Handles to
//! dynamic libraries are stored behind `Arc` and are atomically swapped during reload. Each
//! hot-reloadable function invocation creates `Arc` clone of dynamic library handle for the
//! duration of function execution. This ensures that dynamic library won't be unloaded from memory
//! while there are still functions using it.
//!
//! # Performance impact & benchmarks
//!
//! Impact on performance was measured by benching Fibonacci function with and without
//! `#[hotreload]` attribute. Two implementation were benched: one that uses recursion and is
//! relatively slow, and another that does not use recursion and is relatively fast.
//!
//! Results of benchmarking:
//!
//! | CPU \ implementation | slow, ns | hotreload slow, ns | fast, ns  | hotreload fast, ns  |
//! | -------------------- | -------- | ------------------ | --------- | ------------------- |
//! | i7-13700H            | 18814    | 18499              | 5.4211    | 98.726              |
//!
//! To run benchmarks locally, open `benchmarks` folder in source code repository root and run
//! `cargo build --release --lib` before running `cargo bench`.
//!
//! # Examples
//!
//! Examples of desired project structure and supported functions are located in `examples` folder
//! in source code repository root. The simplest example is the `single_crate` project.
pub use hotcode_core::{
    LibraryWrapper, get_platform_library_file_name, is_outside_dynamic_library, provide_fn,
    provide_library_wrapper,
};
pub use hotcode_proc_macro::hotreload;
