use input_reader::InputReader;

fn look_and_say<S: AsRef<str>>(value: S) -> String {
    let mut result = String::new();
    let mut chars = value.as_ref().chars().peekable();

    while let Some(char) = chars.next() {
        let mut count = 1;

        while let Some(next_char) = chars.peek() {
            if next_char == &char {
                chars.next(); // consume next char.
                count += 1;
            } else {
                break;
            }
        }

        result.push_str(format!("{}{}", count, char).as_str());
    }

    result
}

fn apply_n_times<F, T>(mut value: T, func: F, n: usize) -> T
where
    F: Fn(T) -> T,
{
    for _ in 0..n {
        value = func(value);
    }
    value
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

    let forty_times = apply_n_times(input.clone(), look_and_say, 40);
    let fifty_times = apply_n_times(input, look_and_say, 50);

    println!(
        "40 times: {}, 50 times: {}",
        forty_times.len(),
        fifty_times.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say("1"), "11".to_owned());
        assert_eq!(look_and_say("11"), "21".to_owned());
        assert_eq!(look_and_say("21"), "1211".to_owned());
        assert_eq!(look_and_say("1211"), "111221".to_owned());
        assert_eq!(look_and_say("111221"), "312211".to_owned());
    }

    #[test]
    fn test_apply_n_times() {
        assert_eq!(apply_n_times(1, |n| n + n, 10), 1024);
    }
}
