//! Input reader library for puzzle solutions.
//!
//! This library provides a flexible way to read input from multiple sources:
//! - Files (default for puzzle input)
//! - Command-line arguments
//! - Standard input
//!
//! The main entry point is [`read_input`], which handles argument parsing
//! and returns input that can be consumed as lines or characters.

mod args;
mod paths;
mod terminal;
mod types;

use crate::args::parse_args;
use crate::paths::{find_input_file_path, get_identifier};
use crate::terminal::{print_help, print_no_input, prompt_overwrite_confirmation};
pub use crate::types::{Error, Input, Outcome};
use crate::types::{InputMethod, InternalError};
use std::{fs, io::Write};

/// Reads input for a puzzle from various sources.
///
/// This is the main entry point for the input reader library. It parses command-line
/// arguments and loads input from the appropriate source (file, args, or stdin).
///
/// The identifier is automatically derived from the current executable name.
///
/// # Returns
///
/// * `Ok(Outcome::Exit)` - The caller should exit the application
/// * `Ok(Outcome::Continue(input))` - Input is ready for processing
/// * `Err(Error)` - A path or I/O error occurred
pub fn read_input() -> Result<Outcome, Error> {
    let identifier = get_identifier()?;
    let args = parse_args();

    if args.help {
        print_help(&identifier);
        return Ok(Outcome::Exit);
    }

    let input = match args.input {
        InputMethod::Auto => read_input_auto(&identifier, &args.data),
        InputMethod::File => read_input_file(&identifier),
        InputMethod::Args => read_input_args(&args.data),
        InputMethod::Stdin => read_input_stdin(),
    };

    // Handle internal errors
    let input = match input {
        Ok(input) => input,
        Err(InternalError::NoInput) => {
            print_no_input();
            return Ok(Outcome::Exit);
        }
        Err(InternalError::Path(e)) => return Err(e),
        Err(InternalError::Io(e)) => return Err(e.into()),
    };

    if args.save {
        // Only save if input is from memory (args/stdin), not from file
        if let Input::Memory(ref lines) = input {
            let input_file_path = find_input_file_path(&identifier).map_err(|e| match e {
                InternalError::NoInput | InternalError::Io(_) => {
                    unreachable!("find_input_file_path never returns NoInput or Io")
                }
                InternalError::Path(p) => p,
            })?;

            // Check if file exists and prompt for confirmation if needed
            if input_file_path.exists() && !args.force && !prompt_overwrite_confirmation() {
                // User declined to overwrite, skip saving
                return Ok(input.into());
            }

            let mut file = fs::File::create(input_file_path)?;
            for line in lines {
                writeln!(file, "{line}")?;
            }
        }
    }

    Ok(input.into())
}

fn read_input_auto(identifier: &str, data: &[String]) -> Result<Input, InternalError> {
    read_input_file(identifier)
        .or_else(|_| read_input_args(data))
        .or_else(|_| read_input_stdin())
}

fn read_input_file(identifier: &str) -> Result<Input, InternalError> {
    use std::fs::File;
    use std::io::BufReader;

    let input_file_path = find_input_file_path(identifier)?;

    if !input_file_path.exists() {
        return Err(InternalError::NoInput);
    }

    let input_file = File::open(&input_file_path).map_err(|e| InternalError::Path(e.into()))?;
    let reader = BufReader::new(input_file);

    Ok(Input::File(reader))
}

fn read_input_args(data: &[String]) -> Result<Input, InternalError> {
    if data.is_empty() {
        Err(InternalError::NoInput)
    } else {
        Ok(Input::Memory(data.to_vec()))
    }
}

#[expect(
    clippy::arithmetic_side_effects,
    reason = "blank_count is reset before reaching overflow"
)]
fn read_input_stdin() -> Result<Input, InternalError> {
    use crate::terminal::print_request_for_input;
    use std::io::{self, BufRead};

    print_request_for_input();

    let stdin = io::stdin();
    let mut lines = Vec::new();
    let mut blank_count = 0;

    for line in stdin.lock().lines() {
        let line = line.map_err(InternalError::Io)?;

        if line.is_empty() {
            blank_count += 1;
            if blank_count >= 2 {
                break;
            }
            lines.push(line);
        } else {
            blank_count = 0;
            lines.push(line);
        }
    }

    // Remove leading and trailing blank lines
    if lines.first().is_some_and(String::is_empty) {
        lines.remove(0);
    }
    if lines.last().is_some_and(String::is_empty) {
        lines.pop();
    }

    if lines.is_empty() {
        return Err(InternalError::NoInput);
    }

    Ok(Input::Memory(lines))
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "unwrap is okay in tests")]
mod tests {
    use super::*;

    #[test]
    fn read_input_args_returns_memory_input() {
        let data = vec!["line1".to_string(), "line2".to_string()];
        let result = read_input_args(&data);
        assert!(result.is_ok());

        let input = result.unwrap();
        let lines: Vec<String> = input.lines().map(|r| r.unwrap()).collect();
        assert_eq!(lines, vec!["line1", "line2"]);
    }

    #[test]
    fn read_input_args_returns_no_input_for_empty() {
        let data: Vec<String> = vec![];
        let result = read_input_args(&data);
        assert!(matches!(result, Err(InternalError::NoInput)));
    }
}
