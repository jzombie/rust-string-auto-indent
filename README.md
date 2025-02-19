# Multi-line String Auto-Indent

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

## Example 1: Basic Indentation

This example removes unnecessary leading spaces while preserving the relative indentation of nested lines.

```rust
use string_auto_indent::{auto_indent, LineEnding};

let text = r#"
    String Auto Indent

    Level 1
        Level 2
            Level 3
    "#;

// Expected output after applying auto indentation
let expected = r#"
String Auto Indent

Level 1
    Level 2
        Level 3
"#;

// Verify that `auto_indent` correctly normalizes indentation
assert_eq!(
    auto_indent(text),
    expected,
    "The auto_indent function should normalize leading whitespace."
);

// Ensure the original text is not identical to the expected output
// This confirms that `auto_indent` actually modifies the string.
assert_ne!(
    text,
    expected,
    "The original text should *not* be identical to the expected output before normalization."
);
```

### Example Output

#### With `auto-indent`

```text
String Auto Indent

Level 1
    Level 2
        Level 3
```

#### Without `auto-intent`

```text
    String Auto Indent

    Level 1
        Level 2
            Level 3
```

## Example 2: Mixed Indentation

This example demonstrates how `auto_indent` normalizes inconsistent indentation while preserving the relative structure of nested content.

```rust
use string_auto_indent::{auto_indent, LineEnding};

let text = r#"
                    String Auto Indent

                        1. Point 1
                            a. Sub point a
                            b. Sub point b
                        2. Point 2
                            a. Sub point a
                            b. Sub piont b
                                1b. Sub piont 1b
    "#;

// Expected output after applying auto indentation
let expected = r#"
String Auto Indent

    1. Point 1
        a. Sub point a
        b. Sub point b
    2. Point 2
        a. Sub point a
        b. Sub piont b
            1b. Sub piont 1b
"#;

// Verify that `auto_indent` correctly normalizes indentation
assert_eq!(
    auto_indent(text),
    expected,
    "The auto_indent function should normalize leading whitespace."
);

// Ensure the original text is not identical to the expected output
// This confirms that `auto_indent` actually modifies the string.
assert_ne!(
    text,
    expected,
    "The original text should *not* be identical to the expected output before normalization."
);
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
- Declaring multi-line variables in code where the indentation should match the codebase for readability, but the actual string content should not retain unnecessary leading spaces.
- Ensuring consistent formatting in generated strings for use in templates, serialization, or output rendering.

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
