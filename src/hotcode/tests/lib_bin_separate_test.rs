mod build_utils;
use build_utils::*;

#[test]
fn hotreload_works() -> std::io::Result<()> {
    // Arrange
    const LIB_PACKAGE_NAME: &str = "lib_separate";
    const BIN_PACKAGE_NAME: &str = "bin_separate";
    let target_dir = copy_test_project("lib_bin_separate")?;
    let lib_dir = target_dir.as_ref().join(LIB_PACKAGE_NAME);
    let bin_dir = target_dir.as_ref().join(BIN_PACKAGE_NAME);
    let target_lib = lib_dir.join("src").join("lib.rs");
    let updated_lib = lib_dir.join("src").join("lib_updated.rs");
    cargo_clean_rebuild_in(&lib_dir, LIB_PACKAGE_NAME, Profile::Dev)?;

    // Act
    let mut test_run_process = cargo_run_in(&bin_dir, "bin_separate", Profile::Dev)?;
    test_run_process.wait_for_input_from_stdout()?;
    std::fs::copy(updated_lib, target_lib)?;
    cargo_clean_rebuild_in(&lib_dir, LIB_PACKAGE_NAME, Profile::Dev)?;
    test_run_process.send_enter_to_stdin()?;

    // Assert
    test_run_process.wait_for_successful_exit()?;
    Ok(())
}
