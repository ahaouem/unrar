use clap::{App, Arg};
use std::thread;
use std::time::Instant;
use unrar_lib::{create_temp_dir, execute_unar_command, install_unar, is_unar_installed};

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

fn main() {
    let matches = get_matches();
    let rar_files = get_rar_files(&matches);

    let keep_original = matches.is_present("keep");
    let overwrite_existing = matches.is_present("overwrite");

    if !is_unar_installed() {
        install_unar();
    }

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
