# Mark Twain CLI (`mark-twain-cli`)

A modern, fast, and interactive command-line interface (CLI) to query the **Mark Twain Vector Database API**.

Built with Rust, this tool is optimized for zero-dependency execution on target machines, providing an elegant terminal experience with loaders, colors, and an interactive menu.

---

## Features
* 📊 **Database Metadata**: View collection info, point count, active models, and dimensions.
* 🔍 **Semantic Search**: Run natural language queries directly against the vector space.
* ✍️ **Style Analysis**: Compare stylistic fingerprints of arbitrary text snippets against Mark Twain's linguistic profile.
* 🕹️ **Interactive TUI Mode**: A guided menu-driven interface with auto-loaders and smooth exits.
* ⚡ **Optimized Binary Size**: Highly tuned production builds utilizing LTO and size optimization flags.

---

## Installation

You do **not** need Rust installed to run the pre-compiled binary. Simply run the automated installer:

### macOS / Linux / Git Bash
```bash
curl -fsSL https://raw.githubusercontent.com/otrobonita-studios/mark-twain-cli/main/install.sh | bash
```

---

## Configuration

The CLI automatically reads the following environment variables if set:

* `MARK_TWAIN_API_URL`: The base URL of the Mark Twain research API. Default: `https://mark.otrobonita.com`
* `RESEARCH_API_KEY`: Optional Bearer authentication token for secured endpoints.

You can override these values dynamically using the global CLI flags `--url` (`-u`) and `--api-key` (`-k`).

---

## Usage

### 1. Interactive Mode (Default)
Simply run the executable with no arguments to start the interactive menus:
```bash
mark-twain-cli
```

### 2. Similarity Search Command
Perform a direct semantic query:
```bash
mark-twain-cli search --query "river at night" --limit 5
```
*Shortcuts:* `mark-twain-cli search -q "river at night" -l 5`

### 3. Style Analysis Command
Analyze text style:
```bash
mark-twain-cli analyze-style --text "Well, the first week went by, and we didn't do much..."
```
*Shortcuts:* `mark-twain-cli analyze-style -t "..."`

---

## Building Locally

### Prerequisites
* Rust Toolchain (Latest Stable): Installed via `rustup`

### Steps
1. Clone the repository.
2. Build in release mode to generate the optimized binary:
   ```bash
   cargo build --release
   ```
3. The binary will be compiled to `target/release/mark-twain-cli` (or `mark-twain-cli.exe` on Windows).

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
