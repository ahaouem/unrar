use std::path::Path;
use std::process::Command;

pub fn is_unar_installed() -> bool {
    Command::new("which")
        .arg("unar")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn install_unar() {
    if is_unar_installed() {
        println!("Package unar already installed.");
        return;
    }

    println!("Installing unar using Homebrew...");
    let output = Command::new("brew")
        .arg("install")
        .arg("unar")
        .output()
        .expect("Failed to execute brew command");

    if output.status.success() {
        println!("Package unar installed.");
    } else {
        eprintln!("Failed to install unar using Homebrew.");
    }
}

pub fn create_temp_dir(output_dir: &str) -> std::path::PathBuf {
    let date = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let temp_dir = std::path::PathBuf::from(format!("{}/{}", output_dir, date));

    if let Err(e) = std::fs::create_dir_all(&temp_dir) {
        eprintln!("Failed to create directory: {}", e);
        std::process::exit(1);
    }

    temp_dir
}

pub fn execute_unar_command(
    rar_file: &str,
    temp_dir: &std::path::PathBuf,
    keep_original: bool,
    overwrite_existing: bool,
) {
    if !Path::new(rar_file).exists() {
        eprintln!("File '{}' does not exist.", rar_file);
        std::process::exit(1);
    }

    let mut cmd = Command::new("unar");
    cmd.args(&[rar_file, "-o", temp_dir.to_str().unwrap()]);

    if overwrite_existing {
        cmd.arg("-force-overwrite");
    }

    let output = cmd.output().expect("Failed to execute unar command");

    if !output.status.success() {
        eprintln!("Failed to extract '{}'", rar_file);
    }

    if !keep_original {
        if let Err(e) = std::fs::remove_file(rar_file) {
            eprintln!("Failed to delete '{}': {}", rar_file, e);
        }
    }
}
