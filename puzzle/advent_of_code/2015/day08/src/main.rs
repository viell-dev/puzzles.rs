use input_reader::InputReader;

fn string_character_length<S: AsRef<str>>(string: S) -> usize {
    string.as_ref().chars().count()
}

fn string_memory_length<S: AsRef<str>>(string: S) -> usize {
    let input = string.as_ref();
    let mut result = String::with_capacity(input.len());
    let mut chars = input[1..input.len() - 1].chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' && chars.peek() == Some(&'\\') {
            chars.next(); // consume peeked \
            result.push('\\');
        } else if c == '\\' && chars.peek() == Some(&'"') {
            chars.next(); // consume peeked "
            result.push('"');
        } else if c == '\\' && chars.peek() == Some(&'x') {
            chars.next(); // consume peeked x

            // Look ahead for exactly two hex digits and consume them if they are hex digits
            let mut hex = String::with_capacity(2);
            for _ in 0..2 {
                match chars.peek() {
                    Some(c) if c.is_ascii_hexdigit() => {
                        hex.push(chars.next().expect("it should exist"));
                    }
                    _ => break,
                }
            }

            // Only convert if we got exactly 2 hex digits
            if hex.len() == 2 {
                if let Ok(ascii) = u8::from_str_radix(&hex, 16) {
                    result.push(ascii as char);
                    continue;
                }
            }

            // If conversion failed, preserve original sequence
            result.push('\\');
            result.push('x');
            result.push_str(&hex);
        } else {
            result.push(c);
        }
    }

    result.chars().count()
}

fn string_encoded_length<S: AsRef<str>>(string: S) -> usize {
    format!(
        "\"{}\"",
        string.as_ref().replace('\\', "\\\\").replace('"', "\\\"")
    )
    .chars()
    .count()
}

fn main() {
    let input_reader = InputReader::new().with_path("./input.txt");

    let (character_sum, memory_sum, encoded_sum) = input_reader
        .read_streaming()
        .expect("failed to read input")
        .map(|line| line.expect("failed to read line"))
        .filter(|line| !line.trim().is_empty())
        .fold((0, 0, 0), |(chars, mem, enc), line| {
            (
                chars + string_character_length(&line),
                mem + string_memory_length(&line),
                enc + string_encoded_length(&line),
            )
        });

    let diff1 = character_sum - memory_sum;
    let diff2 = encoded_sum - character_sum;

    println!("literal - memory: {}, encoded - literal: {}", diff1, diff2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_character_length() {
        assert_eq!(string_character_length(r#""""#), 2);
        assert_eq!(string_character_length(r#""abc""#), 5);
        assert_eq!(string_character_length(r#""aaa\"aaa""#), 10);
        assert_eq!(string_character_length(r#""\x27""#), 6);
        assert_eq!(string_character_length(r#""\xa8br\x8bjr\"""#), 16);
    }

    #[test]
    fn test_string_memory_length() {
        assert_eq!(string_memory_length(r#""""#), 0);
        assert_eq!(string_memory_length(r#""abc""#), 3);
        assert_eq!(string_memory_length(r#""aaa\"aaa""#), 7);
        assert_eq!(string_memory_length(r#""\x27""#), 1);
        assert_eq!(string_memory_length(r#""\xa8br\x8bjr\"""#), 7);
    }

    #[test]
    fn test_string_encoded_length() {
        assert_eq!(string_encoded_length(r#""""#), 6);
        assert_eq!(string_encoded_length(r#""abc""#), 9);
        assert_eq!(string_encoded_length(r#""aaa\"aaa""#), 16);
        assert_eq!(string_encoded_length(r#""\x27""#), 11);
        assert_eq!(string_encoded_length(r#""\xa8br\x8bjr\"""#), 24);
    }

    #[test]
    fn test_string_diff() {
        let input = [
            r#""""#,
            r#""abc""#,
            r#""aaa\"aaa""#,
            r#""\x27""#,
            r#""\xa8br\x8bjr\"""#,
        ];

        let character_sum = input.iter().map(string_character_length).sum::<usize>();
        let memory_sum = input.iter().map(string_memory_length).sum::<usize>();
        let encoded_sum = input.iter().map(string_encoded_length).sum::<usize>();

        assert_eq!(character_sum, 39);
        assert_eq!(memory_sum, 18);
        assert_eq!(encoded_sum, 66);
        assert_eq!(character_sum - memory_sum, 21);
        assert_eq!(encoded_sum - character_sum, 27);
    }
}
