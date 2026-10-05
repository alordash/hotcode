#[cfg(test)]
use automock::*;
use std::ffi::c_void;

#[cfg_attr(test, mock)]
#[inline(always)]
unsafe fn dladdr(addr: *const c_void, info: *mut libc::Dl_info) -> libc::c_int {
    libc::dladdr(addr, info)
}

#[cfg_attr(test, mock)]
#[inline(always)]
fn current_exe() -> std::io::Result<std::path::PathBuf> {
    std::env::current_exe()
}

pub fn slow_is_outside_dynamic_library() -> bool {
    let mut info = libc::Dl_info {
        dli_fname: core::ptr::null(),
        dli_fbase: core::ptr::null_mut(),
        dli_sname: core::ptr::null(),
        dli_saddr: core::ptr::null_mut(),
    };

    let current_fn_ptr = slow_is_outside_dynamic_library as *const c_void;

    unsafe {
        if dladdr(current_fn_ptr, &mut info) == 0 {
            return true;
        }
        if info.dli_fname.is_null() {
            return true;
        }
        let Ok(current_exe_path) = std::env::current_exe() else {
            return true;
        };

        let module_name_bytes = std::ffi::CStr::from_ptr(info.dli_fname).to_bytes();
        let result = current_exe_path
            .as_os_str()
            .as_encoded_bytes()
            .ends_with(module_name_bytes);
        return result;
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use std::path::*;

    #[test]
    fn slow_is_outside_dynamic_library_dladdrReturnsZero_ReturnsTrue() {
        // Arrange
        dladdr::setup(Arg::Any, Arg::Any).returns(0);

        // Act
        let result = slow_is_outside_dynamic_library();

        // Assert
        assert!(result);

        dladdr::received(
            slow_is_outside_dynamic_library as *const c_void,
            Arg::is(Dl_info_is_uninitialized),
            1.time(),
        )
        .no_other_calls();
    }

    #[test]
    fn slow_is_outside_dynamic_library_dliFNameIsNull() {
        // Arrange
        dladdr::setup(Arg::Any, Arg::Any)
            .returns(1)
            .and_does(|(_, info)| {
                info.dli_fname = core::ptr::null();
            });

        // Act
        let result = slow_is_outside_dynamic_library();

        // Assert
        assert!(result);

        dladdr::received(
            slow_is_outside_dynamic_library as *const c_void,
            Arg::is(Dl_info_is_uninitialized),
            1.time(),
        )
        .no_other_calls();
    }

    #[test]
    fn slow_is_outside_dynamic_library_CurrentExeEndsWithModuleName_ReturnsTrue() {
        // Arrange
        let module_name = b"quo vadis\0";
        dladdr::setup(Arg::Any, Arg::Any)
            .returns(1)
            .and_does(move |(_, info)| {
                info.dli_fname = module_name;
            });

        let current_exe = format!("veridis quo/{module_name}");
        current_exe::setup().returns(Ok(PathBuf::from(current_exe)));

        // Act
        let result = slow_is_outside_dynamic_library();

        // Assert
        assert!(result);

        dladdr::received(
            slow_is_outside_dynamic_library as *const c_void,
            Arg::is(Dl_info_is_uninitialized),
            1.time(),
        )
        .no_other_calls();
    }

    #[test]
    fn slow_is_outside_dynamic_library_CurrentExeDoesNotEndWithModuleName_ReturnsFalse() {
        // Arrange
        dladdr::setup(Arg::Any, Arg::Any)
            .returns(1)
            .and_does(move |(_, info)| {
                info.dli_fname = b"quo vadis\0";
            });

        current_exe::setup().returns(Ok(PathBuf::from("veridis quo")));

        // Act
        let result = slow_is_outside_dynamic_library();

        // Assert
        assert!(result);

        dladdr::received(
            slow_is_outside_dynamic_library as *const c_void,
            Arg::is(Dl_info_is_uninitialized),
            1.time(),
        )
        .no_other_calls();
    }

    fn Dl_info_is_uninitialized(dl_info: &libc::Dl_info) -> bool {
        dl_info.dli_fname.is_null()
            && dl_info.dli_fbase.is_null()
            && dl_info.dli_sname.is_null()
            && dl_info.dli_saddr.is_null()
    }
}
