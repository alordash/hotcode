//! Library for hot-reloading functions in Rust.
//!
//! # Usage
//!
//! `hotcode` works only inside library crates. A typical supported project consists of a library
//! crate with runnable binaries, or an extra binary crate that uses your library.
//!
//! First, add `hotcode` as a dependency to your library crate.
//! Then, add `cdylib` and `lib` to your library's crate types:
//! ```toml
//! # Cargo.toml
//! [lib]
//! crate-type = ["cdylib", "lib"]
//! ```
//!
//! Finally, apply `#[hotreload]` attribute to the function that you want to reload:
//! ```
//! // source code
//! #[hotcode::hotreload]
//! fn get() -> i32 {
//!     1
//! }
//! ```
//!
//! Now you can run your binary, change your library code, run `cargo build`, and your
//! application will use updated library code without restarting.
//!
//! # Hotreload in `release` mode
//!
//! By default, `#[hotreload]` attribute works only when `debug_assertions` compiler flag is
//! enabled, so you can keep this attribute even when building your application for production - it
//! will do nothing in this case. However, if you want to use hotreload in `release` builds as well,
//! pass `always` argument to the attribute like this: `#[hotreload(always)]`. This forces
//! `hotcode` to reload your library code even in `release` mode.
//! ```
//! #[hotcode::hotreload(always)]
//! fn get() -> i32 { 1 }
//! ```
//!
//! # How it works
//!
//! `hotcode` adds an `if` block at the beginning of affected functions. It checks whether the
//! currently invoked function is loaded from a dynamic library or is statically linked (`cdylib`
//! and `lib` parts of your `crate-type` config, respectively). If it is located in a dynamic
//! module, then the body of your function is invoked; otherwise, `hotcode` tries to lazily load
//! your function from the dynamic module. The first call requires multiple initialization steps:
//! 1. perform a syscall to check whether the entered function is from a dynamic module or is
//!    statically linked, cache result;
//! 2. create a copy of your crate's dynamic library file (so that it doesn't block the source file
//!    for subsequent rebuilds);
//! 3. start a file watcher over the copy of your dynamic library file in a separate thread;
//! 4. load your crate as a dynamic library from its copy on disk, store it in a static cache;
//! 5. find your function's symbol in your dynamic library, store it in a static cache;
//! 6. invoke your function loaded from the dynamic library.
//!
//! All these steps can add a delay of several milliseconds. However, as soon as the library is
//! loaded and the pointer to your function is cached, any subsequent calls will have very little
//! overhead, because all that's left to do is:
//! 1. read already initialized `LazyLock` bool static variable to determine whether the entered
//!    function is in a dynamically or statically linked module (no syscalls performed);
//! 2. find your library in `IndexMap` cache by its `&str` name;
//! 3. find your function's pointer in `IndexMap` cache by its `&str` name;
//! 4. invoke your function loaded from the dynamic library.
//!
//! This adds a flat overhead of approximately 100ns on modern CPUs.
//!
//! When you rebuild your library, the system notifies `hotcode`'s file watcher, and it reloads your
//! library in the static cache from the newly copied version of your rebuilt library. Handles to
//! dynamic libraries are stored behind `Arc` and are atomically swapped during reload. Each
//! hot-reloadable function invocation creates `Arc` clone of the dynamic library handle for the
//! duration of the function's execution. This ensures that a dynamic library won't be unloaded
//! from memory while there are still functions using it.
//!
//! # Benchmarks
//!
//! The impact on performance was measured by benchmarking a Fibonacci function with and without
//! `#[hotreload]` attribute. Two implementations were benchmarked: one that uses recursion and
//! is relatively slow, and another that does not use recursion and is relatively fast.
//!
//! Benchmark results:
//!
//! | CPU \ implementation | slow, ns | hotreload slow, ns | fast, ns  | hotreload fast, ns  |
//! | -------------------- | -------- | ------------------ | --------- | ------------------- |
//! | i7-13700H            | 18814    | 18499              | 5.4211    | 98.726              |
//!
//! Slow implementation did not notice any changes, but fast implementation became ~19 times slower.
//!
//! To run benchmarks locally, open `benchmarks` folder in the source code repository root
//! and run `cargo build --release --lib` before running `cargo bench`.
//!
//! # Limitations
//!
//! 1. Cannot hot-reload functions with generics, because they are generally not supported in
//!    dynamic libraries.
//! 2. Changing function ABI without changing its call sites is Undefined Behavior. For example, if
//!    you have the following code:
//!    ```rust
//!    # use hotcode::hotreload;
//!    fn foo() { bar("warpten") }
//!
//!    #[hotreload]
//!    fn bar(str: &str) { println!("{str:?}") }
//!    ```
//!    and then change it to:
//!    ```rust
//!    # use hotcode::hotreload;
//!    fn foo() { bar(&[1, 2, 3]) }
//!
//!    #[hotreload]
//!    fn bar(array: &[i32]) { println!("{array:?}") }
//!    ```
//!    you will get UB because `foo` was not hot-reloaded - it still passes string instead of array.
//!
//! # Examples
//!
//! Examples of desired project structure and supported functions are located in `examples`
//! folder in the source code repository root. The simplest example is the `single_crate` project.
pub use hotcode_core::{
    LibraryWrapper, get_platform_library_file_name, is_outside_dynamic_library, provide_fn,
    provide_library_wrapper,
};
pub use hotcode_proc_macro::hotreload;
