//! Day 1: Not Quite Lisp

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (floor, basement) = get_answers(input);

    println!("The instructions took Santa to floor {}.", floor);
    println!(
        "The character that caused Santa to first enter the basement is at position {}.",
        basement
    );
}

/// Get the answers for the puzzle.
fn get_answers(input: &str) -> (isize, isize) {
    // Current floor.
    let mut floor: isize = 0;
    // Position of first basement entry.
    let mut basement: isize = 0;

    // Iterate over each character in the input.
    for (i, c) in input.chars().enumerate() {
        // Update the floor based on the current character.
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        // If this is the first basement entry, record the position.
        if basement == 0 && floor == -1 {
            basement = i as isize + 1;
        }
    }

    // Return the answers.
    (floor, basement)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        assert_eq!(get_answers("(())").0, 0);
        assert_eq!(get_answers("()()").0, 0);
        assert_eq!(get_answers("(((").0, 3);
        assert_eq!(get_answers("(()(()(").0, 3);
        assert_eq!(get_answers("))(((((").0, 3);
        assert_eq!(get_answers("())").0, -1);
        assert_eq!(get_answers("))(").0, -1);
        assert_eq!(get_answers(")))").0, -3);
        assert_eq!(get_answers(")())())").0, -3);
    }

    /// Test the examples given for part 2 on the puzzle page.
    #[test]
    fn test_part2_examples() {
        assert_eq!(get_answers(")").1, 1);
        assert_eq!(get_answers("()())").1, 5);
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (232, 1783));
    }
}
