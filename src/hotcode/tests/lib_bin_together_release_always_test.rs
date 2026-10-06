mod build_utils;
use build_utils::*;

#[test]
fn hotreload_with_always_argument_works_in_release() -> std::io::Result<()> {
    // Arrange
    const PACKAGE_NAME: &str = "lib_bin_together_release_always";
    let target_dir = copy_test_project(PACKAGE_NAME)?;
    let target_lib = target_dir.as_ref().join("src").join("lib.rs");
    let updated_lib = target_dir.as_ref().join("src").join("lib_updated.rs");
    cargo_clean_rebuild_in(target_dir.as_ref(), PACKAGE_NAME, Profile::Release)?;

    // Act
    let mut test_run_process = cargo_run_in(
        target_dir.as_ref(),
        "bin_together_release_always",
        Profile::Release,
    )?;
    test_run_process.wait_for_input_from_stdout()?;
    std::fs::copy(updated_lib, target_lib)?;
    cargo_clean_rebuild_in(target_dir.as_ref(), PACKAGE_NAME, Profile::Release)?;
    test_run_process.send_enter_to_stdin()?;

    // Assert
    test_run_process.wait_for_successful_exit()?;
    Ok(())
}
