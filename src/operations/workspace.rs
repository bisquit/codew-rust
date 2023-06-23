use std::process::Command;

pub fn open_workspace() {
    Command::new("code")
        .arg(".")
        .spawn()
        .expect("failed");
}
