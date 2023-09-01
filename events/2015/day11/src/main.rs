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
    let next_password = get_next_password(input);
    let next_next_password = get_next_password(&next_password);

    (next_password, next_next_password)
}

fn is_valid_password(password: &str) -> bool {
    // If all characters are ASCII lowercase
    let is_all_ascii_lowercase = |p: &str| p.chars().all(|c| c.is_ascii_lowercase());

    let has_sequence_of_three_consecutive_characters = |p: &str| {
        for window in p.chars().collect::<Vec<_>>().windows(3) {
            let [a, b, c] = if let [a, b, c] = window {
                [*a, *b, *c]
            } else {
                panic!("Invalid input");
            };

            if (b as u32 == a as u32 + 1) && (c as u32 == b as u32 + 1) {
                return true;
            }
        }

        false
    };

    let does_hot_have_forbidden_letters =
        |p: &str| !p.chars().any(|c| matches!(c, 'i' | 'o' | 'l'));

    let has_two_non_overlapping_pairs_of_letters = |p: &str| {
        let mut first_pair: Option<char> = None;
        for window in p.chars().collect::<Vec<_>>().windows(2) {
            let [a, b] = if let [a, b] = window {
                [*a, *b]
            } else {
                panic!("Invalid input.");
            };

            if a == b {
                if first_pair.is_none() {
                    first_pair = Some(a);
                } else if Some(a) == first_pair {
                    continue;
                } else if Some(a) != first_pair {
                    return true;
                }
            }
        }

        false
    };

    is_all_ascii_lowercase(password)
        && has_sequence_of_three_consecutive_characters(password)
        && does_hot_have_forbidden_letters(password)
        && has_two_non_overlapping_pairs_of_letters(password)
}

fn get_next_password(password: &str) -> String {
    let mut incremented_password = increment_password(password);

    while !is_valid_password(&incremented_password) {
        incremented_password = increment_password(&incremented_password);
    }

    incremented_password
}

fn increment_password(password: &str) -> String {
    // Initialize a carry bit.
    let mut carry = 1;
    // Initialize a result string.
    let mut result = String::new();

    // Increment every character in the password in reverse.
    for (i, c) in password.chars().rev().enumerate() {
        let mut ord = c as u8 + carry;

        if ord > b'z' {
            ord = b'a';
            carry = 1;
        } else {
            carry = 0;
        }

        result.insert(0, ord as char);

        if carry == 0 {
            println!("Password: {}", password);
            println!("Result before: {}", result);
            result.insert_str(0, &password[0..password.len() - i + 1]);
            println!("Result after: {}", result);
            break;
        }
    }

    if carry == 1 {
        result.insert(0, 'a');
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

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

        // The next passwords should be valid.
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
