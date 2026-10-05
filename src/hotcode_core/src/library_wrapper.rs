use super::*;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};
use std::time::Instant;

pub struct LibraryWrapper {
    library_copy_path: PathBuf,
    fn_ptrs_map: GrowMap<&'static [u8], *mut core::ffi::c_void>,
    maybe_inner: Option<libloading::Library>,
}

static TIME_START: LazyLock<Instant> = LazyLock::new(Instant::now);

impl LibraryWrapper {
    pub fn new(library_source_path: PathBuf) -> Self {
        let tick_stamp = TIME_START.elapsed().as_millis();
        let library_copy_path = {
            let mut library_copy_path_base = library_source_path.clone();
            library_copy_path_base.set_extension("");
            let mut library_source_path_string = library_copy_path_base.into_os_string();
            library_source_path_string.push("_");
            library_source_path_string.push(tick_stamp.to_string());
            let mut result_library_copy_path: PathBuf = library_source_path_string.into();
            if let Some(source_extension) = library_source_path.extension() {
                result_library_copy_path.set_extension(source_extension);
            }
            result_library_copy_path
        };
        std::fs::copy(&library_source_path, &library_copy_path)
            .unwrap_or_else(|e| panic!(
                "Unable to copy library with tick stamp from {library_source_path:?} to {library_copy_path:?}: {e:?}"));

        let inner = unsafe {
            libloading::Library::new(library_copy_path.clone()).unwrap_or_else(|e| {
                panic!("Error opening shared library {library_copy_path:?} in directory {library_copy_path:?}: {e:?}")
            })
        };
        let result = Self {
            library_copy_path,
            fn_ptrs_map: GrowMap::new(),
            maybe_inner: Some(inner),
        };

        return result;
    }

    pub fn library_copy_path(&self) -> &Path {
        &self.library_copy_path
    }

    pub fn get<F>(self: Arc<Self>, symbol_name: &'static [u8]) -> FnGuard<F> {
        let raw_fn_ptr = self.fn_ptrs_map.get_or_insert_with(symbol_name, || unsafe {
            let raw_fn_ptr = self
                .maybe_inner
                .as_ref()
                .expect("Dynamic library must be loaded into wrapper.")
                .get::<fn()>(symbol_name)
                .unwrap_or_else(|e| {
                    panic!(
                        "Unable to find function with symbol name '{}': {:?}",
                        String::from_utf8_lossy(symbol_name),
                        e
                    )
                })
                .try_as_raw_ptr()
                .unwrap_or_else(|| {
                    panic!(
                        "Unable to convert symbol '{}' to raw ptr: empty",
                        String::from_utf8_lossy(symbol_name)
                    )
                });
            return raw_fn_ptr;
        });
        let fn_ptr = raw_fn_ptr.cast::<()>();
        let r#fn = unsafe { core::mem::transmute_copy(&fn_ptr) };
        let result = FnGuard::new(r#fn, self.clone());
        return result;
    }
}

impl Drop for LibraryWrapper {
    fn drop(&mut self) {
        if let Some(inner) = self.maybe_inner.take()
            && let Err(e) = inner.close()
        {
            println!(
                "[WARNING] Unable to close dynamic library '{:?}': {:?}",
                self.library_copy_path, e
            );
        }

        if let Err(e) = std::fs::remove_file(&self.library_copy_path) {
            println!(
                "[WARNING] Unable to clear copy of dynamic library '{:?}': {:?}",
                self.library_copy_path, e
            );
        }
    }
}
