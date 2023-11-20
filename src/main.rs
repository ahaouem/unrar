use clap::{App, Arg};
use std::{fs, thread};
use std::process::Command;
use std::path::{Path, PathBuf};
use chrono::Local;
use std::time::Instant;

fn get_matches() -> clap::ArgMatches<'static> {
    App::new("RAR Unpacker")
        .version("1.0")
        .author("Aleksander Haouem")
        .about("Unpacks multiple RAR files concurrently")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input RAR files to use")
                .required(true)
                .multiple(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .takes_value(true)
                .help("Sets the output directory"),
        )
        .arg(
            Arg::with_name("keep")
                .long("keep")
                .help("Keep the original RAR file after extraction"),
        )
        .arg(
            Arg::with_name("overwrite")
                .long("overwrite")
                .help("Allow overwriting existing files during extraction"),
        )
        .get_matches()
}

fn get_rar_files<'a>(matches: &'a clap::ArgMatches) -> clap::Values<'a> {
    matches.values_of("INPUT").unwrap()
}

fn validate_rar_file(rar_file: &str) {
    if !Path::new(rar_file).exists() {
        eprintln!("File '{}' does not exist.", rar_file);
        std::process::exit(1);
    }
}

fn create_temp_dir(output_dir: &str) -> PathBuf {
    let date = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let temp_dir = PathBuf::from(format!("{}/{}", output_dir, date));

    if let Err(e) = fs::create_dir_all(&temp_dir) {
        eprintln!("Failed to create directory: {}", e);
        std::process::exit(1);
    }

    temp_dir
}

fn execute_unar_command(rar_file: &str, temp_dir: &PathBuf, keep_original: bool, overwrite_existing: bool) {
    validate_rar_file(rar_file);

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
        if let Err(e) = fs::remove_file(rar_file) {
            eprintln!("Failed to delete '{}': {}", rar_file, e);
        }
    }
}

fn main() {
    let matches = get_matches();
    let rar_files = get_rar_files(&matches);

    let keep_original = matches.is_present("keep");
    let overwrite_existing = matches.is_present("overwrite");

    let start_time = Instant::now();

    let mut handles = vec![];
    for rar_file in rar_files {
        let rar_file = rar_file.to_string();
        let output_dir = matches.value_of("output").unwrap_or("./temp").to_string();
        let temp_dir = create_temp_dir(&output_dir);

        let handle = thread::spawn(move || {
            execute_unar_command(&rar_file, &temp_dir, keep_original, overwrite_existing);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start_time.elapsed();
    println!("Total execution time: {:.2?} seconds", duration);
}