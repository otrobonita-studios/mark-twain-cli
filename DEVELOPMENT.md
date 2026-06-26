# Developer Documentation

This document describes how to build, test, and release the **Mark Twain CLI** tool locally.

---

## Building Locally

### Prerequisites
* Rust Toolchain (Latest Stable): Installed via `rustup`

### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/otrobonita-studios/mark-twain-cli.git
   cd mark-twain-cli
   ```
2. Build in release mode to generate the optimized binary:
   ```bash
   cargo build --release
   ```
3. The binary will be compiled to `target/release/mark-twain-cli` (or `mark-twain-cli.exe` on Windows).

### Running & Installing Locally

You can run the compiled binary directly:
* **Windows (PowerShell)**: `.\target\release\mark-twain-cli.exe`
* **macOS / Linux**: `./target/release/mark-twain-cli`

#### Make it available globally:
To run the CLI from any directory without typing the full path, copy the binary to a directory already in your `PATH` (such as your Rust cargo bin directory):
* **Windows (PowerShell)**:
  ```powershell
  Copy-Item .\target\release\mark-twain-cli.exe C:\Users\Dindator\.cargo\bin\
  ```
* **macOS / Linux**:
  ```bash
  cp target/release/mark-twain-cli ~/.cargo/bin/
  ```

---

## Release Checklist

To package the binaries for distribution:
1. Compile for your target systems:
   * **macOS Intel**: `cargo build --release --target x86_64-apple-darwin`
   * **macOS Apple Silicon**: `cargo build --release --target aarch64-apple-darwin`
   * **Windows**: `cargo build --release --target x86_64-pc-windows-msvc` (or `x86_64-pc-windows-gnu`)
2. Package the binaries:
   * On macOS, compress the binary to `mark-twain-cli-<version>-macos-<arch>.tar.gz`
   * On Windows, compress the executable to `mark-twain-cli-<version>-windows-x86_64.zip`
3. Upload to GitHub Releases matching the tag name (e.g. `v0.1.0`).
