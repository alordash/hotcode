use std::path::{Path, PathBuf};
use tempfile::{TempDir, tempdir};
use test_project_process::*;

const CARGO_MANIFEST_DIR_VAR_NAME: &str = "CARGO_MANIFEST_DIR";

pub fn copy_test_project(test_project_name: &'static str) -> std::io::Result<TempDir> {
    let hotcode_dir = PathBuf::from(std::env::var(CARGO_MANIFEST_DIR_VAR_NAME).unwrap_or_else(|e| {
        panic!(
            "Unable to read project root path from env var '{CARGO_MANIFEST_DIR_VAR_NAME}': {e:?}"
        )
    }));
    let source_dir = hotcode_dir.join("tests").join(test_project_name);
    let target_dir = tempdir()?;

    dircpy::copy_dir(source_dir, &target_dir)?;

    let target_toml = target_dir.path().join("Cargo.toml");
    let target_toml_content = std::fs::read_to_string(&target_toml)?;
    let new_target_toml_content = target_toml_content.replace(
        r#"hotcode = { path = "../.." }"#,
        &format!(r#"hotcode = {{ path = {hotcode_dir:?} }}"#),
    );
    std::fs::write(target_toml, new_target_toml_content)?;

    return Ok(target_dir);
}

#[allow(unused)]
pub fn cargo_clean_rebuild_in(
    target_dir: &Path,
    test_project_name: &'static str,
    profile: Profile,
) -> std::io::Result<()> {
    let mut clean_args = vec!["clean", "-p", test_project_name];
    match profile {
        Profile::Dev => {}
        Profile::Release => {
            clean_args.push("--release");
        }
    }
    std::process::Command::new("cargo")
        .args(clean_args)
        .current_dir(target_dir)
        .status()?;
    let mut build_args = vec!["build", "--lib"];
    match profile {
        Profile::Dev => {}
        Profile::Release => {
            build_args.push("--release");
        }
    }
    std::process::Command::new("cargo")
        .args(build_args)
        .current_dir(target_dir)
        .status()?;
    Ok(())
}

#[allow(unused)]
pub fn cargo_clean_workspace_rebuild_in(
    target_dir: &Path,
    profile: Profile,
) -> std::io::Result<()> {
    let mut clean_args = vec!["clean", "--workspace"];
    match profile {
        Profile::Dev => {}
        Profile::Release => {
            clean_args.push("--release");
        }
    }
    std::process::Command::new("cargo")
        .args(clean_args)
        .current_dir(target_dir)
        .status()?;
    let mut build_args = vec!["build", "--lib"];
    match profile {
        Profile::Dev => {}
        Profile::Release => {
            build_args.push("--release");
        }
    }
    std::process::Command::new("cargo")
        .args(build_args)
        .current_dir(target_dir)
        .status()?;
    Ok(())
}

pub fn cargo_run_in(
    target_dir: &Path,
    bin_name: &str,
    profile: Profile,
) -> std::io::Result<TestProjectProcess> {
    let mut args = vec!["run", "--bin", bin_name];
    match profile {
        Profile::Dev => {}
        Profile::Release => {
            args.push("--release");
        }
    }
    let run_process = std::process::Command::new("cargo")
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .current_dir(target_dir)
        .spawn()?;
    let test_project_process = TestProjectProcess::new(run_process);
    return Ok(test_project_process);
}

mod test_project_process {
    use std::io::{Read, Write};

    pub struct TestProjectProcess {
        child: std::process::Child,
    }

    impl TestProjectProcess {
        pub fn new(child: std::process::Child) -> Self {
            Self { child }
        }

        pub fn wait_for_input_from_stdout(&mut self) -> std::io::Result<()> {
            let stdout = self
                .child
                .stdout
                .as_mut()
                .expect("Test project process must have stdout");
            stdout.read_exact(&mut [0u8])
        }

        pub fn send_enter_to_stdin(&mut self) -> std::io::Result<()> {
            let stdin = self
                .child
                .stdin
                .as_mut()
                .expect("Test project process must have stdin");
            stdin.write_all(b"\n")?;
            stdin.flush()
        }

        pub fn wait_for_successful_exit(self) -> std::io::Result<()> {
            let output = self.child.wait_with_output()?;
            if !output.status.success() {
                panic!(
                    "Error running target bin, exit code: {:?}",
                    output.status.code()
                );
            }
            Ok(())
        }
    }
}

#[allow(unused)]
pub enum Profile {
    Dev,
    Release,
}
