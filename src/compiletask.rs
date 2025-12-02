use std::process::Command;

pub fn compiletobinary(name: &str) {
    Command::new("gcc")
        .arg(format!("{name}.c"))
        .arg("-O3")
        .arg("-march=native")
        .arg("-flto")
        .arg("-o")
        .arg(format!("{name}"))
        .status()
        .expect("failed to compile");
    }