use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref MAIN_DIR: Mutex<PathBuf> = Mutex::new(create_main_dir());
}

fn create_main_dir() -> PathBuf {
    let date = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let main_dir = Path::new("./temp").join(&date);
    fs::create_dir_all(&main_dir).expect("Failed to create main directory for tests.");
    main_dir
}

fn get_main_dir() -> PathBuf {
    MAIN_DIR.lock().unwrap().clone()
}

fn get_file_size_in_mb(file_path: &Path) -> std::io::Result<f64> {
    let metadata = fs::metadata(file_path)?;
    Ok(metadata.len() as f64 / 1024.0 / 1024.0)
}

fn output_dir_has_files(output_dir: &Path) -> bool {
    fs::read_dir(output_dir)
        .map(|mut entries| entries.any(|e| e.is_ok()))
        .unwrap_or(false)
}

fn test_unpacking_rar(file_name: &str) {
    let main_dir = get_main_dir();
    let rar_path = Path::new(file_name);
    let file_size = get_file_size_in_mb(rar_path).unwrap_or(0.0);
    println!("{} is {:.2} MB", file_name, file_size);

    let output_dir = main_dir.join(file_name.trim_end_matches(".rar"));
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir).expect("Failed to create output directory for test.");
    }

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(file_name)
        .arg("--keep")
        .arg("--overwrite")
        .arg("--output")
        .arg(output_dir.to_str().unwrap())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Unpacking command failed for {}", file_name);
    assert!(output_dir_has_files(&output_dir), "No files found in output directory for {}", file_name);
}

#[test]
fn test_rar1_unpacking() {
    test_unpacking_rar("rar1.rar");
}

#[test]
fn test_rar2_unpacking() {
    test_unpacking_rar("rar2.rar");
}

#[test]
fn test_rar3_unpacking() {
    test_unpacking_rar("rar3.rar");
}

#[test]
fn test_rar4_unpacking() {
    test_unpacking_rar("rar4.rar");
}

#[test]
fn test_rar5_unpacking() {
    test_unpacking_rar("rar5.rar");
}