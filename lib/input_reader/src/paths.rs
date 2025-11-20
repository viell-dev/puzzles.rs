//! Path resolution for puzzle input files.
//!
//! This module provides functionality to locate the directory containing puzzle input files.
//! It uses different strategies depending on the build configuration:
//!
//! - **Debug builds**: Traverses up from `CARGO_MANIFEST_DIR` to find the repository root
//!   (identified by a `.git` directory), then returns the `input` subdirectory. This shared
//!   directory contains input files for all puzzles.
//! - **Release builds**: Returns the parent directory of the current executable, where
//!   input files are expected to be siblings of the binary.

use crate::types::{Error, InternalError};
use std::{env, path::PathBuf};

/// Gets the identifier from the current executable name.
///
/// Extracts the file stem from `current_exe()`.
///
/// # Errors
///
/// Returns [`Error::Io`] if the current executable path cannot be determined.
/// Returns [`Error::NotFound`] if the executable name cannot be extracted.
pub(crate) fn get_identifier() -> Result<String, Error> {
    let exe_path = env::current_exe().map_err(Error::Io)?;

    exe_path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(String::from)
        .ok_or(Error::NotFound)
}

pub(crate) fn find_input_file_path(identifier: &str) -> Result<PathBuf, InternalError> {
    let working_dir = find_working_dir()?;

    let input_file_path = if cfg!(debug_assertions) {
        working_dir.join(format!("{identifier}.txt"))
    } else {
        working_dir.join("input.txt")
    };

    Ok(input_file_path)
}

/// Finds the working directory containing puzzle input files.
///
/// In debug builds, this function traverses up from `CARGO_MANIFEST_DIR` to find the
/// repository root (identified by a `.git` directory), then returns the path to the
/// `input` subdirectory if it exists.
///
/// # Errors
///
/// Returns [`Error::Var`] if `CARGO_MANIFEST_DIR` is not set.
/// Returns [`Error::NotFound`] if no `.git` directory is found or if the `input`
/// subdirectory doesn't exist in the repository root.
#[cfg(debug_assertions)]
fn find_working_dir() -> Result<PathBuf, Error> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .map_err(Error::Var)?;

    find_working_dir_from(manifest_dir)
}

/// Finds the working directory containing puzzle input files.
///
/// In release builds, this function returns the parent directory of the current
/// executable, assuming input files are located alongside the binary.
///
/// # Errors
///
/// Returns [`Error::Io`] if the current executable path cannot be determined.
/// Returns [`Error::NotFound`] if the executable has no parent directory.
#[cfg(not(debug_assertions))]
fn find_working_dir() -> Result<PathBuf, Error> {
    let exe_path = env::current_exe().map_err(Error::Io)?;

    if let Some(parent) = exe_path.parent() {
        Ok(parent.to_path_buf())
    } else {
        Err(Error::NotFound)
    }
}

/// Traverses up from `start_dir` to find `.git`, then returns the `input` subdirectory.
#[cfg(debug_assertions)]
fn find_working_dir_from(start_dir: PathBuf) -> Result<PathBuf, Error> {
    let mut current_dir = start_dir;

    loop {
        if current_dir.join(".git").exists() {
            if current_dir.join("input").exists() {
                return Ok(current_dir.join("input"));
            }
            return Err(Error::NotFound);
        }

        match current_dir.parent() {
            Some(parent) if parent != current_dir => {
                current_dir = parent.to_path_buf();
            }
            _ => return Err(Error::NotFound),
        }
    }
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "unwrap is okay in tests")]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn find_input_from_dir_returns_input_directory() {
        // Create a temporary directory structure with .git and input
        let temp_dir = TempDir::new().unwrap();
        let git_dir = temp_dir.path().join(".git");
        let input_dir = temp_dir.path().join("input");

        fs::create_dir(&git_dir).unwrap();
        fs::create_dir(&input_dir).unwrap();

        let result = find_working_dir_from(temp_dir.path().to_path_buf());
        assert!(result.is_ok());
        assert!(result.unwrap().ends_with("input"));
    }

    #[test]
    fn find_input_from_dir_traverses_to_git_root() {
        // Create a temporary directory structure to test traversal
        let temp_dir = TempDir::new().unwrap();
        let git_dir = temp_dir.path().join(".git");
        let input_dir = temp_dir.path().join("input");
        let nested_dir = temp_dir.path().join("nested").join("deeply");

        fs::create_dir(&git_dir).unwrap();
        fs::create_dir(&input_dir).unwrap();
        fs::create_dir_all(&nested_dir).unwrap();

        let result = find_working_dir_from(nested_dir);
        assert_eq!(result.unwrap(), input_dir);
    }

    #[test]
    fn find_input_from_dir_returns_not_found_without_input_dir() {
        // Create a temporary directory with .git but no input
        let temp_dir = TempDir::new().unwrap();
        let git_dir = temp_dir.path().join(".git");

        fs::create_dir(&git_dir).unwrap();

        let result = find_working_dir_from(temp_dir.path().to_path_buf());
        assert!(matches!(result, Err(Error::NotFound)));
    }
}
