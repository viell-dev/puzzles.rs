//! Day 8: Matchsticks

use std::str;

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (a, b) = get_answers(input);

    println!("a is: {}", a);
    println!("b is: {}", b);
}

/// Get the answers for the puzzle.
fn get_answers(input: &str) -> (usize, usize) {
    // Parse the input.
    let parsed_input = parse_input(input);

    // Get the number of characters in the parsed input.
    let parsed_chars = parsed_input.chars().count();
    // Get the number of characters of the input excluding line-breaks.
    let input_chars = input.chars().filter(|c| c != &'\n').count();

    /* Get the difference between the number of characters in the input
    and parsed input. */
    let diff = input_chars - parsed_chars;

    // Return the answers.
    (diff, 0)
}

/// Parse the input string, line by line, and return it as a continuous string
/// without line-breaks.
fn parse_input(input: &str) -> String {
    // Initialize a string to contain the parsed input.
    let mut parsed_input = String::new();

    // For every line in the input.
    for line in input.lines() {
        // Parse the line.
        let parsed_line = parse_line(line);
        // Push the parsed line to the parsed input.
        parsed_input.push_str(&parsed_line);
    }

    // Return the parsed input.
    parsed_input
}

/// Parse the unescaped string in the given line and return the escaped string.
fn parse_line(line: &str) -> String {
    // Initialize a string to contain the parsed line.
    let mut parsed_line = String::new();
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
                push a backslash to the parsed line. */
                Some('\\') => parsed_line.push('\\'),
                /* If the next character is a double-quote,
                push a double-quote to the parsed line. */
                Some('"') => parsed_line.push('"'),
                /* If the next character is the letter x, get the following
                two characters and parse them as a unicode code point.
                Then get the character of that code point and push it to
                the parsed line. */
                Some('x') => {
                    // Get next two characters.
                    let hex_chars = [chars.next().unwrap(), chars.next().unwrap()];
                    // Convert the array of characters to a string.
                    let hex = String::from_iter(hex_chars);
                    // Convert the hex to a u32.
                    let unicode = u32::from_str_radix(&hex, 16).unwrap();
                    // Get the character from the u32.
                    let ch = char::from_u32(unicode).unwrap();
                    // Push the character to the parsed line.
                    parsed_line.push(ch);
                }
                // Panic for unknown escape sequences.
                _ => panic!("Invalid escape sequence"),
            };
        } else {
            // Push characters that aren't backslash to the parsed line.
            parsed_line.push(c);
        }
    }

    // Return the parsed line.
    parsed_line
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        assert_eq!(parse_line(r#""""#), "");
        assert_eq!(parse_line(r#""abc""#), "abc");
        assert_eq!(parse_line(r#""aaa\"aaa""#), "aaa\"aaa");
        assert_eq!(parse_line(r#""\x27""#), "'");

        // Input for test.
        let input = "\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\"";

        // Parse the input.
        let parsed_input = parse_input(input);

        // Get the number of characters in the parsed input.
        let parsed_chars = parsed_input.chars().count();
        // Get the number of characters of the input excluding line-breaks.
        let input_chars = input.chars().filter(|c| c != &'\n').count();

        /* Get the difference between the number of characters in the input
        and parsed input. */
        let diff = input_chars - parsed_chars;

        assert_eq!(input_chars, 23);
        assert_eq!(parsed_chars, 11);
        assert_eq!(diff, 12);
    }

    /// Test the examples given for part 2 on the puzzle page.
    #[test]
    fn test_part2_examples() {}

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (1371, 0));
    }
}
