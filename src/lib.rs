#[cfg(doctest)]
doc_comment::doctest!("../README.md");

pub mod line_ending;
pub use line_ending::LineEnding;

/// Struct that encapsulates auto-indentation logic.
struct AutoIndent {
    line_ending: LineEnding,
}

impl AutoIndent {
    /// Creates a new instance by detecting the line ending from the input.
    fn new(input: &str) -> Self {
        Self {
            line_ending: LineEnding::detect(input),
        }
    }

    /// Applies auto-indentation rules.
    fn apply(&self, input: &str) -> String {
        if input.trim().is_empty() {
            return String::new();
        }

        // Normalize to `\n` for consistent processing
        let input = LineEnding::normalize(input);
        let mut lines: Vec<&str> = input.lines().collect();

        // Track whether the original input ended with a newline
        let ends_with_newline = input.ends_with('\n');

        // Remove the first line if it's empty
        let first_line = if lines.first().map(|s| s.trim()).unwrap_or("").is_empty() {
            lines.remove(0);
            None
        } else {
            Some(lines.remove(0)) // Take first line exactly as is
        };

        // Find the minimum indentation for all remaining lines
        let min_indent = lines
            .iter()
            .filter(|line| !line.trim().is_empty()) // Ignore empty lines
            .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
            .min()
            .unwrap_or(0);

        // Adjust indentation for all lines except the first
        let mut result: Vec<String> = Vec::new();

        if let Some(first) = first_line {
            result.push(first.to_string()); // Preserve the first line exactly
        }

        result.extend(lines.iter().map(|line| {
            if line.trim().is_empty() {
                String::new() // Convert empty lines to actual empty lines
            } else {
                line.chars().skip(min_indent).collect() // Trim only relative indentation
            }
        }));

        // Ensure the final line is empty if it originally contained only whitespace
        if result.last().map(|s| s.trim()).unwrap_or("").is_empty() {
            *result.last_mut().unwrap() = String::new();
        }

        // Preserve the original trailing newline behavior
        let mut output = self.line_ending.restore_from_lines(result);
        if ends_with_newline {
            output.push_str(self.line_ending.as_str());
        }

        output
    }
}

/// Auto-indents a string while preserving original line endings.
pub fn auto_indent(input: &str) -> String {
    AutoIndent::new(input).apply(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_ending::LineEnding;

    #[test]
    fn test_basic_implementation() {
        let input = r#"Basic Test
        1
            2
                3
        "#;

        let line_ending = LineEnding::detect(input);

        // With auto-indent
        assert_eq!(
            auto_indent(input),
            // string_replace_all("Basic Test\n1\n    2\n        3\n", "\n", e.as_str())
            line_ending.restore("Basic Test\n1\n    2\n        3\n")
        );

        // Without auto-indent
        assert_eq!(
            input,
            line_ending
                .restore("Basic Test\n        1\n            2\n                3\n        ")
        );
    }

    #[test]
    fn test_empty_first_line() {
        let input = r#"
        1
            2
                3
        "#;

        let line_ending = LineEnding::detect(input);

        // With auto-indent
        assert_eq!(
            auto_indent(input),
            line_ending.restore("1\n    2\n        3\n")
        );

        // Without auto-indent
        assert_eq!(
            input,
            line_ending.restore("\n        1\n            2\n                3\n        "),
        );
    }

    #[test]
    fn test_indented_first_line() {
        let input = r#"     <- First Line
        Second Line
        "#;

        let line_ending = LineEnding::detect(input);

        // With auto-indent
        assert_eq!(
            auto_indent(input),
            line_ending.restore("     <- First Line\nSecond Line\n")
        );

        // Without auto-indent
        assert_eq!(
            input,
            line_ending.restore("     <- First Line\n        Second Line\n        "),
        );
    }

    #[test]
    fn test_mixed_indentation() {
        let input = r#"First Line
        Second Line
Third Line
        "#;

        let line_ending = LineEnding::detect(input);

        // With auto-indent
        assert_eq!(
            auto_indent(input),
            line_ending.restore("First Line\n        Second Line\nThird Line\n",)
        );

        // Without auto-indent
        assert_eq!(
            input,
            line_ending.restore("First Line\n        Second Line\nThird Line\n        "),
        );
    }

    #[test]
    fn test_single_line_no_change() {
        let input = "Single line no change";

        let line_ending = LineEnding::detect(input);

        // With auto-indent
        assert_eq!(
            auto_indent(input),
            line_ending.restore("Single line no change")
        );

        // Without auto-indent
        assert_eq!(input, line_ending.restore("Single line no change"));
    }

    #[test]
    fn test_multiple_blank_lines() {
        let input = r#"First Line
        
            A

            B

            C

                D

        E
        "#;

        let line_ending = LineEnding::detect(input);

        // With auto-indent
        assert_eq!(
            auto_indent(input),
            line_ending.restore("First Line\n\n    A\n\n    B\n\n    C\n\n        D\n\nE\n")
        );

        // Without auto-indent
        assert_eq!(
            input,
            line_ending.restore(
                "First Line\n        \n            A\n\n            B\n\n            C\n\n                D\n\n        E\n        "
            ),
        );
    }
}
