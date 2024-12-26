use std::collections::HashSet;

use input_reader::InputReader;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const OLD_FORBIDDEN: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn old_nice_string(line: &str) -> bool {
    // Create sliding window of two characters
    let window = line.chars().zip(line.chars().skip(1));

    // Requirements
    let mut vowels_found = 0;
    let mut repeat_found = false;

    for (index, (a, b)) in window.enumerate() {
        // Short-circuit if a forbidden sequence is encountered
        if OLD_FORBIDDEN.contains(&format!("{}{}", a, b).as_str()) {
            return false;
        }

        // Check if the first letter is a vowel
        if index == 0 && VOWELS.contains(&a) {
            vowels_found += 1;
        }

        // Check if any of the rest of the letters are vowels
        if VOWELS.contains(&b) {
            vowels_found += 1;
        }

        // Check if a sequence of duplicate characters occurs
        if a == b {
            repeat_found = true;
        }
    }

    // Check if the requirements where met
    vowels_found >= 3 && repeat_found
}

fn new_nice_string(line: &str) -> bool {
    // Create two sliding windows of two characters
    let window1 = line.chars().zip(line.chars().skip(1));
    let mut window2 = window1.clone();

    // Found pairs
    let mut pairs = HashSet::new();

    // Requirements
    let mut has_repeat_pair = false;
    let mut has_repeat_letter_with_spacer = false;

    for (index, current_pair) in window1.enumerate() {
        // Get previous pair if the is one
        let mut previous_pair = None;
        if index > 0 {
            previous_pair = window2.next();
        }

        // Check if pair has been seen before
        if pairs.contains(&current_pair) {
            has_repeat_pair = true;
        }

        // Add pair to list
        pairs.insert(current_pair);

        // If there is a previous pair
        if let Some(previous_pair) = previous_pair {
            // Short-circuit if a overlapping pair was found
            if previous_pair == current_pair {
                return false;
            }

            // Check if a repeat letter was found with a different letter in-between
            if previous_pair.0 != current_pair.0 && previous_pair.0 == current_pair.1 {
                has_repeat_letter_with_spacer = true;
            }
        }
    }

    // Check if the requirements where met
    has_repeat_pair && has_repeat_letter_with_spacer
}

fn main() {
    let input_reader = InputReader::new().with_path("./input.txt");
    let input = match input_reader.read() {
        Ok(lines) => lines
            .iter()
            .filter_map(|line| match line.trim() {
                line if !line.is_empty() => Some(line.to_owned()),
                _ => None,
            })
            .collect::<Vec<_>>(),
        Err(error) => panic!("Error reading input: {:#?}", error),
    };

    let old_nice_strings = input.iter().filter(|line| old_nice_string(line)).count();
    let new_nice_strings = input.iter().filter(|line| new_nice_string(line)).count();

    println!(
        "Old nice strings: {}, New nice strings: {}",
        old_nice_strings, new_nice_strings
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_old_nice_string() {
        assert!(old_nice_string("ugknbfddgicrmopn"));
        assert!(old_nice_string("aaa"));

        assert!(!old_nice_string("jchzalrnumimnmhp"));
        assert!(!old_nice_string("haegwjzuvuyypxyu"));
        assert!(!old_nice_string("dvszwmarrgswjxmb"));
        assert!(!old_nice_string("qjhvhtzxzqqjkmpb"));
        assert!(!old_nice_string("xxyxx"));
        assert!(!old_nice_string("uurcxstgmygtbstg"));
        assert!(!old_nice_string("ieodomkazucvgmuy"));
    }

    #[test]
    fn test_new_nice_string() {
        assert!(new_nice_string("qjhvhtzxzqqjkmpb"));
        assert!(new_nice_string("xxyxx"));

        assert!(!new_nice_string("ugknbfddgicrmopn"));
        assert!(!new_nice_string("aaa"));
        assert!(!new_nice_string("jchzalrnumimnmhp"));
        assert!(!new_nice_string("haegwjzuvuyypxyu"));
        assert!(!new_nice_string("dvszwmarrgswjxmb"));
        assert!(!new_nice_string("uurcxstgmygtbstg"));
        assert!(!new_nice_string("ieodomkazucvgmuy"));
    }
}
