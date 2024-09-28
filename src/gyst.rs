use std::fs::File;
use std::process::{Command, Output};

#[allow(dead_code)]
fn git_init(path: &str) {
    let path: String = path.to_string() + "\\";
    let _ = run_command(vec!["git", "init", path.as_str()]);
    let _ = File::create(format!("{}\\.gitignore", path));
}

pub fn run_command(args: Vec<&str>) -> Output {
    if std::env::consts::OS == "windows" {
        return Command::new("cmd").arg("/C").args(args).output().unwrap();
    } else {
        return Command::new("sh").arg("-c").args(args).output().unwrap();
    }
}
