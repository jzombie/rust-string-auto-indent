# Multi-line String Auto Indent

A Rust utility for automatically normalizing multi-line string indentation while preserving platform-specific line endings.

## Overview

When working with multi-line strings inside indented code blocks, unwanted leading spaces may be introduced. This can affect readability, logging output, and formatted text generation.

`string-auto-indent` provides an automated way to normalize multi-line strings without modifying the first line's indentation.

## Installation

```sh
cargo install string-auto-indent
```

## Example

```rust
use string_auto_indent::{auto_indent, LineEnding};

let text = r#"
    String Auto Indent

    Level 1
        Level 2
            Level 3
"#;

let line_ending = LineEnding::detect(text);

// With auto-indent
assert_eq!(
    auto_indent(text),
    // Restore platform-specific line endings for testing
    line_ending.restore("String Auto Indent\n\nLevel 1\n    Level 2\n        Level 3\n")
);

// Without auto-indent
assert_eq!(
    text,
    // Restore platform-specific line endings for testing
    line_ending.restore("\n    String Auto Indent\n\n    Level 1\n        Level 2\n            Level 3\n"),
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

[license-page]: https://github.com/jzombie/rust-cargo-pkg-info-struct-builder/blob/main/LICENSE
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
