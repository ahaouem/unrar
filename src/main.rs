use clap::{App, Arg};
use std::fs;
use std::process::Command;
use std::path::{Path, PathBuf};
use chrono::Local;
use std::time::Instant;

fn main() {
    let matches = App::new("RAR Unpacker")
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
        .get_matches();

    let rar_file = matches.value_of("INPUT").unwrap_or_else(|| {
        eprintln!("Input file not provided.");
        std::process::exit(1);
    });

    if !Path::new(rar_file).exists() {
        eprintln!("File '{}' does not exist in the current directory.", rar_file);
        std::process::exit(1);
    }

    let keep_original = matches.is_present("keep");
    let output_dir = matches.value_of("output").unwrap_or("./temp");

    let date = Local::now().format("%Y-%m-%d_%H-%M").to_string();
    let temp_dir = PathBuf::from(format!("{}/{}", output_dir, date));

    if let Err(e) = fs::create_dir_all(&temp_dir) {
        eprintln!("Failed to create unique temp directory: {}", e);
        return;
    }

    let start_time = Instant::now();

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
                return;
            }

            if !keep_original {
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
            return;
        }
    }

    let duration = start_time.elapsed();
    if !matches.is_present("quiet") {
        println!("Extraction successful. Files are available in {:?}", &temp_dir);
    }
    println!("Executed in {:.2?} seconds", duration);
}
