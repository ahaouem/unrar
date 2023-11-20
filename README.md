# RAR Unpacker

## About

This tool is designed to unpack RAR files. Developed by Aleksander Haouem, it leverages Rust's powerful system programming capabilities to efficiently handle file operations. Although it can be compiled for different platforms, it is primarily intended for use on macOS.

## Features

- **Unpacks RAR Files:** Quickly extracts RAR files to a temporary directory.
- **Automatic Dependency Management:** Automatically installs 'unar' if not present on the system, which is particularly useful for macOS users.

## Usage

To use the RAR Unpacker, you need to have Rust and Cargo installed. This tool is optimized for macOS, but can be built and used on other platforms as well. Follow these steps:

1. Clone the repository: `git clone https://github.com/ahaouem/unrar.git`
2. Navigate to the cloned repository: `cd unrar`
3. Build the project: `cargo build --release`
4. Run the too `./target/release/rar-unpacker [path-to-rar-file] `

Replace `[path-to-rar-file]` with the path to the RAR file you wish to unpack.

Optional Options:

- `--keep`: Retain the original RAR file after extraction.
- `--output [output-directory]`: Specify a custom output directory for extracted files.
- `--overwrite`: Allow overwriting existing files during extraction.

By default, the original RAR file will be deleted after successful extraction if the `--keep` option is not used.

## Dependencies

- Rust (Installation instructions can be found at [Rust's official site](https://www.rust-lang.org/learn/get-started))
- Cargo (Comes with Rust installation)
- `unar` (will be installed automatically if not present, primarily for macOS)

## Contributing

Contributions are welcome! If you have a feature request, bug report, or pull request, please feel free to open an issue or submit a PR.

## License

This project is licensed under the [MIT License](LICENSE).  
