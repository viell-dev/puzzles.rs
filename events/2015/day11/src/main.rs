//! Day 11: Corporate Policy

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (next_password, next_next_password) = get_answers(input);

    println!("Santas next password should be: {}", next_password);
    println!(
        "The next password after that would be: {}",
        next_next_password
    );
}

/// Get the answers for the puzzle.
fn get_answers(input: &str) -> (String, String) {
    // Get the next password.
    let next_password = get_next_password(input);
    // Get the password after the one above.
    let next_next_password = get_next_password(&next_password);

    // Return the answers.
    (next_password, next_next_password)
}

/// Checks if a password is valid.
fn is_valid_password(password: &str) -> bool {
    // Checks if all characters are ASCII lowercase
    let is_all_ascii_lowercase = |p: &str| p.chars().all(|c| c.is_ascii_lowercase());

    // Checks if the password has a sequence of three consecutive characters
    let has_sequence_of_three_consecutive_characters = |p: &str| {
        // For each window of three characters in the password.
        for window in p.chars().collect::<Vec<_>>().windows(3) {
            // Separate the window into three characters.
            let [a, b, c] = if let [a, b, c] = window {
                [*a, *b, *c]
            } else {
                panic!("Invalid input");
            };

            // If the three characters are consecutive, return true.
            if (b as u32 == a as u32 + 1) && (c as u32 == b as u32 + 1) {
                return true;
            }
        }

        // Else return false.
        false
    };

    // Checks if the password has any of the forbidden letters
    let does_hot_have_forbidden_letters =
        |p: &str| !p.chars().any(|c| matches!(c, 'i' | 'o' | 'l'));

    // Checks if the password has two non-overlapping pairs of letters
    let has_two_non_overlapping_pairs_of_letters = |p: &str| {
        // Initialize the first pair.
        let mut first_pair: Option<char> = None;

        // For each window of two characters in the password.
        for window in p.chars().collect::<Vec<_>>().windows(2) {
            // Separate the window into two characters.
            let [a, b] = if let [a, b] = window {
                [*a, *b]
            } else {
                panic!("Invalid input.");
            };

            // If the two characters are the same.
            if a == b {
                // If the first pair is None, set it to the current pair.
                if first_pair.is_none() {
                    first_pair = Some(a);
                }
                /* Else if the first pair is the same as the current pair,
                continue. */
                else if Some(a) == first_pair {
                    continue;
                }
                /* Else if the first pair is not the same as the current pair,
                return true. */
                else if Some(a) != first_pair {
                    return true;
                }
            }
        }

        // Else return false.
        false
    };

    // Lazily check if the password is valid and return the result.
    is_all_ascii_lowercase(password)
        && has_sequence_of_three_consecutive_characters(password)
        && does_hot_have_forbidden_letters(password)
        && has_two_non_overlapping_pairs_of_letters(password)
}

/// Get the next valid password.
fn get_next_password(password: &str) -> String {
    // Initialize an incremented password.
    let mut incremented_password = increment_password(password);

    // While the incremented password is not valid.
    while !is_valid_password(&incremented_password) {
        // Increment the incremented password.
        incremented_password = increment_password(&incremented_password);
    }

    // Return the valid incremented password.
    incremented_password
}

/// Increment a password.
fn increment_password(password: &str) -> String {
    // Initialize a carry bit.
    let mut carry = 1;
    // Initialize a result string.
    let mut result = String::new();

    // For every character in the password in reverse order.
    for (i, c) in password.chars().rev().enumerate() {
        // Increment the character by the carry bit.
        let mut ord = c as u8 + carry;

        /* If the character is greater than 'z',
        set it to 'a' and set the carry bit to 1. */
        if ord > b'z' {
            ord = b'a';
            carry = 1;
        }
        // Else set the carry bit to 0.
        else {
            carry = 0;
        }

        // Insert the character into the result string.
        result.insert(0, ord as char);

        /* If the carry bit is 0, insert the rest of the password
        into the result string and break. */
        if carry == 0 {
            result.insert_str(0, &password[0..password.len() - 1 - i]);
            break;
        }
    }

    /* If the carry bit is 1, insert an 'a'
    at the beginning of the result string. */
    if carry == 1 {
        result.insert(0, 'a');
    }

    // Return the result string.
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the `increment_password` function.
    #[test]
    fn test_increment_password() {
        assert_eq!(increment_password("aaa"), "aab");
        assert_eq!(increment_password("aza"), "azb");
        assert_eq!(increment_password("az"), "ba");
        assert_eq!(increment_password("azz"), "baa");
        assert_eq!(increment_password("abzz"), "acaa");
        assert_eq!(increment_password("ba"), "bb");
        assert_eq!(increment_password("bab"), "bac");
        assert_eq!(increment_password("zz"), "aaa");
    }

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        assert_eq!(is_valid_password("hijklmmn"), false);
        assert_eq!(is_valid_password("abbceffg"), false);
        assert_eq!(is_valid_password("abbcegjk"), false);
        assert_eq!(get_next_password("abcdefgh"), "abcdffaa");
        assert_eq!(get_next_password("ghijklmn"), "ghjaabcc");

        // Extra tests: The next passwords should be valid.
        assert_eq!(is_valid_password("abcdffaa"), true);
        assert_eq!(is_valid_password("ghjaabcc"), true);
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(
            get_answers(input),
            (String::from("cqjxxyzz"), String::from("cqkaabcc"))
        );
    }
}
