pub mod utils;

use std::process::Command;

use assert_cmd::assert::OutputAssertExt;
use utils::{Workspace, PROGRAM};

#[test]
fn from_dockerfile() {
    let workspace = Workspace::new(
        "
        missions:
            hello-world:
                build: .
                script: true",
    )
    .with_dockerfile("FROM busybox");

    Command::new(PROGRAM)
        .arg("execute")
        .current_dir(&workspace.0)
        .assert()
        .success();
}

#[test]
fn from_recipe() {
    let workspace = Workspace::new(
        "
        missions:
            hello-world:
                recipe: FROM busybox
                script: true",
    );

    Command::new(PROGRAM)
        .arg("execute")
        .current_dir(&workspace.0)
        .assert()
        .success();
}
