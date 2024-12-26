use std::{
    env::{self},
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

/// The default message displayed when prompting for user input via stdin.
const DEFAULT_MESSAGE: &str = "Enter input (end with two blank lines):";

/// Errors that can occur while reading input.
#[derive(Debug)]
pub enum ReadError {
    /// Error occurred while reading from stdin.
    StdinReadError(io::Error),
    /// Error occurred while reading from a file.
    FileReadError(io::Error),
    /// Error occurred while reading the path of the current executable.
    ExePathReadError(io::Error),
}

impl std::error::Error for ReadError {}

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StdinReadError(e) => write!(f, "Failed to read from stdin: {}", e),
            Self::FileReadError(e) => write!(f, "Failed to read from file: {}", e),
            Self::ExePathReadError(e) => write!(f, "Failed to read executable path: {}", e),
        }
    }
}

/// A builder for configuring and performing input reading operations.
#[derive(Default)]
pub struct InputReader {
    /// Optional file path to read from
    path: Option<String>,
    /// Optional message to display when reading from stdin
    message: Option<String>,
}

impl InputReader {
    /// Creates a new `InputReader` with default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the file path to read from.
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Sets the message to display when reading from stdin.
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Performs the input reading operation based on the configured options.
    pub fn read(&self) -> Result<Vec<String>, ReadError> {
        read_input(self.path.as_deref(), self.message.as_deref())
    }
}

/// Trims leading and trailing empty lines from a vector of strings.
/// Empty lines are those with zero length. Lines containing only whitespace
/// are not considered empty.
///
/// # Arguments
/// * `lines` - A slice of strings to process
///
/// # Returns
/// A new `Vec<String>` with leading and trailing empty lines removed.
fn trim_empty_lines(lines: &[String]) -> Vec<String> {
    let mut start = 0;
    let mut end = lines.len();

    while start < end && lines[start].is_empty() {
        start += 1;
    }
    while start < end && lines[end - 1].is_empty() {
        end -= 1;
    }

    lines[start..end].to_vec()
}

/// Reads input from command-line arguments, skipping the first argument (program name).
///
/// # Arguments
///
/// - `args`: An iterator over the command-line arguments.
///
/// # Returns
///
/// A `Vec<String>` containing the arguments (excluding the program name).
fn read_input_from_args(args: impl Iterator<Item = String>) -> Vec<String> {
    let lines = args.skip(1).collect::<Vec<_>>();
    trim_empty_lines(&lines)
}

/// Reads input from a specified file.
///
/// # Arguments
///
/// - `path`: The file path to read from.
///
/// # Returns
///
/// A `Result` containing a `Vec<String>` of lines or a `ReadError` if the file could not be read.
fn read_input_from_file(path: &str) -> Result<Vec<String>, ReadError> {
    let file = File::open(path).map_err(ReadError::FileReadError)?;
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .map_err(ReadError::FileReadError)?;

    Ok(trim_empty_lines(&lines))
}

/// Reads input from a generic reader until two successive blank lines are encountered.
///
/// # Arguments
///
/// - `reader`: The reader to read from (e.g., stdin or a file).
/// - `message`: A message to display before reading input.
///
/// # Returns
///
/// A `Result` containing a `Vec<String>` of lines or a `ReadError` if an error occurs during reading.
fn read_input_from_reader<R>(reader: R, message: &str) -> Result<Vec<String>, ReadError>
where
    R: BufRead,
{
    println!("{}", message);

    let mut lines = Vec::new();
    let mut blank_line_count = 0;

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => return Err(ReadError::StdinReadError(error)),
        };

        if line.trim().is_empty() {
            blank_line_count += 1;
        } else {
            blank_line_count = 0;
        }

        if blank_line_count >= 2 {
            break;
        }

        lines.push(line);
    }

    Ok(trim_empty_lines(&lines))
}

/// Determines the appropriate working directory for the application.
///
/// This function attempts to find the working directory in the following order:
/// 1. If running through Cargo (development), returns the project directory using `CARGO_MANIFEST_DIR`
/// 2. Otherwise (production), returns the directory containing the executable
///
/// # Returns
///
/// Returns a `Result` containing a [`PathBuf`] pointing to either:
/// - The project directory (during development)
/// - The executable's directory (in production)
fn get_working_dir() -> Result<PathBuf, ReadError> {
    env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .or_else(|_| {
            env::current_exe()
                .map_err(ReadError::ExePathReadError)
                .and_then(|path| {
                    path.parent()
                        .ok_or_else(|| {
                            ReadError::ExePathReadError(io::Error::new(
                                io::ErrorKind::NotFound,
                                "Failed to get executable directory",
                            ))
                        })
                        .map(|p| p.to_path_buf())
                })
        })
}

/// Reads input from one of three sources, in the following priority:
/// 1. Command-line arguments (excluding the program name).
/// 2. A specified file.
/// 3. Stdin, prompted with a message.
///
/// # Arguments
///
/// - `path`: Optional file path to read from.
/// - `message`: Optional message to display when prompting for stdin.
///
/// # Returns
///
/// A `Result` containing a `Vec<String>` of lines or a `ReadError` if reading fails.
///
/// # Edge Cases
///
/// If the input contains only empty lines, the returned Vec<String> will be empty.
fn read_input(path: Option<&str>, message: Option<&str>) -> Result<Vec<String>, ReadError> {
    // Change working directory to the project directory or the directory of the current executable.
    env::set_current_dir(get_working_dir()?).map_err(ReadError::ExePathReadError)?;

    // Attempt to read input from args
    let args = read_input_from_args(env::args());
    if !args.is_empty() {
        return Ok(args);
    }

    // Attempt to read input from file
    if let Some(path) = path {
        if let Ok(lines) = read_input_from_file(path) {
            return Ok(lines);
        }
    }

    // Attempt to read input from stdin
    let stdin = io::stdin();
    read_input_from_reader(stdin.lock(), message.unwrap_or(DEFAULT_MESSAGE))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn test_trim_empty_lines() {
        assert_eq!(
            trim_empty_lines(&Vec::<String>::new()),
            Vec::<String>::new()
        );
        assert_eq!(
            trim_empty_lines(&["   ".to_string(), "".to_string()]),
            vec!["   ".to_string()]
        );
        assert_eq!(
            trim_empty_lines(&["line1".to_string(), "line2".to_string()]),
            vec!["line1", "line2"]
        );
        assert_eq!(
            trim_empty_lines(&[
                "".to_string(),
                "line1".to_string(),
                "line2".to_string(),
                "".to_string()
            ]),
            vec!["line1", "line2"]
        );
    }

    #[test]
    fn test_read_input_from_args() {
        // Mock arguments
        let mock_args = [
            "program_name".to_string(),
            "".to_string(),
            "arg1".to_string(),
            "arg2".to_string(),
            "".to_string(),
        ];
        let args = read_input_from_args(mock_args.into_iter());
        assert_eq!(args, vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_read_input_from_file() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file
            .write_all(b"\nline1\nline2\nline3\n")
            .expect("Failed to write to temp file");

        let result = read_input_from_file(temp_file.path().to_str().unwrap());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["line1", "line2", "line3"]);
    }

    #[test]
    fn test_read_input_from_file_not_found() {
        let result = read_input_from_file("non_existent_file.txt");

        assert!(result.is_err());

        #[allow(clippy::match_wildcard_for_single_variants)]
        match result.unwrap_err() {
            ReadError::FileReadError(_) => {} // Expected error
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_read_input_from_reader() {
        let input = "\nline1\nline2\n\n\nline3";
        let cursor = Cursor::new(input);

        let result = read_input_from_reader(cursor, "Test message");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["line1", "line2"]);
    }

    #[test]
    fn test_read_input_from_reader_unicode() {
        let input = "line1\n您好\nこんにちは\n\n\nline3";
        let cursor = Cursor::new(input);

        let result = read_input_from_reader(cursor, "Test message");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["line1", "您好", "こんにちは"]);
    }

    #[test]
    fn test_builder_pattern() {
        let reader = InputReader::new()
            .with_path("test.txt")
            .with_message("Enter data:");

        assert!(reader.path.is_some());
        assert!(reader.message.is_some());
    }

    #[test]
    fn test_error_display() {
        let err = ReadError::FileReadError(io::Error::new(io::ErrorKind::NotFound, "test"));
        assert!(err.to_string().contains("Failed to read from file"));
    }
}
