use crate::LibraryWrapper;
use crate::static_library::DYNAMIC_LIBRARIES_MAP;
use arc_swap::ArcSwap;
use notify_debouncer_full::{DebounceEventResult, DebouncedEvent, notify};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

pub fn spawn(
    library_file_name_string: String,
    library_path: PathBuf,
    shared_library_wrapper: Arc<ArcSwap<LibraryWrapper>>,
) {
    let static_library_path: &Path = library_path.leak();
    let mut watcher = notify_debouncer_full::new_debouncer(
        Duration::from_millis(100),
        None,
        move |events_result: DebounceEventResult| {
            let Some(current_lib) = DYNAMIC_LIBRARIES_MAP
                .get(&library_file_name_string)
                .map(|x| x.load())
            else {
                return;
            };
            let debounced_events = events_result.unwrap_or_else(|e| {
                panic!("Error handling debounced library file update event: {e:?}")
            });
            let current_lib_path = current_lib.library_copy_path();
            if !library_was_changed(debounced_events, current_lib_path, static_library_path) {
                return;
            }
            let new_library = LibraryWrapper::new(static_library_path.to_owned());
            shared_library_wrapper.store(Arc::new(new_library));
        },
    )
    .unwrap();

    let parent_library_path = static_library_path.parent().unwrap_or_else(|| {
        panic!("Unable to get parent directory of library path {static_library_path:?}.")
    });
    watcher
        .watch(parent_library_path, notify::RecursiveMode::NonRecursive)
        .unwrap();

    core::mem::forget(watcher);
}

// TODO - test it
fn library_was_changed(
    debounced_events: Vec<DebouncedEvent>,
    current_lib_path: &Path,
    static_library_path: &Path,
) -> bool {
    let mut lib_was_changed = false;
    for paths in debounced_events
        .into_iter()
        .filter_map(|x| match x.event.kind {
            notify::EventKind::Create(_) | notify::EventKind::Modify(_) => Some(x.event.paths),
            _ => None,
        })
    {
        for path in paths.into_iter() {
            if path == current_lib_path {
                return false;
            } else if path == static_library_path {
                lib_was_changed = true;
            }
        }
    }
    return lib_was_changed;
}
