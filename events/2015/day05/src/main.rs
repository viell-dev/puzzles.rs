fn is_nice_string(input: &str) -> bool {
    input
        .chars()
        .filter(|c| c == &'a' || c == &'e' || c == &'i' || c == &'o' || c == &'u')
        .count()
        >= 3
        && input
            .chars()
            .zip(input.chars().skip(1))
            .any(|(a, b)| a == b)
        && !input.contains("ab")
        && !input.contains("cd")
        && !input.contains("pq")
        && !input.contains("xy")
}

fn is_new_nice_string(input: &str) -> bool {
    let mut pairs = Vec::new();
    let mut has_repeating_pair = false;
    let mut has_repeating_letter = false;
    let mut has_overlapping_pair = false;

    for (i, c) in input.chars().enumerate() {
        if i > 0 {
            let pair = format!("{}{}", input.chars().nth(i - 1).unwrap(), c);

            if i > 1 {
                let prev_pair = format!(
                    "{}{}",
                    input.chars().nth(i - 2).unwrap(),
                    input.chars().nth(i - 1).unwrap()
                );
                if prev_pair == pair {
                    has_overlapping_pair = true;
                    break;
                }
            }

            if pairs.contains(&pair) {
                has_repeating_pair = true;
            }

            pairs.push(pair);
        }
        if i > 1 {
            if input.chars().nth(i - 2).unwrap() == c {
                has_repeating_letter = true;
            }
        }
    }

    has_repeating_pair && has_repeating_letter && !has_overlapping_pair
}

fn main() {
    let input = include_str!("input.txt").trim();
    let nice_strings = input.lines().filter(|s| is_nice_string(s)).count();
    println!("Nice strings: {}", nice_strings);
    let new_nice_strings = input.lines().filter(|s| is_new_nice_string(s)).count();
    println!("New nice strings: {}", new_nice_strings);
}
