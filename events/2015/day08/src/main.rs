//! Day 8: Matchsticks

use std::str;

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (a, b) = get_answers(input);

    println!("a is: {}", a);
    // 1070 too low
    // 1670 too high
    println!("b is: {}", b);
    todo!();
}

/// Get the answers for the puzzle.
fn get_answers(input: &str) -> (usize, usize) {
    let mut parsed_strings = String::new();

    for line in input.lines() {
        let parsed = parse_string(line);
        println!("Line: {} -> {}", line, parsed);
        parsed_strings.push_str(&parsed);
    }

    let input_len = input.chars().filter(|c| c != &'\n').count();
    let parsed_len = parsed_strings.chars().count();
    let diff = input_len - parsed_len;

    println!(
        "Input len: {}, Parsed len: {}, Diff: {}",
        input_len, parsed_len, diff
    );

    (diff, 0)
}

fn parse_string(line: &str) -> String {
    let mut parsed = String::new();
    let mut bytes = line[1..line.len() - 1].bytes();

    while let Some(byte) = bytes.next() {
        match byte {
            b'\\' => {
                match bytes.next() {
                    None => (),
                    Some(b'\\') => parsed.push('\\'),
                    Some(b'"') => parsed.push('"'),
                    Some(b'x') => {
                        let hex_bytes = [bytes.next().unwrap(), bytes.next().unwrap()];
                        let hex = str::from_utf8(&hex_bytes).unwrap();
                        let unicode = u32::from_str_radix(hex, 16).unwrap();
                        let c = char::from_u32(unicode).unwrap();
                        parsed.push(c);
                    }
                    _ => panic!("Invalid escape sequence"),
                };
            }
            _ => parsed.push(byte as char),
        }
    }

    parsed
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        assert_eq!(parse_string(r#""""#), "");
        assert_eq!(parse_string(r#""abc""#), "abc");
        assert_eq!(parse_string(r#""aaa\"aaa""#), "aaa\"aaa");
        assert_eq!(parse_string(r#""\x27""#), "'");

        let input = "\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\"";
        let mut parsed_strings = String::new();

        for line in input.lines() {
            let parsed = parse_string(line);
            parsed_strings.push_str(&parsed);
        }

        let input_len = input.chars().filter(|c| c != &'\n').count();

        let total_parsed: usize = parsed_strings.chars().count();

        let diff: usize = input_len - total_parsed;

        assert_eq!(input_len, 23);
        assert_eq!(total_parsed, 11);
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
