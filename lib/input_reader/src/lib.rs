use std::{
    env::{self},
    fs::File,
    io::{self, BufRead, BufReader},
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
}

/// Trims leading and trailing empty lines from a vector of strings.
///
/// # Arguments
///
/// * `lines` - A `Vec<String>` containing the lines of input to be processed.
///
/// # Returns
///
/// A new `Vec<String>` with leading and trailing empty lines removed.
/// Lines are considered empty if they contain only whitespace.
fn trim_empty_lines(lines: Vec<String>) -> Vec<String> {
    let trimmed: Vec<_> = lines
        .into_iter()
        .skip_while(|line| line.is_empty())
        .collect();
    let trimmed = trimmed
        .into_iter()
        .rev()
        .skip_while(|line| line.is_empty())
        .collect::<Vec<_>>();
    trimmed.into_iter().rev().collect()
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
    trim_empty_lines(lines)
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

    Ok(trim_empty_lines(lines))
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

    Ok(trim_empty_lines(lines))
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
pub fn read_input(path: Option<&str>, message: Option<&str>) -> Result<Vec<String>, ReadError> {
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
        assert_eq!(trim_empty_lines(Vec::<String>::new()), Vec::<String>::new());
        assert_eq!(
            trim_empty_lines(vec!["   ".to_string(), "".to_string()]),
            vec!["   ".to_string()]
        );
        assert_eq!(
            trim_empty_lines(vec!["line1".to_string(), "line2".to_string()]),
            vec!["line1", "line2"]
        );
        assert_eq!(
            trim_empty_lines(vec![
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
        let content = "\nline1\nline2\nline3\n";

        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file
            .write_all(content.as_bytes())
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
}
