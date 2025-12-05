//! Type definitions for the input reader library.
//!
//! This module contains all the public and internal types used throughout the library.

use std::{
    env, error, fmt,
    fs::File,
    io::{self, BufRead, BufReader, Seek},
    string::ToString,
    vec::Vec,
};
use utf8_chars::BufReadCharsExt;

// =============================================================================
// Error
// =============================================================================

/// Errors that can be returned to consumers of the library.
#[derive(Debug)]
pub enum Error {
    /// The requested path or directory was not found.
    NotFound,
    /// An I/O error occurred while accessing the filesystem.
    Io(io::Error),
    /// An environment variable was not set or invalid.
    Var(env::VarError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Self {
        Error::Var(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotFound => write!(f, "path or directory not found"),
            Error::Io(err) => write!(f, "I/O error: {err}"),
            Error::Var(err) => write!(f, "environment variable error: {err}"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::NotFound => None,
            Error::Io(err) => Some(err),
            Error::Var(err) => Some(err),
        }
    }
}

// =============================================================================
// InternalError
// =============================================================================

/// Internal errors used within the library.
///
/// These errors are not exposed to consumers and are handled internally
/// to produce appropriate `Outcome` or `Error` values.
#[derive(Debug)]
pub(crate) enum InternalError {
    /// No input was provided or found.
    NoInput,
    /// A path-related error occurred.
    Path(Error),
    /// An I/O error occurred while reading input.
    Io(io::Error),
}

impl From<Error> for InternalError {
    fn from(err: Error) -> Self {
        InternalError::Path(err)
    }
}

impl From<env::VarError> for InternalError {
    fn from(err: env::VarError) -> Self {
        InternalError::Path(err.into())
    }
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InternalError::NoInput => write!(f, "no input provided or found"),
            InternalError::Path(err) => write!(f, "path error: {err}"),
            InternalError::Io(err) => write!(f, "I/O error: {err}"),
        }
    }
}

// =============================================================================
// InputMethod
// =============================================================================

/// The method used to read input.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub(crate) enum InputMethod {
    /// Automatically determine the input method.
    #[default]
    Auto,
    /// Read from a file.
    File,
    /// Read from command-line arguments.
    Args,
    /// Read from standard input.
    Stdin,
}

// =============================================================================
// Input
// =============================================================================

/// Input data that can be read as lines or characters.
///
/// This type represents input that has been successfully loaded from a file,
/// command-line arguments, or standard input. It should be consumed using
/// either [`lines()`](Input::lines) or [`chars()`](Input::chars).
#[derive(Debug)]
#[must_use = "Input should be consumed with lines() or chars()"]
pub enum Input {
    /// Input from a file, read using a buffered reader.
    File(BufReader<File>),
    /// Input from memory (args or stdin), stored as lines.
    Memory(Vec<String>),
}

impl Input {
    /// Allows creating Memory variant Inputs for use in tests.
    pub fn new(lines: Vec<String>) -> Self {
        Input::Memory(lines)
    }

    /// Converts a vector of items into a vector of inputs.
    pub fn from_each<T>(items: Vec<T>) -> Vec<Self>
    where
        Self: From<T>,
    {
        items.into_iter().map(Self::from).collect()
    }

    /// Returns an iterator over the lines of input.
    pub fn lines(self) -> Box<dyn Iterator<Item = io::Result<String>>> {
        match self {
            Input::File(reader) => Box::new(reader.lines()),
            Input::Memory(vec) => Box::new(vec.into_iter().map(Ok)),
        }
    }

    /// Returns an iterator over the characters of input.
    ///
    /// # Implementation Notes
    ///
    /// ## Memory Leak Trade-off (File variant)
    ///
    /// When consuming file-based input, this method intentionally leaks the
    /// `BufReader` using [`Box::leak`]. This is done to satisfy Rust's lifetime
    /// requirements for the character iterator while maintaining an ergonomic API.
    ///
    /// **Why this is acceptable:**
    /// - This library is designed for short-lived CLI programs that exit after
    ///   processing input
    /// - The OS automatically reclaims all process memory on exit
    /// - The leaked memory is bounded by the buffer size (typically 8KB), not
    ///   the file size
    /// - Alternative designs would require complex lifetime parameters throughout
    ///   the API, significantly hurting usability
    ///
    /// **When this might be a concern:**
    /// - Long-running services that repeatedly create `Input` instances
    /// - Applications that process many files in a single run
    ///
    /// For such cases, consider using [`lines()`](Input::lines) instead, which
    /// does not leak memory.
    ///
    /// ## Double Allocation (Memory variant)
    ///
    /// The in-memory variant joins all lines and collects characters into a `Vec`
    /// before creating the iterator. This is a pragmatic choice favoring simplicity
    /// over performance, suitable for typical puzzle input sizes. If profiling
    /// reveals this as a bottleneck, a custom iterator could eliminate the
    /// double allocation.
    pub fn chars(self) -> Box<dyn Iterator<Item = io::Result<char>>> {
        match self {
            Input::File(reader) => {
                // We can leave the cleanup of the memory to the OS on exit.
                let reader = Box::leak(Box::new(reader));
                Box::new(reader.chars())
            }
            Input::Memory(vec) => {
                // We could do a manual iterator for performance,
                // but only if we need it.
                // The double allocation is fine for now.
                let joined = vec.join("\n");
                Box::new(joined.chars().map(Ok).collect::<Vec<_>>().into_iter())
            }
        }
    }
}

impl Clone for Input {
    fn clone(&self) -> Self {
        match self {
            Input::File(reader) => {
                // Convert File -> Memory on clone to ensure independence.
                // We use try_clone() to get a new file handle, then read all
                // contents into memory. We can't return Input::File because
                // cloned file descriptors share the file offset, which would
                // cause one Input's reads to affect the other's position.
                match reader.get_ref().try_clone() {
                    Ok(mut file) => {
                        let new_reader = BufReader::new(&file);
                        let lines: Vec<String> = new_reader.lines().map_while(Result::ok).collect();

                        // Rewind to the beginning so the original Input can still be used.
                        // Since cloned file descriptors share the offset, seeking on the
                        // clone also resets the original's position.
                        let _ = file.rewind();

                        Input::Memory(lines)
                    }
                    Err(_) => Input::Memory(vec![]),
                }
            }
            Input::Memory(vec) => Input::Memory(vec.clone()),
        }
    }
}

impl From<&str> for Input {
    fn from(line: &str) -> Self {
        Input::new(vec![line.to_string()])
    }
}

impl From<String> for Input {
    fn from(line: String) -> Self {
        Input::new(vec![line])
    }
}

impl From<Vec<&str>> for Input {
    fn from(lines: Vec<&str>) -> Self {
        Input::new(lines.into_iter().map(ToString::to_string).collect())
    }
}

impl From<Vec<String>> for Input {
    fn from(lines: Vec<String>) -> Self {
        Input::new(lines)
    }
}

// =============================================================================
// Outcome
// =============================================================================

/// The outcome after attempting to read input.
///
/// Callers should match on this to determine whether to continue processing
/// or exit the application.
#[derive(Debug)]
pub enum Outcome {
    /// The caller should exit the application.
    Exit,
    /// Input is ready for processing.
    Continue(Input),
}

impl From<Input> for Outcome {
    fn from(input: Input) -> Self {
        Outcome::Continue(input)
    }
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "unwrap is okay in tests")]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // Input::lines() tests

    #[test]
    fn input_memory_lines_returns_all_lines() {
        let input = Input::Memory(vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ]);
        let lines: Vec<String> = input.lines().map(|r| r.unwrap()).collect();
        assert_eq!(lines, vec!["line1", "line2", "line3"]);
    }

    #[test]
    fn input_memory_lines_handles_empty() {
        let input = Input::Memory(vec![]);
        let lines: Vec<String> = input.lines().map(|r| r.unwrap()).collect();
        assert!(lines.is_empty());
    }

    #[test]
    fn input_file_lines_returns_all_lines() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "file line 1").unwrap();
        writeln!(temp_file, "file line 2").unwrap();

        let file = std::fs::File::open(temp_file.path()).unwrap();
        let reader = BufReader::new(file);
        let input = Input::File(reader);

        let lines: Vec<String> = input.lines().map(|r| r.unwrap()).collect();
        assert_eq!(lines, vec!["file line 1", "file line 2"]);
    }

    // Input::chars() tests

    #[test]
    fn input_memory_chars_returns_all_chars() {
        let input = Input::Memory(vec!["ab".to_string(), "cd".to_string()]);
        let chars: Vec<char> = input.chars().map(|r| r.unwrap()).collect();
        // Memory joins with newlines
        assert_eq!(chars, vec!['a', 'b', '\n', 'c', 'd']);
    }

    #[test]
    fn input_file_chars_returns_all_chars() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "abc").unwrap();

        let file = std::fs::File::open(temp_file.path()).unwrap();
        let reader = BufReader::new(file);
        let input = Input::File(reader);

        let chars: Vec<char> = input.chars().map(|r| r.unwrap()).collect();
        assert_eq!(chars, vec!['a', 'b', 'c']);
    }

    // Error From implementations tests

    #[test]
    fn error_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "test");
        let err = Error::from(io_err);
        assert!(matches!(err, Error::Io(_)));
    }

    #[test]
    fn error_from_var_error() {
        let var_err = env::VarError::NotPresent;
        let err = Error::from(var_err);
        assert!(matches!(err, Error::Var(_)));
    }

    #[test]
    fn internal_error_from_error() {
        let err = Error::NotFound;
        let internal = InternalError::from(err);
        assert!(matches!(internal, InternalError::Path(Error::NotFound)));
    }

    #[test]
    fn outcome_from_input() {
        let input = Input::Memory(vec!["test".to_string()]);
        let outcome = Outcome::from(input);
        assert!(matches!(outcome, Outcome::Continue(_)));
    }

    // Clone tests

    #[test]
    fn input_memory_clone() {
        let input = Input::Memory(vec!["line1".to_string(), "line2".to_string()]);
        let cloned = input.clone();

        // Both should be Memory variants
        assert!(matches!(cloned, Input::Memory(_)));

        // Verify content is the same
        let lines: Vec<String> = cloned.lines().map(|r| r.unwrap()).collect();
        assert_eq!(lines, vec!["line1", "line2"]);
    }

    #[test]
    fn input_file_clone_works() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "line 1").unwrap();
        writeln!(temp_file, "line 2").unwrap();

        let file = std::fs::File::open(temp_file.path()).unwrap();
        let reader = BufReader::new(file);
        let input = Input::File(reader);

        let cloned = input.clone();

        // Both original and clone should work independently
        let original_lines: Vec<String> = input.lines().map(|r| r.unwrap()).collect();
        let cloned_lines: Vec<String> = cloned.lines().map(|r| r.unwrap()).collect();

        assert_eq!(original_lines, vec!["line 1", "line 2"]);
        assert_eq!(cloned_lines, vec!["line 1", "line 2"]);
    }
}
