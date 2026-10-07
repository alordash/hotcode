use std::sync::LazyLock;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod unix;

#[cfg(windows)]
static IS_OUTSIDE_DYNAMIC_LIBRARY: LazyLock<bool> =
    LazyLock::new(windows::slow_is_outside_dynamic_library);

#[cfg(unix)]
static IS_OUTSIDE_DYNAMIC_LIBRARY: LazyLock<bool> =
    LazyLock::new(unix::slow_is_outside_dynamic_library);

#[doc(hidden)]
pub fn is_outside_dynamic_library() -> bool {
    *IS_OUTSIDE_DYNAMIC_LIBRARY
}
