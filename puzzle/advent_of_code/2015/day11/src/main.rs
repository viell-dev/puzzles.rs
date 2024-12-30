use input_reader::InputReader;

fn has_straight_three(password: &str) -> bool {
    let chars: Vec<char> = password.chars().collect();

    for i in 0..chars.len() - 2 {
        if chars[i + 1] as u8 == chars[i] as u8 + 1 && chars[i + 2] as u8 == chars[i] as u8 + 2 {
            return true;
        }
    }

    false
}

fn has_forbidden_chars(password: &str) -> bool {
    password.contains('i') || password.contains('o') || password.contains('l')
}

fn has_two_pairs(password: &str) -> bool {
    let chars: Vec<char> = password.chars().collect();
    let mut pairs = Vec::new();

    let mut i = 0;
    while i < chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            pairs.push(chars[i]);
            i += 2;
        } else {
            i += 1;
        }
    }

    pairs.len() >= 2
}

fn increment_password(password: &str) -> String {
    let mut chars: Vec<char> = password.chars().collect();
    let mut i = chars.len() - 1;

    loop {
        if chars[i] == 'z' {
            chars[i] = 'a';
            if i == 0 {
                break;
            }
            i -= 1;
        } else {
            chars[i] = (chars[i] as u8 + 1) as char;
            break;
        }
    }

    chars.iter().collect()
}

fn is_valid_password(password: &str) -> bool {
    !has_forbidden_chars(password) && has_straight_three(password) && has_two_pairs(password)
}

fn find_next_password(current: &str) -> String {
    let mut password = current.to_string();
    loop {
        password = increment_password(&password);
        if is_valid_password(&password) {
            return password;
        }
    }
}

fn main() {
    let input_reader = InputReader::new().with_path("./input.txt");
    let input = match input_reader.read() {
        Ok(lines) => match lines.first() {
            Some(line) if !line.trim().is_empty() => line.trim().to_owned(),
            _ => panic!("Error reading input: Input was empty"),
        },
        Err(error) => panic!("Error reading input: {:#?}", error),
    };

    let next_password = find_next_password(&input);
    let next_password_again = find_next_password(&next_password);

    println!(
        "Next password: {}, Next password again: {}",
        next_password, next_password_again
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_straight_three() {
        assert!(has_straight_three("abc"));
        assert!(has_straight_three("bcd"));
        assert!(!has_straight_three("abd"));
    }

    #[test]
    fn test_has_forbidden_chars() {
        //spell-checker: disable
        assert!(has_forbidden_chars("abcdefgi"));
        assert!(has_forbidden_chars("abcdefgo"));
        assert!(has_forbidden_chars("abcdefgl"));
        assert!(!has_forbidden_chars("abcdefgh"));
        //spell-checker: enable
    }

    #[test]
    fn test_has_two_pairs() {
        //spell-checker: disable
        assert!(has_two_pairs("aabb"));
        assert!(has_two_pairs("aabcc"));
        assert!(!has_two_pairs("abcd"));
        //spell-checker: enable
    }

    #[test]
    fn test_increment_password() {
        assert_eq!(increment_password("xx"), "xy");
        assert_eq!(increment_password("xy"), "xz");
        assert_eq!(increment_password("xz"), "ya");
    }

    #[test]
    fn test_is_valid_password() {
        //spell-checker: disable
        assert!(!is_valid_password("hijklmmn"));
        assert!(!is_valid_password("abbceffg"));
        assert!(!is_valid_password("abbcegjk"));
        //spell-checker: enable
    }

    #[test]
    fn test_find_next_password() {
        //spell-checker: disable
        assert_eq!(find_next_password("abcdefgh"), "abcdffaa");
        assert_eq!(find_next_password("ghijklmn"), "ghjaabcc");
        //spell-checker: enable
    }
}
