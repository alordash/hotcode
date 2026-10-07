# Supported functions example

This example demonstrates various functions that can be hot-reloaded by `hotcode`.

Run project with `cargo run`, then make changes to [`src/lib.rs`](src/lib.rs) and rebuild library with
`cargo build --lib` without stopping running binary. You should see new values in console output:

```
...
[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]
# code change and rebuild
[1, 1, 1, 1, 1, 1, 1, 1, 1, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22]
...
```