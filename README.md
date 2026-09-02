# Mark Twain CLI (`mark-twain-cli`)

A modern, fast, and interactive command-line interface (CLI) to query the **Mark Twain Vector Database API**.

Built with Rust, this tool is optimized for zero-dependency execution on target machines, providing an elegant terminal experience with loaders, colors, and an interactive menu.

---

## Features
* 📊 **Database Metadata**: View collection info, point count, active models, and dimensions.
* 🔍 **Semantic Search**: Run natural language queries directly against the vector space.
* ✍️ **Corpus Proximity**: Find the passages in Twain's work semantically closest to a text snippet, with real similarity scores. It reports proximity to the indexed corpus — not a judgement about authorship.
* 🕹️ **Interactive TUI Mode**: A guided menu-driven interface with auto-loaders and smooth exits.
* ⚡ **Optimized Binary Size**: Highly tuned production builds utilizing LTO and size optimization flags.

---

## Installation

You do **not** need Rust installed to run the pre-compiled binary. Simply run the automated installer:

### macOS / Linux / Git Bash
```bash
curl -fsSL https://raw.githubusercontent.com/otrobonita-studios/mark-twain-cli/main/install.sh | bash
```

The installer resolves the latest release, verifies the downloaded archive against
the `SHA256SUMS` published with that release, and installs to `~/.local/bin`
(`~/bin` under Git Bash). It refuses to install anything it cannot verify, and it
tells you if the install directory is not on your `PATH`.

Prebuilt binaries are published for:

| Platform | Architectures |
|----------|---------------|
| Linux    | x86_64, aarch64 |
| macOS    | x86_64 (Intel), aarch64 (Apple Silicon) |
| Windows  | x86_64 |

To verify a download by hand, or to install somewhere else, the release assets and
`SHA256SUMS` are on the [releases page](https://github.com/otrobonita-studios/mark-twain-cli/releases).

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

### 3. Corpus Proximity Command
Find the closest passages to a piece of text:
```bash
mark-twain-cli analyze-style --text "Well, the first week went by, and we didn't do much..."
```
*Shortcuts:* `mark-twain-cli analyze-style -t "..."`

### 4. Getting Help
The CLI comes with built-in interactive help documentation:
* **Global Help**: View all available commands, options, and global flags:
  ```bash
  mark-twain-cli --help
  ```
* **Search Help**: View parameters accepted by the semantic search command (e.g. `--limit`, `--query`):
  ```bash
  mark-twain-cli search --help
  ```
* **Style Analysis Help**: View instructions for the style analysis command:
  ```bash
  mark-twain-cli analyze-style --help
  ```

---

## Development

If you want to contribute, build from source, or package releases, please refer to the **[DEVELOPMENT.md](DEVELOPMENT.md)** guide.

