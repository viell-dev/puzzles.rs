//! Day 5: Doesn't He Have Intern-Elves For This?

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (nice_strings, new_nice_strings) = get_answers(input);

    println!("The number of nice strings is: {}", nice_strings);
    println!(
        "The number of nice strings, under the new rules, is: {}",
        new_nice_strings
    );
}

fn get_answers(input: &str) -> (usize, usize) {
    /* Fold over the lines of the input, counting the number of nice strings
    and new nice strings. */
    input
        .lines()
        .fold((0, 0), |(nice_strings, new_nice_strings), line| {
            (
                if is_nice_string(line) {
                    nice_strings + 1
                } else {
                    nice_strings
                },
                if is_new_nice_string(line) {
                    new_nice_strings + 1
                } else {
                    new_nice_strings
                },
            )
        })
}

/// Check if a string is nice according to the rules in part 1 of the puzzle.
fn is_nice_string(line: &str) -> bool {
    // Check if the string contains at least three vowels.
    line.chars()
        .filter(|c| c == &'a' || c == &'e' || c == &'i' || c == &'o' || c == &'u')
        .count()
        >= 3
        // Check if the string contains at least one letter that appears twice in a row.
        && line.chars().zip(line.chars().skip(1)).any(|(a, b)| a == b)
        // Check if the string contains any of the forbidden substrings.
        && !line.contains("ab")
        && !line.contains("cd")
        && !line.contains("pq")
        && !line.contains("xy")
}

/// Check if a string is nice according to the rules in part 2 of the puzzle.
fn is_new_nice_string(line: &str) -> bool {
    // Initialize a vector of pairs of characters.
    let mut pairs = Vec::new();

    // Initialize flags for the conditions.
    let mut has_repeating_pair = false;
    let mut has_repeating_letter = false;
    let mut has_overlapping_pair = false;

    // Iterate over the characters in the string.
    for (i, c) in line.chars().enumerate() {
        // Check if we have at least two characters.
        if i > 0 {
            // Get the pair of the previous character and the current character.
            let pair = format!("{}{}", line.chars().nth(i - 1).unwrap(), c);

            // Check if we have at least three characters.
            if i > 1 {
                // Get the pair of the previous two characters.
                let prev_pair = format!(
                    "{}{}",
                    line.chars().nth(i - 2).unwrap(),
                    line.chars().nth(i - 1).unwrap()
                );
                // Check if the current pair overlaps with the previous pair.
                if prev_pair == pair {
                    has_overlapping_pair = true;
                    break;
                }
            }

            // Check if the current pair has already been seen.
            if pairs.contains(&pair) {
                has_repeating_pair = true;
            }

            // Add the current pair to the vector of pairs.
            pairs.push(pair);
        }

        // Check if we have at least three characters.
        if i > 1 {
            /* Check if the current character is the same as the character
            two positions back. */
            if line.chars().nth(i - 2).unwrap() == c {
                has_repeating_letter = true;
            }
        }
    }

    // Check if all the conditions are met.
    has_repeating_pair && has_repeating_letter && !has_overlapping_pair
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        assert_eq!(is_nice_string("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice_string("aaa"), true);
        assert_eq!(is_nice_string("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice_string("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice_string("dvszwmarrgswjxmb"), false);
    }

    /// Test the examples given for part 2 on the puzzle page.
    #[test]
    fn test_part2_examples() {
        assert_eq!(is_new_nice_string("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_new_nice_string("xxyxx"), true);
        assert_eq!(is_new_nice_string("uurcxstgmygtbstg"), false);
        assert_eq!(is_new_nice_string("ieodomkazucvgmuy"), false);
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (258, 53));
    }
}
