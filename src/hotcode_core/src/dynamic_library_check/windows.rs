#[cfg(test)]
use automock::*;
use winapi::shared::minwindef::*;
use winapi::um::libloaderapi::*;
use winapi::um::winnt::*;

#[cfg_attr(test, mock)]
#[allow(non_snake_case)]
#[inline(always)]
unsafe fn get_module_handle_ex_w(
    dwFlags: DWORD,
    lpModuleName: LPCWSTR,
    phModule: *mut HMODULE,
) -> BOOL {
    unsafe { GetModuleHandleExW(dwFlags, lpModuleName, phModule) }
}

#[cfg_attr(test, mock)]
#[allow(non_snake_case)]
#[inline(always)]
unsafe fn get_module_file_name_w(hModule: HMODULE, lpFilename: LPWSTR, nSize: DWORD) -> DWORD {
    unsafe { GetModuleFileNameW(hModule, lpFilename, nSize) }
}

#[cfg_attr(test, mock)]
#[inline(always)]
fn current_exe() -> std::io::Result<std::path::PathBuf> {
    std::env::current_exe()
}

const FLAGS: DWORD =
    GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT;
const FILE_NAME_BUFFER_LENGTH: usize = 4096;
pub fn slow_is_outside_dynamic_library() -> bool {
    let current_fn_ptr = slow_is_outside_dynamic_library as *const _;
    let mut h_module: HMODULE = core::ptr::null_mut();
    unsafe {
        if get_module_handle_ex_w(FLAGS, current_fn_ptr, &mut h_module) == 0 {
            return true;
        }
        if h_module.is_null() {
            return true;
        }
        let mut file_name_buffer = [0u16; FILE_NAME_BUFFER_LENGTH];
        let file_name_length = get_module_file_name_w(
            h_module,
            file_name_buffer.as_mut_ptr(),
            file_name_buffer.len() as DWORD,
        );
        if file_name_length == 0 {
            return true;
        }
        let Ok(current_exe_path) = current_exe() else {
            return true;
        };

        let module_name_bytes = &file_name_buffer[..file_name_length as usize];
        let module_name = String::from_utf16_lossy(module_name_bytes);
        dbg!(&current_exe_path);
        dbg!(&module_name);

        let result = current_exe_path.ends_with(module_name);
        return result;
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use std::path::*;

    #[test]
    fn slow_is_outside_dynamic_library_GetModuleHandleExWReturnsZero_ReturnsTrue() {
        // Arrange
        get_module_handle_ex_w::setup(Arg::Any, Arg::Any, Arg::Any).returns(0);

        // Act
        let result = slow_is_outside_dynamic_library();

        // Assert
        assert!(result);

        get_module_handle_ex_w::received(
            FLAGS,
            slow_is_outside_dynamic_library as *const _,
            Arg::Any,
            1.time(),
        )
        .no_other_calls();
    }

    #[test]
    fn slow_is_outside_dynamic_library_ReturnedModuleIsNull_ReturnsTrue() {
        // Arrange
        get_module_handle_ex_w::setup(Arg::Any, Arg::Any, Arg::Any)
            .returns(1)
            .and_does(|(_, _, h_module)| unsafe {
                **h_module = core::ptr::null_mut();
            });

        // Act
        let result = slow_is_outside_dynamic_library();

        // Assert
        assert!(result);

        get_module_handle_ex_w::received(
            FLAGS,
            slow_is_outside_dynamic_library as *const _,
            Arg::Any,
            1.time(),
        )
        .no_other_calls();
    }

    #[test]
    fn slow_is_outside_dynamic_library_ReturnedFileNameLengthIsZero_ReturnsTrue() {
        // Arrange
        let p_h_module = 1234 as HMODULE;
        get_module_handle_ex_w::setup(Arg::Any, Arg::Any, Arg::Any)
            .returns(1)
            .and_does(move |(_, _, h_module)| unsafe {
                **h_module = p_h_module;
            });

        get_module_file_name_w::setup(Arg::Any, Arg::Any, Arg::Any).returns(0);

        // Act
        let result = slow_is_outside_dynamic_library();

        // Assert
        assert!(result);

        get_module_handle_ex_w::received(
            FLAGS,
            slow_is_outside_dynamic_library as *const _,
            Arg::Any,
            1.time(),
        )
        .no_other_calls();

        get_module_file_name_w::received(
            p_h_module,
            Arg::Any,
            FILE_NAME_BUFFER_LENGTH as DWORD,
            1.time(),
        )
        .no_other_calls();
    }

    #[test]
    fn slow_is_outside_dynamic_library_ReturnedCurrentExeIsError_ReturnsTrue() {
        // Arrange
        let p_h_module = 1234 as HMODULE;
        get_module_handle_ex_w::setup(Arg::Any, Arg::Any, Arg::Any)
            .returns(1)
            .and_does(move |(_, _, h_module)| unsafe {
                **h_module = p_h_module;
            });

        get_module_file_name_w::setup(Arg::Any, Arg::Any, Arg::Any).returns(1);

        current_exe::setup().returns(Err(std::io::Error::other("whatever")));

        // Act
        let result = slow_is_outside_dynamic_library();

        // Assert
        assert!(result);

        get_module_handle_ex_w::received(
            FLAGS,
            slow_is_outside_dynamic_library as *const _,
            Arg::Any,
            1.time(),
        )
        .no_other_calls();

        get_module_file_name_w::received(
            p_h_module,
            Arg::Any,
            FILE_NAME_BUFFER_LENGTH as DWORD,
            1.time(),
        )
        .no_other_calls();

        current_exe::received(1.time()).no_other_calls();
    }

    #[test]
    fn slow_is_outside_dynamic_library_CurrentExeEndsWithModuleName_ReturnsTrue() {
        // Arrange
        let p_h_module = 1234 as HMODULE;
        get_module_handle_ex_w::setup(Arg::Any, Arg::Any, Arg::Any)
            .returns(1)
            .and_does(move |(_, _, h_module)| unsafe {
                **h_module = p_h_module;
            });

        let module_name = "quo vadis";
        let module_name_utf16: Vec<u16> = module_name.encode_utf16().collect();
        get_module_file_name_w::setup(Arg::Any, Arg::Any, Arg::Any)
            .returns(module_name_utf16.len() as DWORD)
            .and_does(move |(_, file_name_buffer_ptr, _)| unsafe {
                core::ptr::copy_nonoverlapping(
                    module_name_utf16.as_ptr(),
                    *file_name_buffer_ptr,
                    module_name_utf16.len(),
                );
            });

        let current_exe = format!("veridis quo/{module_name}");
        current_exe::setup().returns(Ok(PathBuf::from(current_exe)));

        // Act
        let result = slow_is_outside_dynamic_library();

        // Assert
        assert!(result);

        get_module_handle_ex_w::received(
            FLAGS,
            slow_is_outside_dynamic_library as *const _,
            Arg::Any,
            1.time(),
        )
        .no_other_calls();

        get_module_file_name_w::received(
            p_h_module,
            Arg::Any,
            FILE_NAME_BUFFER_LENGTH as DWORD,
            1.time(),
        )
        .no_other_calls();

        current_exe::received(1.time()).no_other_calls();
    }

    #[test]
    fn slow_is_outside_dynamic_library_CurrentExeDoesNotEndWithModuleName_ReturnsFalse() {
        // Arrange
        let p_h_module = 1234 as HMODULE;
        get_module_handle_ex_w::setup(Arg::Any, Arg::Any, Arg::Any)
            .returns(1)
            .and_does(move |(_, _, h_module)| unsafe {
                **h_module = p_h_module;
            });

        let module_name_utf16: Vec<u16> = "quo vadis".encode_utf16().collect();
        get_module_file_name_w::setup(Arg::Any, Arg::Any, Arg::Any)
            .returns(module_name_utf16.len() as DWORD)
            .and_does(move |(_, file_name_buffer_ptr, _)| unsafe {
                core::ptr::copy_nonoverlapping(
                    module_name_utf16.as_ptr(),
                    *file_name_buffer_ptr,
                    module_name_utf16.len(),
                );
            });

        current_exe::setup().returns(Ok(PathBuf::from("veridis quo")));

        // Act
        let result = slow_is_outside_dynamic_library();

        // Assert
        assert!(!result);

        get_module_handle_ex_w::received(
            FLAGS,
            slow_is_outside_dynamic_library as *const _,
            Arg::Any,
            1.time(),
        )
        .no_other_calls();

        get_module_file_name_w::received(
            p_h_module,
            Arg::Any,
            FILE_NAME_BUFFER_LENGTH as DWORD,
            1.time(),
        )
        .no_other_calls();

        current_exe::received(1.time()).no_other_calls();
    }
}
