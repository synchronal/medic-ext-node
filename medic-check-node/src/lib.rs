#![cfg_attr(feature = "strict", deny(warnings))]

pub mod cli;

use medic_lib::CheckResult::{self, CheckError, CheckOk};
use medic_lib::std_to_string;
use std::process::Command;

pub fn npm_exists() -> CheckResult {
    match Command::new("which").args(["npm"]).output() {
        Ok(which) => {
            if which.status.success() {
                CheckOk
            } else {
                let stdout = std_to_string(which.stdout);
                let stderr = std_to_string(which.stderr);
                CheckError(
                    "Unable to find npm.".into(),
                    Some(stdout),
                    Some(stderr),
                    Some("asdf install nodejs".into()),
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to search for npm. Is `which` in your PATH?".into(),
            None,
            None,
            None,
        ),
    }
}

pub fn packages_installed(cd: Option<String>, prefix: Option<String>) -> CheckResult {
    let mut check = Command::new("npm");
    let mut remedy = Command::new("npm");
    check.arg("ls").arg("--prefer-offline");
    remedy.arg("install");

    if let Some(path) = cd {
        if let Ok(expanded) = std::fs::canonicalize(path) {
            check.current_dir(&expanded);
            remedy.current_dir(&expanded);
        } else {
            return CheckError(
                "Given a `cd` param to a directory that does not exist.".into(),
                None,
                None,
                None,
            );
        }
    }
    if let Some(path) = prefix {
        check.arg("--prefix").arg(&path);
        remedy.arg("--prefix").arg(&path);
    }

    match check.output() {
        Ok(output) => {
            let stdout = std_to_string(output.stdout);
            let stderr = std_to_string(output.stderr);
            if output.status.success() && !stdout.contains("UNMET DEPENDENCY") {
                CheckOk
            } else {
                CheckError(
                    "NPM dependencies out of date.".into(),
                    Some(stdout),
                    Some(stderr),
                    Some(format!("({remedy:?})")),
                )
            }
        }
        Err(_err) => CheckError(
            "Unable to determine which NPM packages are installed.".into(),
            None,
            None,
            Some("asdf install nodejs".into()),
        ),
    }
}

pub fn corepack_shim_installed(name: String, version: String) -> CheckResult {
    enable_corepack()?;
    corepack_shim_in_path(&name, &version)?;
    corepack_shim_version_matches(&name, &version)
}

fn corepack_shim_in_path(name: &String, version: &String) -> CheckResult {
    let shim = format!("{name}@{version}");
    let mut remedy = Command::new("corepack");
    remedy.args(["install", "--global", &shim]);

    match Command::new("which").args([name]).output() {
        Ok(output) => {
            if output.status.success() {
                CheckOk
            } else {
                CheckError(
                    "Node corepack shim not installed.".into(),
                    None,
                    None,
                    Some(format!("({remedy:?})")),
                )
            }
        }
        Err(_err) => CheckError(
            "Node corepack shim not installed.".into(),
            None,
            None,
            Some(format!("({remedy:?})")),
        ),
    }
}

fn corepack_shim_version_matches(name: &String, version: &String) -> CheckResult {
    let shim = format!("{name}@{version}");
    let mut remedy = Command::new("corepack");
    remedy.args(["install", "--global", &shim]);

    let output = Command::new(name).args(["--version"]).output().unwrap();
    let stdout = std_to_string(output.stdout);
    let stderr = std_to_string(output.stderr);

    if output.status.success() && str::trim(&stdout).eq(version) {
        CheckOk
    } else {
        CheckError(
            "Node corepack shim out of date.".into(),
            Some(stdout),
            Some(stderr),
            Some(format!("({remedy:?})")),
        )
    }
}

fn enable_corepack() -> CheckResult {
    match Command::new("corepack").arg("enable").output() {
        Ok(_) => CheckOk,
        Err(error) => CheckError(
            "Unable to enable corepack".into(),
            None,
            Some(format!("{error}")),
            None,
        ),
    }
}
