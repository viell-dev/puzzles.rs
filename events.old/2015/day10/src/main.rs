//! Day 10: Elves Look, Elves Say

use std::str::FromStr;

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (look_and_say_40, look_and_say_50) = get_answers(input);

    println!(
        "The length of the result of applying the process 40 times is: {}",
        look_and_say_40
    );
    println!(
        "The length of the result of applying the process 50 times is: {}",
        look_and_say_50
    );
}

/// Get the answers for the puzzle.
fn get_answers(input: &str) -> (usize, usize) {
    // Initialize the look-and-say input.
    let mut look_and_say_input = String::from_str(input).unwrap();

    // Apply look-and-say 40 times.
    for _ in 0..40 {
        look_and_say_input = look_and_say(&look_and_say_input);
    }

    // Get the length of the look-and-say sequence after being applied 40 times.
    let look_and_say_40 = look_and_say_input.chars().count();

    // Apply look-and-say 10 times. Adding up to 50 times in total.
    for _ in 0..10 {
        look_and_say_input = look_and_say(&look_and_say_input);
    }

    // Get the length of the look-and-say sequence after being applied 50 times.
    let look_and_say_50 = look_and_say_input.chars().count();

    // Return the answers.
    (look_and_say_40, look_and_say_50)
}

/// Return the look-and-say sequence for the given input.
fn look_and_say(input: &str) -> String {
    // Initialize the result string.
    let mut result = String::new();
    // Get a peekable iterator over the characters in the input.
    let mut chars = input.chars().peekable();

    // Loop over the characters in the input.
    while let Some(c) = chars.next() {
        // Initialize the count.
        let mut count = 1;

        // Increment the count while the next character is the same.
        while chars.peek() == Some(&c) {
            chars.next();
            count += 1;
        }

        // Add the count to the result.
        result.push_str(&count.to_string());
        // Add the character to the result.
        result.push(c);
    }

    // Return the result.
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (360_154, 5_103_798));
    }
}
