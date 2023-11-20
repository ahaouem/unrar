# RAR Unpacker

## About

This tool, developed by Aleksander Haouem, is designed for efficiently unpacking multiple RAR files concurrently. It leverages Rust's powerful system programming capabilities and is optimized for use on macOS, but compatible with other platforms.

## Features

- **Concurrent Unpacking**: Utilizes Rust's concurrency features to unpack multiple RAR files simultaneously.
- **Flexible File Management**: Options to keep the original RAR files (`--keep`) and to overwrite existing files during extraction (`--overwrite`).
- **Custom Output Directory**: Allows setting a custom output directory (`--output`) for extracted files.
- **Automated Temporary Directory Creation**: Automatically creates a temporary directory named with the current date and time, for organizing extracted files.
- **Error Handling**: Validates the existence of RAR files and handles errors during the extraction process.

## Usage

1. Clone the repository: `git clone https://github.com/ahaouem/unrar.git`
2. Navigate to the cloned repository: `cd unrar`
3. Build the project: `cargo build --release`
4. Run the tool: `./target/release/rar-unpacker [RAR_FILE_PATHS] --output [OUTPUT_DIRECTORY] --keep --overwrite`

Replace `[RAR_FILE_PATHS]` with the paths to the RAR files you wish to unpack. The `--output`, `--keep`, and `--overwrite` flags are optional.

- `--output [output-directory]`: Sets a custom output directory for extracted files.
- `--keep`: Keeps the original RAR file after extraction.
- `--overwrite`: Allows overwriting existing files during extraction.

By default, the tool extracts files to a temporary directory with a timestamp and deletes the original RAR files unless the `--keep` option is used.

## Dependencies

- Rust (Installation instructions at [Rust's official site](https://www.rust-lang.org/learn/get-started))
- Cargo (Included with Rust installation)
- `unar` (Will be installed automatically if not present, especially for macOS users)

## Contributing

Contributions are welcome! Please open an issue for feature requests, bug reports, or pull requests.
