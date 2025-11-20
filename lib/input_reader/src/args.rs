//! Command-line argument parsing for puzzle input handling.
//!
//! This module provides a fail-safe argument parser that prioritizes usability over strictness.
//! Unknown flags and invalid values are treated as data rather than causing errors.

use crate::types::InputMethod;
use std::env;

/// Parsed command-line arguments.
///
/// Contains the flags and data extracted from command-line arguments.
#[derive(Debug, PartialEq)]
pub(crate) struct ParsedArgs {
    /// Whether help was requested (`--help` or `-h`).
    pub help: bool,
    /// The input method to use (`--input` or `-i`).
    pub input: InputMethod,
    /// Whether to save the input (`--save` or `-s`).
    pub save: bool,
    /// Whether to force operations without prompts (`--force` or `-f`).
    pub force: bool,
    /// Positional arguments and unrecognized flags treated as data.
    pub data: Vec<String>,
}

/// Parses command-line arguments from the environment.
///
/// This is a fail-safe parser that treats unknown flags and invalid values as data.
///
/// # Supported flags
///
/// - `--help`, `-h`: Request help
/// - `--input [method]`, `-i [method]`: Set input method
///   - Valid values: `file`, `args`, `stdin`
///   - Value is optional; if omitted or invalid, defaults to `file`
/// - `--save`, `-s`: Enable save mode
/// - `--force`, `-f`: Force operations without prompts
/// - `--`: Stop parsing flags, treat everything after as data
///
/// # Behavior
///
/// - Repeated flags overwrite previous values (no errors)
/// - Unknown flags are added to data
/// - Invalid input method values default to `File` and the value becomes data
/// - Short flags can be grouped (e.g., `-isf` for input, save, and force)
/// - Only the last flag in a group can take a value (e.g., `-sfi stdin`)
pub(crate) fn parse_args() -> ParsedArgs {
    parse_args_from(env::args().skip(1))
}

/// Parses arguments from an iterator.
fn parse_args_from(args: impl Iterator<Item = String>) -> ParsedArgs {
    let mut help = false;
    let mut input = InputMethod::Auto;
    let mut save = false;
    let mut force = false;
    let mut data = Vec::new();

    let mut args_iter = args.peekable();
    let mut parse_flags = true;

    while let Some(arg) = args_iter.next() {
        let arg = arg.trim();

        if arg == "--" {
            parse_flags = false;
            continue;
        }

        if !parse_flags {
            data.push(arg.to_string());
            continue;
        }

        if arg.starts_with("--") && arg.len() > 2 {
            // Individual flags
            match arg {
                "--help" | "-h" => help = true,
                "--input" | "-i" => {
                    input = parse_input_value(&mut args_iter);
                }
                "--save" | "-s" => save = true,
                "--force" | "-f" => force = true,
                _ => data.push(arg.to_string()),
            }
        } else if arg.starts_with('-') && arg.len() > 1 {
            // Grouped short flags
            for (pos, flag) in arg[1..].chars().enumerate() {
                #[expect(
                    clippy::arithmetic_side_effects,
                    reason = "we already checked the length"
                )]
                let is_last_flag = pos == arg.len() - 2;

                match flag {
                    'h' => help = true,
                    'i' => {
                        if is_last_flag {
                            input = parse_input_value(&mut args_iter);
                        } else {
                            input = InputMethod::File;
                        }
                    }
                    's' => save = true,
                    'f' => force = true,
                    _ => data.push(arg.to_string()),
                }
            }
        } else {
            data.push(arg.to_string());
        }
    }

    ParsedArgs {
        help,
        input,
        save,
        force,
        data,
    }
}

/// Parses the input method value from the next argument.
///
/// Peeks at the next argument and returns the corresponding `InputMethod`.
/// Only consumes the argument if it's a valid input method value.
/// Invalid or missing values default to `File`, leaving unknown values as data.
fn parse_input_value(
    args_iter: &mut std::iter::Peekable<impl Iterator<Item = String>>,
) -> InputMethod {
    match args_iter.peek() {
        Some(value) => match value.as_ref() {
            "file" => {
                args_iter.next();
                InputMethod::File
            }
            "args" => {
                args_iter.next();
                InputMethod::Args
            }
            "stdin" => {
                args_iter.next();
                InputMethod::Stdin
            }
            _ => InputMethod::File,
        },
        _ => InputMethod::File,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    /// Helper to create args from string slices
    fn args(items: &[&str]) -> impl Iterator<Item = String> {
        items
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .into_iter()
    }

    #[test]
    fn empty_args() {
        let result = parse_args_from(args(&[]));
        assert_eq!(
            result,
            ParsedArgs {
                help: false,
                input: InputMethod::Auto,
                save: false,
                force: false,
                data: vec![],
            }
        );
    }

    #[test]
    fn help_long_flag() {
        let result = parse_args_from(args(&["--help"]));
        assert!(result.help);
    }

    #[test]
    fn help_short_flag() {
        let result = parse_args_from(args(&["-h"]));
        assert!(result.help);
    }

    #[test]
    fn input_file_long() {
        let result = parse_args_from(args(&["--input", "file"]));
        assert_eq!(result.input, InputMethod::File);
    }

    #[test]
    fn input_args_long() {
        let result = parse_args_from(args(&["--input", "args"]));
        assert_eq!(result.input, InputMethod::Args);
    }

    #[test]
    fn input_stdin_long() {
        let result = parse_args_from(args(&["--input", "stdin"]));
        assert_eq!(result.input, InputMethod::Stdin);
    }

    #[test]
    fn input_short_flag() {
        let result = parse_args_from(args(&["-i", "args"]));
        assert_eq!(result.input, InputMethod::Args);
    }

    #[test]
    fn save_long_flag() {
        let result = parse_args_from(args(&["--save"]));
        assert!(result.save);
    }

    #[test]
    fn save_short_flag() {
        let result = parse_args_from(args(&["-s"]));
        assert!(result.save);
    }

    #[test]
    fn grouped_short_flags() {
        let result = parse_args_from(args(&["-hs"]));
        assert!(result.help);
        assert!(result.save);
    }

    #[test]
    fn grouped_flags_with_input_last() {
        let result = parse_args_from(args(&["-hsi", "stdin"]));
        assert!(result.help);
        assert!(result.save);
        assert_eq!(result.input, InputMethod::Stdin);
    }

    #[test]
    fn grouped_flags_input_not_last_defaults_to_file() {
        let result = parse_args_from(args(&["-his"]));
        assert!(result.help);
        assert!(result.save);
        assert_eq!(result.input, InputMethod::File);
    }

    #[test]
    fn invalid_input_value_becomes_data() {
        let result = parse_args_from(args(&["--input", "foo"]));
        assert_eq!(result.input, InputMethod::File);
        assert_eq!(result.data, vec!["foo"]);
    }

    #[test]
    fn missing_input_value_defaults_to_file() {
        let result = parse_args_from(args(&["--input"]));
        assert_eq!(result.input, InputMethod::File);
    }

    #[test]
    fn unknown_long_flag_becomes_data() {
        let result = parse_args_from(args(&["--unknown"]));
        assert_eq!(result.data, vec!["--unknown"]);
    }

    #[test]
    fn positional_args_become_data() {
        let result = parse_args_from(args(&["foo", "bar"]));
        assert_eq!(result.data, vec!["foo", "bar"]);
    }

    #[test]
    fn double_dash_separator() {
        let result = parse_args_from(args(&["--", "--help", "-s"]));
        assert!(!result.help);
        assert!(!result.save);
        assert_eq!(result.data, vec!["--help", "-s"]);
    }

    #[test]
    fn repeated_flags_overwrite() {
        let result = parse_args_from(args(&["--input", "file", "--input", "stdin"]));
        assert_eq!(result.input, InputMethod::Stdin);
    }

    #[test]
    fn mixed_flags_and_data() {
        let result = parse_args_from(args(&["-hs", "--input", "args", "data1", "data2"]));
        assert!(result.help);
        assert!(result.save);
        assert_eq!(result.input, InputMethod::Args);
        assert_eq!(result.data, vec!["data1", "data2"]);
    }

    #[test]
    fn flags_after_positional_args() {
        let result = parse_args_from(args(&["data", "--help"]));
        assert!(result.help);
        assert_eq!(result.data, vec!["data"]);
    }

    #[test]
    fn data_after_double_dash_includes_flags() {
        let result = parse_args_from(args(&["--save", "--", "--input", "file"]));
        assert!(result.save);
        assert_eq!(result.input, InputMethod::Auto);
        assert_eq!(result.data, vec!["--input", "file"]);
    }

    #[test]
    fn force_long_flag() {
        let result = parse_args_from(args(&["--force"]));
        assert!(result.force);
    }

    #[test]
    fn force_short_flag() {
        let result = parse_args_from(args(&["-f"]));
        assert!(result.force);
    }

    #[test]
    fn grouped_flags_with_force() {
        let result = parse_args_from(args(&["-hsf"]));
        assert!(result.help);
        assert!(result.save);
        assert!(result.force);
    }

    #[test]
    fn grouped_flags_with_force_and_input() {
        let result = parse_args_from(args(&["-sfi", "stdin"]));
        assert!(result.save);
        assert!(result.force);
        assert_eq!(result.input, InputMethod::Stdin);
    }
}
