//! Day 8: Matchsticks

use std::str;

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (decoded_diff, encoded_diff) = get_answers(input);

    println!(
        "The number of characters of code for string {} {} {} ",
        "literals minus the number of characters in memory for the",
        "values of the strings in total for the entire file is:",
        decoded_diff
    );
    println!(
        "The total number of characters to represent {} {} {} ",
        "the newly encoded strings minus the number of characters",
        "of code in each original string literal is:",
        encoded_diff
    );
}

/// Get the answers for the puzzle.
fn get_answers(input: &str) -> (usize, usize) {
    // Get the number of characters of the input excluding line-breaks.
    let input_chars = input.chars().filter(|c| c != &'\n').count();

    // Decode the input.
    let decoded_chars = decode_input(input);
    /* Get the difference between the number of characters in the input
    and decoded input. */
    let decoded_diff = input_chars - decoded_chars;

    // Encode the input.
    let encoded_chars = encode_input(input);
    /* Get the difference between the number of characters in the input
    and encoded input. */
    let encoded_diff = encoded_chars - input_chars;

    // Return the answers.
    (decoded_diff, encoded_diff)
}

/// Decode the input string, line by line, and return it as a continuous string
/// without line-breaks.
fn decode_input(input: &str) -> usize {
    // Initialize a string to contain the decoded input.
    let mut decoded_input = String::new();

    // For every line in the input.
    for line in input.lines() {
        // Decode the line.
        let decoded_line = decode_line(line);
        // Push the decoded line to the decoded input.
        decoded_input.push_str(&decoded_line);
    }

    // Return the number of characters in the decoded input.
    decoded_input.chars().count()
}

fn encode_input(input: &str) -> usize {
    // Initialize a string to contain the encoded input.
    let mut encoded_input = String::new();

    // For every line in the input.
    for line in input.lines() {
        // Encode the line.
        let encoded_line = encode_line(line);
        // Push the encoded line to the encoded input.
        encoded_input.push_str(&encoded_line);
    }

    // Return the number of characters in the encoded input.
    encoded_input.chars().count()
}

/// Decode the given line and return it.
fn decode_line(line: &str) -> String {
    // Initialize a string to contain the decoded line.
    let mut decoded_line = String::new();
    /* Get an iterator over the characters in the line.
    Skipping the double-quotes at the start end end of the line */
    let mut chars = line[1..line.len() - 1].chars();

    // While there are characters left in the iterator.
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                // If there is no next character; do nothing.
                None => (),
                /* If the next character is a backslash,
                push a backslash to the decoded line. */
                Some('\\') => decoded_line.push('\\'),
                /* If the next character is a double-quote,
                push a double-quote to the decoded line. */
                Some('"') => decoded_line.push('"'),
                /* If the next character is the letter x, get the following
                two characters and Decode them as a unicode code point.
                Then get the character of that code point and push it to
                the decoded line. */
                Some('x') => {
                    // Get next two characters.
                    let hex_chars = [chars.next().unwrap(), chars.next().unwrap()];
                    // Convert the array of characters to a string.
                    let hex = String::from_iter(hex_chars);
                    // Convert the hex to a u32.
                    let unicode = u32::from_str_radix(&hex, 16).unwrap();
                    // Get the character from the u32.
                    let ch = char::from_u32(unicode).unwrap();
                    // Push the character to the decoded line.
                    decoded_line.push(ch);
                }
                // Panic for unknown escape sequences.
                _ => panic!("Invalid escape sequence"),
            };
        } else {
            // Push characters that aren't backslash to the decoded line.
            decoded_line.push(c);
        }
    }

    // Return the decoded line.
    decoded_line
}

/// Encode the given line and return it.
fn encode_line(line: &str) -> String {
    // Initialize a string to contain the encoded line.
    let mut encoded_line = String::new();

    // Push the starting double-quote to the encoded line.
    encoded_line.push('"');

    // Iterator over every character in the line.
    for c in line.chars() {
        match c {
            /* If the next character is a backslash,
            push a backslash to the encoded line. */
            '\\' => encoded_line.push_str(r#"\\"#),
            /* If the next character is a double-quote,
            push a double-quote to the encoded line. */
            '"' => encoded_line.push_str(r#"\""#),
            /* Push characters that aren't backslash or
            double-quote to the encoded line. */
            ch => encoded_line.push(ch),
        };
    }

    // Push the ending double-quote to the encoded line.
    encoded_line.push('"');

    // Return the encoded line.
    encoded_line
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        assert_eq!(decode_line(r#""""#), r#""#);
        assert_eq!(decode_line(r#""abc""#), r#"abc"#);
        assert_eq!(decode_line(r#""aaa\"aaa""#), r#"aaa"aaa"#);
        assert_eq!(decode_line(r#""\x27""#), r#"'"#);

        // Input for test.
        let input = r#"
""
"abc"
"aaa\"aaa"
"\x27"
"#
        .trim();

        // Get the number of characters of the input excluding line-breaks.
        let input_chars = input.chars().filter(|c| c != &'\n').count();

        // Decode the input.
        let decoded_chars = decode_input(input);

        /* Get the difference between the number of characters in the input
        and decoded input. */
        let diff = input_chars - decoded_chars;

        assert_eq!(input_chars, 23);
        assert_eq!(decoded_chars, 11);
        assert_eq!(diff, 12);
    }

    /// Test the examples given for part 2 on the puzzle page.
    #[test]
    fn test_part2_examples() {
        assert_eq!(encode_line(r#""""#), r#""\"\"""#);
        assert_eq!(encode_line(r#""abc""#), r#""\"abc\"""#);
        assert_eq!(encode_line(r#""aaa\"aaa""#), r#""\"aaa\\\"aaa\"""#);
        assert_eq!(encode_line(r#""\x27""#), r#""\"\\x27\"""#);

        // Input for test.
        let input = r#"
""
"abc"
"aaa\"aaa"
"\x27"
"#
        .trim();

        // Get the number of characters of the input excluding line-breaks.
        let input_chars = input.chars().filter(|c| c != &'\n').count();

        // Encode the input.
        let encoded_chars = encode_input(input);

        /* Get the difference between the number of characters in the input
        and encoded input. */
        let diff = encoded_chars - input_chars;

        assert_eq!(input_chars, 23);
        assert_eq!(encoded_chars, 42);
        assert_eq!(diff, 19);
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (1371, 2117));
    }
}
