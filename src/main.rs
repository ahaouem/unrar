use clap::{App, Arg};
use std::fs;
use std::process::Command;
use std::path::{Path, PathBuf};
use chrono::Local;
use std::time::Instant;

fn get_matches() -> clap::ArgMatches<'static> {
    App::new("RAR Unpacker")
        .version("1.0")
        .author("Aleksander Haouem")
        .about("Unpacks RAR files")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input RAR file to use")
                .required(true)
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

fn get_rar_file<'a>(matches: &'a clap::ArgMatches) -> &'a str {
    matches.value_of("INPUT").unwrap_or_else(|| {
        eprintln!("Input file not provided.");
        std::process::exit(1);
    })
}
fn validate_rar_file(rar_file: &str) {
    if !Path::new(rar_file).exists() {
        eprintln!("File '{}' does not exist in the current directory.", rar_file);
        std::process::exit(1);
    }
}

fn create_temp_dir(output_dir: &str) -> PathBuf {
    let date = Local::now().format("%Y-%m-%d_%H-%M").to_string();
    let temp_dir = PathBuf::from(format!("{}/{}", output_dir, date));

    if let Err(e) = fs::create_dir_all(&temp_dir) {
        eprintln!("Failed to create unique temp directory: {}", e);
        std::process::exit(1);
    }

    temp_dir
}

fn execute_unar_command(matches: &clap::ArgMatches, rar_file: &str, temp_dir: &PathBuf) {
    let mut cmd = Command::new("unar");
    cmd.args(&[rar_file, "-o", temp_dir.to_str().unwrap()]);

    if !matches.is_present("quiet") {
        cmd.stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
    }

    let output = cmd.output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Unar command did not exit successfully.");
                std::process::exit(1);
            }

            if !matches.is_present("keep") {
                if let Err(e) = fs::remove_file(rar_file) {
                    eprintln!("Failed to remove original file: {}", e);
                }
            }

            if matches.is_present("quiet") {
                println!("Extraction successful. Files are available in {:?}", &temp_dir);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute unar command: {}", e);
            std::process::exit(1);
        }
    }
}

fn main() {
    let matches = get_matches();

    let rar_file = get_rar_file(&matches);
    validate_rar_file(rar_file);

    let output_dir = matches.value_of("output").unwrap_or("./temp");
    let temp_dir = create_temp_dir(output_dir);

    let start_time = Instant::now();

    execute_unar_command(&matches, rar_file, &temp_dir);

    let duration = start_time.elapsed();
    if !matches.is_present("quiet") {
        println!("Extraction successful. Files are available in {:?}", &temp_dir);
    }
    println!("Executed in {:.2?} seconds", duration);
}