use crate::LibraryWrapper;
use crate::static_library::DYNAMIC_LIBRARIES_MAP;
use arc_swap::ArcSwap;
use notify_debouncer_full::{DebounceEventResult, notify};
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
            let events_paths = debounced_events
                .into_iter()
                .filter_map(|x| match x.event.kind {
                    notify::EventKind::Create(_) | notify::EventKind::Modify(_) => {
                        Some(x.event.paths)
                    }
                    _ => None,
                });
            let current_lib_path = current_lib.library_copy_path();
            if !library_was_changed(current_lib_path, static_library_path, events_paths) {
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

fn library_was_changed(
    current_lib_path: &Path,
    static_library_path: &Path,
    events_paths: impl Iterator<Item = Vec<PathBuf>>,
) -> bool {
    let mut lib_was_changed = false;
    for event_paths in events_paths {
        for path in event_paths.into_iter() {
            if path == current_lib_path {
                return false;
            } else if path == static_library_path {
                lib_was_changed = true;
            }
        }
    }
    return lib_was_changed;
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn library_was_changed_AnyPathIsCurrentLibPath_ReturnsFalse() {
        // Arrange
        let current_lib_path = Path::new("current_lib_path");
        let static_library_path = Path::new("static_library_path");
        let events_paths = vec![
            vec![PathBuf::from("1"), PathBuf::from("2"), PathBuf::from("3")],
            vec![
                PathBuf::from("4"),
                current_lib_path.to_owned(),
                PathBuf::from("6"),
            ],
        ];

        // Act
        let result = library_was_changed(
            current_lib_path,
            static_library_path,
            events_paths.into_iter(),
        );

        // Assert
        assert!(!result);
    }

    #[test]
    fn library_was_changed_NonePathIsEqualToStaticLibraryPath_ReturnsFalse() {
        // Arrange
        let current_lib_path = Path::new("current_lib_path");
        let static_library_path = Path::new("static_library_path");
        let events_paths = vec![
            vec![PathBuf::from("1"), PathBuf::from("2")],
            vec![PathBuf::from("3")],
        ];

        // Act
        let result = library_was_changed(
            current_lib_path,
            static_library_path,
            events_paths.into_iter(),
        );

        // Assert
        assert!(!result);
    }

    #[test]
    fn library_was_changed_SomePathIsStaticLibraryPathAndAnyPathIsCurrentLibPath_ReturnsFalse() {
        // Arrange
        let current_lib_path = Path::new("current_lib_path");
        let static_library_path = Path::new("static_library_path");
        let events_paths = vec![
            vec![
                PathBuf::from("1"),
                static_library_path.to_owned(),
                PathBuf::from("3"),
            ],
            vec![
                PathBuf::from("4"),
                current_lib_path.to_owned(),
                PathBuf::from("6"),
            ],
        ];

        // Act
        let result = library_was_changed(
            current_lib_path,
            static_library_path,
            events_paths.into_iter(),
        );

        // Assert
        assert!(!result);
    }

    #[test]
    fn library_was_changed_SomePathIsStaticLibraryPathAndNonePathIsCurrentLibPath_ReturnsTrue() {
        // Arrange
        let current_lib_path = Path::new("current_lib_path");
        let static_library_path = Path::new("static_library_path");
        let events_paths = vec![
            vec![
                PathBuf::from("1"),
                static_library_path.to_owned(),
                PathBuf::from("3"),
            ],
            vec![PathBuf::from("4"), PathBuf::from("5"), PathBuf::from("6")],
        ];

        // Act
        let result = library_was_changed(
            current_lib_path,
            static_library_path,
            events_paths.into_iter(),
        );

        // Assert
        assert!(result);
    }
}
