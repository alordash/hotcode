# External library example

This example demonstrates how to use `hotcode::hotreload` in libraries that are imported as external dependencies by
crates that have runnable binaries.

Run project with `cargo run` from this directory, then make changes to [`library/src/lib.rs`](library/src/lib.rs)
and rebuild library with `cargo build --lib` without stopping running binary. You should see new value in
console output:

```
...
[binary\src\main.rs:6:9] result = 2
# code change and rebuild
[binary\src\main.rs:6:9] result = 4
...
```