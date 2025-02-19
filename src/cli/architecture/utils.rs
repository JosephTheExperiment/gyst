use crate::pub_struct;
use std::fs::{create_dir_all, File};
use std::{
    env,
    io::Write,
    path::PathBuf,
    process::{Command, Output},
};

pub_struct!(
    struct TextFile {
        data: String,
        name: String,
    }
);

pub_struct!(
    struct Directory {
        files: Vec<TextFile>,
        directories: Vec<Directory>,
        name: String,
    }
);

pub fn build_directory(directory: Directory, path: Option<PathBuf>) -> std::io::Result<()> {
    let mut path: PathBuf = path.unwrap_or(env::current_dir()?);
    path.push(directory.name);
    create_dir_all(path.as_path())?;
    
    for file in directory.files {
        let mut file_handel: File = File::create(format!("{}/{}", path.display(), file.name))?;
        file_handel.write_all(file.data.as_bytes())?;
    }

    for dir in directory.directories {
        build_directory(dir, Some(path.clone()))?;
    }

    Ok(())
}

pub fn run_command(args: Vec<&str>) -> std::io::Result<Output> {
    if std::env::consts::OS == "windows" {
        return Command::new("cmd")
            .arg("/C")
            .args(args)
            .output();
    } else {
        return Command::new("sh")
            .arg("-c")
            .args(args)
            .output();
    }
}