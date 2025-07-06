
# Image to ASCII Converter

[![License](https://img.shields.io/github/license/lowpolycat1/image-ascii-rs.svg?style=flat-square)](LICENSE.txt)
&nbsp;
[![Contributors](https://img.shields.io/github/contributors/lowpolycat1/image-ascii-rs.svg?style=flat-square)](https://github.com/lowpolycat1/image-ascii-rs/graphs/contributors/)
&nbsp;
[![Rust](https://img.shields.io/badge/Rust-Stable-blue.svg?style=flat-square)](https://www.rust-lang.org/)
&nbsp;
[![Image Crates](https://img.shields.io/crates/v/image.svg)](https://crates.io/crates/image)
&nbsp;
[![Clap Crates](https://img.shields.io/crates/v/clap.svg)](https://crates.io/crates/clap)

This project is a simple CLI tool written in pure Rust that converts images into ASCII art. Customize output width and save your result to a file or view it in the terminal.

---

## 📦 Built With

* [image crate](https://crates.io/crates/image) – For loading and processing image data
* [clap crate](https://crates.io/crates/clap) – For command-line argument parsing
* Pure Rust – No FFI or C dependencies

---

## 🚀 Getting Started

### Prerequisites

* Rust toolchain (>=1.60.0)
* Cargo package manager

### Installation

```bash
git clone https://github.com/lowpolycat1/image-ascii-rs
cd image-ascii-rs
cargo build --release
```

---

## 🖥️ Usage

```bash
cargo run -- --input <input_path> --output <output_path> --width <width>
```

### Arguments

| Flag       | Description                         | Required |
| ---------- | ----------------------------------- | -------- |
| `--input`  | Path to the input image file        | ✅        |
| `--output` | Path to the output `.txt` file      | ✅        |
| `--width`  | Width of the resulting ASCII output | ✅        |

### Example

```bash
cargo run -- --input assets/cat.png --output out/cat.txt --width 100
```

This will convert the image `cat.png` into ASCII art 100 characters wide and save it to `out/cat.txt`.

---

## 📝 License

MIT License – see [LICENSE.txt](LICENSE.txt) for details.
