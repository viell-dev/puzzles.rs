use std::{
    env::{self},
    fs::File,
    io::{self, BufRead, BufReader, Lines},
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
    /// Error occurred while trying to stream a file.
    StreamingRequiresPath,
}

impl std::error::Error for ReadError {}

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StdinReadError(e) => write!(f, "Failed to read from stdin: {}", e),
            Self::FileReadError(e) => write!(f, "Failed to read from file: {}", e),
            Self::ExePathReadError(e) => write!(f, "Failed to read executable path: {}", e),
            Self::StreamingRequiresPath => {
                write!(f, "Cannot stream input without a path specified")
            }
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
    /// Returns all lines at once as a Vec<String>.
    ///
    /// # Examples
    /// ```
    /// use input_reader::InputReader;
    ///
    /// // Read from a file
    /// let reader = InputReader::new()
    ///     .with_path("input.txt");
    /// let lines = reader.read().expect("Failed to read input");
    ///
    /// // Read from stdin with custom prompt
    /// let reader = InputReader::new()
    ///     .with_message("Please enter data:");
    /// let lines = reader.read().expect("Failed to read input");
    ///
    /// // Read with defaults (uses stdin with default prompt)
    /// let reader = InputReader::new();
    /// let lines = reader.read().expect("Failed to read input");
    /// ```
    pub fn read(&self) -> Result<Vec<String>, ReadError> {
        read_input(self.path.as_deref(), self.message.as_deref())
    }

    /// Returns a line iterator for file input if a path has been set.
    /// This is useful for processing large files line by line without loading
    /// the entire file into memory.
    ///
    /// # Examples
    /// ```
    /// use input_reader::InputReader;
    ///
    /// // Basic streaming example
    /// let reader = InputReader::new()
    ///     .with_path("large_file.txt");
    ///
    /// match reader.read_streaming() {
    ///     Ok(lines) => {
    ///         for line in lines {
    ///             if let Ok(line) = line {
    ///                 println!("Processing: {}", line);
    ///             }
    ///         }
    ///     },
    ///     Err(e) => eprintln!("Failed to read file: {}", e)
    /// }
    /// ```
    ///
    /// Use `read_streaming()` when you need to:
    /// - Process very large files without loading them entirely into memory
    /// - Parse or filter lines before any trimming
    /// - Have more control over the line processing
    ///
    /// For simple cases where you just need trimmed input, prefer using `read()`.
    /// However, if you need to combine streaming with trimming, here's how:
    /// ```
    /// use input_reader::{InputReader, trim_empty_lines};
    ///
    /// let reader = InputReader::new()
    ///     .with_path("large_file.txt");
    ///
    /// if let Ok(lines) = reader.read_streaming() {
    ///     // Collect lines for trimming
    ///     let collected: Vec<String> = lines
    ///         .map(|line| line.expect("Failed to read line"))
    ///         .collect();
    ///     
    ///     // Trim empty lines from start and end
    ///     let trimmed = trim_empty_lines(&collected);
    ///     
    ///     // Process trimmed lines
    ///     for line in trimmed {
    ///         println!("Processing: {}", line);
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns `ReadError::StreamingRequiresPath` if no path was set.
    pub fn read_streaming(&self) -> Result<Lines<BufReader<File>>, ReadError> {
        match &self.path {
            Some(path) => {
                let file = File::open(path).map_err(ReadError::FileReadError)?;
                Ok(BufReader::new(file).lines())
            }
            None => Err(ReadError::StreamingRequiresPath),
        }
    }
}

/// Trims leading and trailing empty lines from a slice of strings.
/// Empty lines are those with zero length. Lines containing only whitespace
/// are not considered empty.
///
/// The function returns references to the original strings, avoiding allocations
/// when possible. If owned strings are needed, the results can be converted using
/// `String::from` or `to_string()`.
///
/// # Arguments
/// * `lines` - A slice of strings to process. Can be either owned (`String`) or
///            borrowed (`&str`) due to the generic bound `S: AsRef<str>`.
///
/// # Returns
/// A new `Vec<&str>` with references to the lines, excluding leading and trailing
/// empty lines.
///
/// # Examples
/// ```
/// use input_reader::trim_empty_lines;
///
/// // Using with string literals (&str)
/// let lines = ["", "hello", "world", ""];
/// let trimmed = trim_empty_lines(&lines);
/// assert_eq!(trimmed, vec!["hello", "world"]);
///
/// // Using with owned Strings
/// let owned = vec![String::from(""), String::from("hello")];
/// let trimmed = trim_empty_lines(&owned);
/// assert_eq!(trimmed, vec!["hello"]);
///
/// // Empty lines between content are preserved
/// let lines = ["", "first", "", "", "last", ""];
/// let trimmed = trim_empty_lines(&lines);
/// assert_eq!(trimmed, vec!["first", "", "", "last"]);
///
/// // Converting results to owned Strings if needed
/// let lines = ["", "hello", "world", ""];
/// let owned_results: Vec<String> = trim_empty_lines(&lines)
///     .into_iter()
///     .map(String::from)
///     .collect();
/// assert!(matches!(&owned_results[0], String));
/// assert_eq!(owned_results, vec!["hello", "world"]);
///
/// // Whitespace-only lines are preserved
/// let lines = ["", "  ", "hello", ""];
/// let trimmed = trim_empty_lines(&lines);
/// assert_eq!(trimmed, vec!["  ", "hello"]);
/// ```
pub fn trim_empty_lines<S: AsRef<str>>(lines: &[S]) -> Vec<&str> {
    if lines.is_empty() {
        return Vec::new();
    }

    // Find start index (first non-empty line)
    let start = lines
        .iter()
        .position(|line| !line.as_ref().is_empty())
        .unwrap_or(lines.len());

    // If all lines were empty, return empty vec
    if start == lines.len() {
        return Vec::new();
    }

    // Find end index (last non-empty line)
    let end = lines
        .iter()
        .rposition(|line| !line.as_ref().is_empty())
        .unwrap_or(0)
        + 1;

    // Allocate vec with exact capacity needed
    let mut result = Vec::with_capacity(end - start);

    // Single pass to collect references
    result.extend(lines[start..end].iter().map(|s| s.as_ref()));

    result
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
        .into_iter()
        .map(String::from)
        .collect()
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

    Ok(trim_empty_lines(&lines)
        .into_iter()
        .map(String::from)
        .collect())
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

    Ok(trim_empty_lines(&lines)
        .into_iter()
        .map(String::from)
        .collect())
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
