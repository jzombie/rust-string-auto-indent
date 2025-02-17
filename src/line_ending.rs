/// Enum representing the detected line ending style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum LineEnding {
    LF,   // "\n" (Unix, Linux, macOS)
    CRLF, // "\r\n" (Windows)
    CR,   // "\r" (old macOS)
}

impl LineEnding {
    /// Detects the line ending style used in the input string.
    pub fn detect(s: &str) -> Self {
        if s.contains("\r\n") {
            Self::CRLF
        } else if s.contains("\r") {
            Self::CR
        } else {
            Self::LF
        }
    }

    /// Returns the string representation of the line ending.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LF => "\n",
            Self::CRLF => "\r\n",
            Self::CR => "\r",
        }
    }

    /// Normalize to `\n` for consistent processing.
    pub fn normalize(s: &str) -> String {
        s.replace("\r\n", "\n").replace("\r", "\n")
    }

    /// Restores line endings back to their original value.
    #[allow(dead_code)]
    pub fn restore(&self, s: &str) -> String {
        s.replace("\n", self.as_str())
    }

    /// Applies the line endiing to the given lines.
    pub fn restore_from_lines(&self, lines: Vec<String>) -> String {
        lines.join(self.as_str())
    }
}
