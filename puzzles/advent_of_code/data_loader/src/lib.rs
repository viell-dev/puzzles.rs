//! # Advent of Code Data Loader
//!
//! The library provides a function to load data from a file into a `BufReader`.
//!
//! The files are assumed to be located in a `data` directory in the root of the
//! project.
//!
//! The filenames should be alphanumeric and may contain underscores. The files
//! should have a `.txt` extension

use std::fs::File;
use std::io::{BufReader, Error};

/// The directory where the data files are located.
const DATA_DIR: &str = "../data/";

/// The extension of the data files.
const DATA_EXT: &str = ".txt";

/// Load the data file with the given filename into a `BufReader`.
///
/// The files are assumed to be located in a `data` directory in the root of the
/// project.
///
/// The filenames should be alphanumeric and may contain underscores. The files
/// should have a `.txt` extension
///
/// # Arguments
///
/// * `filename` - The name of the file to load.
///
/// # Examples
///
/// ```rust
/// use viell_puzzles_advent_of_code_data_loader::load_data;
/// use std::io::BufRead;
///
/// // Load the data from "./data/input.txt" into a BufReader.
/// let data = load_data("input").unwrap();
///
/// // Print each line of the data file.
/// for line in data.lines() {
///     println!("{}", line.unwrap());
/// }
/// ```
pub fn load_data(filename: &str) -> Result<BufReader<File>, Error> {
    // Validate the filename.
    filename.chars().for_each(|c| {
        if !c.is_alphanumeric() && c != '_' {
            panic!("Invalid filename: {filename}");
        }
    });

    // Construct the path to the data file.
    let path = format!("{DATA_DIR}{filename}{DATA_EXT}");

    // Load the data file into a BufReader.
    File::open(path).map(BufReader::new)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{ErrorKind, Read};

    /// The name of the test data file.
    const FILE_NAME: &str = "MyTestFile_01";

    /// The contents of the test data file.
    const FILE_DATA: &str = "Hello, world!";

    /// An invalid filename.
    const INVALID_FILE_NAME: &str = "my-kebab-file";

    /// The name of a missing file.
    const MISSING_FILE_NAME: &str = "missing_file";

    /// Test loading the data file.
    #[test]
    fn test_loading_data() {
        // Load the data file.
        let data = load_data(FILE_NAME);

        // Assert that the data file was not loaded successfully.
        assert!(data.is_ok());

        // Load the contents of the data file into a string.
        let mut buf = String::new();
        data.unwrap().read_to_string(&mut buf).unwrap();

        // Assert that the contents of the data file are correct.
        assert_eq!(buf, FILE_DATA.to_owned());
    }

    /// Test loading the data file with an invalid filename.
    #[test]
    #[should_panic(expected = "Invalid filename")]
    fn test_loading_data_invalid_filename() {
        // Load the data file with an invalid filename.
        let _ = load_data(INVALID_FILE_NAME);
    }

    /// Test loading a missing data file.
    #[test]
    fn test_loading_missing_data() {
        // Load the data file.
        let data = load_data(MISSING_FILE_NAME);

        // Assert that the data file was not loaded successfully.
        assert!(data.is_err());

        // Assert that the error is a "file not found" error.
        assert_eq!(data.err().unwrap().kind(), ErrorKind::NotFound);
    }
}
