# NoteCli

[中文](../README.md) | English

<p align="center">
  <strong>A lightweight, interactive CLI note manager built with Rust.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-2024-orange.svg" alt="Rust Edition 2024" />
  <img src="https://img.shields.io/badge/platform-Windows-green.svg" alt="Platform: Windows" />
  <img src="https://img.shields.io/badge/license-Apache%202.0-blue.svg" alt="License: Apache 2.0" />
</p>

---

> **Disclaimer**: This is a personal Rust practice project and my first attempt at open source. The code may contain immature designs and implementations — some repetitive work was done with AI assistance, so there might be oversights. Suggestions and guidance are very welcome, thank you!

NoteCli is a command-line note manager designed for developers. All data is stored locally as Markdown files with JSON metadata — no database, no network, no vendor lock-in. Every command supports both direct parameter invocation and interactive operation, making it suitable for both scripting and manual use.

## Features

- **Full CRUD** — Create, view, edit, and delete notes with rich metadata
- **Interactive Mode** — All commands automatically launch interactive prompts when arguments are missing (powered by `dialoguer`)
- **Categories & Tags** — Flexible category/tag system for organizing notes
- **Priority Levels** — low / normal / high / urgent, with priority-based sorting
- **Full-text Search** — Supports plain text, regex, and fuzzy search modes with highlighted results
- **Pin, Archive, Done** — Quick status operations
- **Import & Export** — Supports JSON, Markdown, plain text, and CSV formats
- **Configurable** — TOML-based config system for themes, display, storage, and more
- **Offline-first** — All data saved as local `.md` files
- **Windows Support** — Currently supports the Windows platform

## Installation

### Build from Source

```bash
git clone https://github.com/chilling2333/NoteCli.git
cd NoteCli
cargo build --release
```

The compiled binary is at `target/release/note.exe`. Add it to your `PATH` for global access.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.85+ (Edition 2024)

## Quick Start

```bash
# Create a note
note add "Meeting notes: Q3 roadmap discussion"

# Create a note with metadata
note add "Fix login bug" -t "Bug: Login" -c work -T bug urgent -p high

# Interactive mode (omit required arguments to trigger)
note add              # Prompts for content
note edit             # Shows note list for selection
note delete           # Choose deletion method

# List all notes
note list

# Search
note search "roadmap"
note search "error.*timeout" -m regex

# View a note
note show 1

# Pin, archive, mark done
note pin 1
note archive 2
note done 3
```

## Command Reference

### Create Notes

```bash
note add "Note content"                            # Quick create
note add "Content" -t "Title" -c category -T tag1 tag2 -p high  # Full options
note add                                           # Interactive multi-line input
```

### View Notes

```bash
note show 1           # View by ID
note show             # Interactive note selection
note show 1 --raw     # Plain text output (no ANSI colors)
```

### Edit Notes

```bash
note edit 1 -t "New title"               # Change title
note edit 1 -c work -T rust cli           # Change category and tags
note edit 1 -p urgent                     # Change priority
note edit 1 -C "Replacement content"      # Replace body
note edit 1 -a "Appended content"         # Append to body
note edit                                 # Interactive: select note → select field
```

### Delete Notes

```bash
note delete 1                    # Delete by ID
note delete -T bug               # Delete all notes with tag "bug"
note delete -c archive           # Delete all notes in category "archive"
note delete                      # Interactive deletion
```

### List Notes

```bash
note list                        # All notes (sorted by priority)
note list -c work                # Filter by category
note list -T rust -T cli         # Filter by tags
note list -p high                # Filter by priority
note list -s modified -n 20      # Sort by modified time, show 20
note list -d today               # Notes created today
note list --has-tag              # Only show notes with tags
```

### Search

```bash
note search "keyword"             # Plain text search
note search "pattern" -m regex    # Regex search
note search "fuzzy match" -m fuzzy # Fuzzy search
note search "Error" --case-sensitive  # Case-sensitive
```

### Categories & Tags

```bash
note category list                # List all categories
note category tree                # Tree view
note category rename old_name new_name  # Rename

note tag list                     # List all tags
note tag rename old_name new_name    # Rename
```

### Pin / Archive / Done

```bash
note pin 1                        # Pin to top
note unpin 1                      # Unpin
note archive 1                    # Archive (hidden from default list)
note unarchive 1                  # Unarchive
note done 1                       # Mark as done
```

### Import & Export

```bash
# Export
note export -f json -a            # Export all as JSON
note export -f markdown -c work   # Export category "work" as Markdown
note export -f csv -T rust        # Export tag "rust" as CSV
note export                       # Interactive: select format and notes

# Import
note import notes.json            # Import from JSON
note import notes.md              # Import from Markdown (split by ## headings)
note import notes.txt             # Import from plain text (split by ---)
note import                       # Interactive: select file
```

### Stats & Config

```bash
note stats                        # Note database statistics

note config list                  # View all config
note config get display.color     # Get a config value
note config set general.default_priority high   # Set a config value
```

## Storage Structure

All data is stored under `.notecli/` in the working directory:

```
.notecli/
├── config.toml          # Configuration file
├── index.json           # Metadata index cache
└── notes/
    ├── meeting-notes.md
    ├── bug-fix.md
    └── ...
```

Each note is a Markdown file with JSON front matter for metadata:

```markdown
---
{"id":1,"title":"Meeting Notes","category":{"id":1,"name":"work","parentid":0},"tags":[{"id":1,"name":"weekly"}],"priority":"normal","created":"2024-01-15T14:30:00+08:00","modified":"2024-01-15T16:45:00+08:00"}
---

Meeting discussion points and action items...
```

Note files can always be viewed and edited directly with any text editor, and version-controlled with Git.

## Configuration

Default configuration is at `.notecli/config.toml`:

```toml
[general]
default_priority = "normal"     # Default priority for new notes
default_category = "default"    # Default category for new notes

[display]
color = true                    # Enable colored output
date_format = "%Y-%m-%d %H:%M"  # Date display format
title_max_width = 50            # Max display width for titles

[storage]
notes_dir = "./.notecli/notes"
index_file = "./.notecli/index.json"

[search]
default_mode = "plain"          # Default search mode
case_sensitive = false          # Default case sensitivity
max_results = 50                # Max search results

[theme]
title = "cyan bold"             # Title style
id = "yellow"                   # ID style
tag = "green"                   # Tag style
category = "blue"               # Category style
date = "dark_gray"              # Date style
separator = "dark_gray"         # Separator style
priority_low = "white"          # Low priority style
priority_normal = "green"       # Normal priority style
priority_high = "yellow bold"   # High priority style
priority_urgent = "red bold"    # Urgent priority style
```

## Project Structure

```
src/
├── main.rs                  # Entry point
├── arg.rs                   # CLI argument parsing (clap derive)
└── notes/
    ├── mod.rs               # Module root
    ├── model.rs             # Data models (Note, Category, Tag)
    ├── storage.rs           # File I/O and index management
    ├── output.rs            # Terminal output and theming
    ├── theme.rs             # Color/style definitions
    ├── config.rs            # Configuration system
    ├── input.rs             # Interactive input utilities
    └── handle/
        ├── add.rs           # Create notes
        ├── show.rs          # View notes
        ├── edit.rs          # Edit notes
        ├── delete.rs        # Delete notes
        ├── list.rs          # List and filter
        ├── search.rs        # Full-text search
        ├── category.rs      # Category management
        ├── tag.rs           # Tag management
        ├── export.rs        # Export
        ├── import.rs        # Import
        ├── stats.rs         # Statistics
        ├── pin.rs           # Pin/unpin
        ├── archive.rs       # Archive/unarchive
        ├── done.rs          # Mark done
        ├── config.rs        # Config management
        └── completion.rs    # Shell completion
```

## Tech Stack

| Dependency | Purpose |
|------------|---------|
| [clap](https://docs.rs/clap) 4.6 | CLI argument parsing (derive) |
| [dialoguer](https://docs.rs/dialoguer) 0.12 | Interactive prompts |
| [comfy-table](https://docs.rs/comfy-table) 7.1 | Table rendering |
| [colored](https://docs.rs/colored) 3.1 | Colored terminal output |
| [serde](https://docs.rs/serde) + [serde_json](https://docs.rs/serde_json) | Serialization/deserialization |
| [chrono](https://docs.rs/chrono) 0.4 | Date and time handling |
| [regex](https://docs.rs/regex) 1.11 | Regex search |
| [toml](https://docs.rs/toml) 1.1 | Config file parsing |
| [tempfile](https://docs.rs/tempfile) 3.27 | Safe file operations |

## License

This project is licensed under the [Apache License 2.0](../LICENSE).

## Contributing

Contributions are welcome! Feel free to submit Issues for bug reports or feature suggestions, and Pull Requests are also welcome.

1. Fork this repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -m 'Add my feature'`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Open a Pull Request
