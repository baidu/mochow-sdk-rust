use std::process::Command;

fn main() {
    Command::new("cargo").args(&["fmt"]).output().unwrap();
}
