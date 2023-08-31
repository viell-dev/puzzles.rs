//! Day X: ___________

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (a, b) = get_answers(input);

    println!("a is: {}", a);
    println!("b is: {}", b);
    todo!();
}

/// Get the answers for the puzzle.
fn get_answers(input: &str) -> (usize, usize) {
    println!("{}", input);
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        todo!();
    }

    /// Test the examples given for part 2 on the puzzle page.
    #[test]
    fn test_part2_examples() {
        todo!();
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (0, 0));
        todo!();
    }
}
