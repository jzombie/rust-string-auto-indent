# Multi-line String Auto Indent

[![made-with-rust][rust-logo]][rust-src-page]
[![crates.io][crates-badge]][crates-page]
[![Documentation][docs-badge]][docs-page]
[![MIT licensed][license-badge]][license-page]


| OS            | Status                                                                               |
|---------------|--------------------------------------------------------------------------------------|
| Ubuntu-latest | [![Ubuntu Tests][ubuntu-latest-badge]][ubuntu-latest-workflow]                       |
| macOS-latest  | [![macOS Tests][macos-latest-badge]][macos-latest-workflow]                          |
| Windows-latest| [![Windows Tests][windows-latest-badge]][windows-latest-workflow]                    |


A Rust utility for automatically normalizing multi-line string indentation while preserving platform-specific line endings.

## Overview

When working with multi-line strings inside indented code blocks, unwanted leading spaces may be introduced. This can affect readability, logging output, and formatted text generation.

`string-auto-indent` provides an automated way to normalize multi-line strings without modifying the first line's indentation.

## Installation

```sh
cargo add string-auto-indent
```

## Usage

```rust
use string_auto_indent::{auto_indent, LineEnding};

let text = r#"
    String Auto Indent

    Level 1
        Level 2
            Level 3
"#;

// For cross-platform testing
let line_ending = LineEnding::from(text);

// With auto-indent
assert_eq!(
    auto_indent(text),
    // For cross-platform testing: Restore platform-specific line endings
    line_ending.denormalize("String Auto Indent\n\nLevel 1\n    Level 2\n        Level 3\n")
);

// Without auto-indent
assert_eq!(
    text,
    // For cross-platform testing: Restore platform-specific line endings
    line_ending.denormalize("\n    String Auto Indent\n\n    Level 1\n        Level 2\n            Level 3\n"),
);
```

### Example Output

**With `auto-indent` enabled.**

```text
String Auto Indent

Level 1
    Level 2
        Level 3
```

**With `auto-intent` disabled.**

```text
    String Auto Indent

    Level 1
        Level 2
            Level 3
```

## How It Works

1. Detects the platform’s line endings (`\n`, `\r\n`, `\r`) and normalizes input for processing.
2. Preserves the first line exactly as written.
3. Finds the least-indented non-empty line (excluding the first) and adjusts all others accordingly.
4. Ensures blank lines remain but contain no extra spaces.
5. Restores platform-specific line endings when outputting the result.

## When to Use

- Formatting log messages or CLI output while ensuring alignment.
- Cleaning up documentation strings or multi-line literals in indented Rust code.
- Processing structured text while ensuring consistent indentation.

## License
Licensed under **MIT**. See [`LICENSE`][license-page] for details.

[rust-src-page]: https://www.rust-lang.org/
[rust-logo]: https://img.shields.io/badge/Made%20with-Rust-black?&logo=Rust

[crates-page]: https://crates.io/crates/string-auto-indent
[crates-badge]: https://img.shields.io/crates/v/string-auto-indent.svg

[docs-page]: https://docs.rs/string-auto-indent
[docs-badge]: https://docs.rs/string-auto-indent/badge.svg

[license-page]: https://github.com/jzombie/rust-string-auto-indent/blob/main/LICENSE
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg

[ubuntu-latest-badge]: https://github.com/jzombie/rust-string-auto-indent/actions/workflows/rust-tests.yml/badge.svg?branch=main&job=Run%20Rust%20Tests%20(OS%20=%20ubuntu-latest)
[ubuntu-latest-workflow]: https://github.com/jzombie/rust-string-auto-indent/actions/workflows/rust-tests.yml?query=branch%3Amain

[macos-latest-badge]: https://github.com/jzombie/rust-string-auto-indent/actions/workflows/rust-tests.yml/badge.svg?branch=main&job=Run%20Rust%20Tests%20(OS%20=%20macos-latest)
[macos-latest-workflow]: https://github.com/jzombie/rust-string-auto-indent/actions/workflows/rust-tests.yml?query=branch%3Amain

[windows-latest-badge]: https://github.com/jzombie/rust-string-auto-indent/actions/workflows/rust-tests.yml/badge.svg?branch=main&job=Run%20Rust%20Tests%20(OS%20=%20windows-latest)
[windows-latest-workflow]: https://github.com/jzombie/rust-string-auto-indent/actions/workflows/rust-tests.yml?query=branch%3Amain
