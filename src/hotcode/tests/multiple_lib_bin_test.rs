mod build_utils;
use build_utils::*;

#[test]
fn hotreload_works() -> std::io::Result<()> {
    // Arrange
    const LIB_FOO_PACKAGE_NAME: &str = "lib_foo";
    const LIB_BAR_PACKAGE_NAME: &str = "lib_bar";
    const BIN_PACKAGE_NAME: &str = "multiple_bin";
    let target_dir = copy_test_project("multiple_lib_bin")?;
    let bin_dir = target_dir.as_ref().join(BIN_PACKAGE_NAME);

    let lib_foo_dir = target_dir.as_ref().join(LIB_FOO_PACKAGE_NAME);
    let target_foo_lib = lib_foo_dir.join("src").join("lib.rs");
    let updated_foo_lib = lib_foo_dir.join("src").join("lib_updated.rs");

    let lib_bar_dir = target_dir.as_ref().join(LIB_BAR_PACKAGE_NAME);
    let target_bar_lib = lib_bar_dir.join("src").join("lib.rs");
    let updated_bar_lib = lib_bar_dir.join("src").join("lib_updated.rs");
    cargo_clean_workspace_rebuild_in(target_dir.as_ref(), Profile::Dev)?;

    // Act 0 - start test process
    let mut test_run_process = cargo_run_in(&bin_dir, "multiple_bin", Profile::Dev)?;

    // Act 1 - Bar
    test_run_process.wait_for_input_from_stdout()?;
    std::fs::copy(updated_bar_lib, target_bar_lib)?;
    cargo_clean_workspace_rebuild_in(target_dir.as_ref(), Profile::Dev)?;
    test_run_process.send_enter_to_stdin()?;

    // Act 2 - Foo
    test_run_process.wait_for_input_from_stdout()?;
    std::fs::copy(updated_foo_lib, target_foo_lib)?;
    cargo_clean_workspace_rebuild_in(target_dir.as_ref(), Profile::Dev)?;
    test_run_process.send_enter_to_stdin()?;

    // Assert
    test_run_process.wait_for_successful_exit()?;
    Ok(())
}
