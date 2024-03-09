//! Day 2: I Was Told There Would Be No Math

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (total_area, total_ribbon) = get_answers(input);

    println!("Total area: {}", total_area);
    println!("Total ribbon: {}", total_ribbon);
}

/// Get the answers for the puzzle.
fn get_answers(input: &str) -> (usize, usize) {
    // Square feet of wrapping paper
    let mut paper: usize = 0;
    // Feet of ribbon
    let mut ribbon: usize = 0;

    // Loop over each line in the input
    for line in input.lines() {
        // Get the dimensions.
        let (l, w, h) = get_dimensions(line);
        // Calculate the areas of each side. `lw` will be the smallest side.
        let (lw, wh, hl) = (l * w, w * h, h * l);
        // Calculate the total area plus the area of the smallest side.
        let area = 3 * lw + 2 * wh + 2 * hl;
        // Calculate the length of the ribbon and bow.
        let length = 2 * l + 2 * w + l * w * h;

        // Add the areas and lengths to the totals.
        paper += area;
        ribbon += length;
    }

    // Return the answers
    (paper, ribbon)
}

/// Get the dimensions from a line of input sorted from smallest to largest.
fn get_dimensions(line: &str) -> (usize, usize, usize) {
    // Extract the dimensions from the line and parse them into integers
    let mut dimensions = line
        .split("x")
        .map(|s| match s.parse::<usize>() {
            Ok(i) => i,
            // If the parsing failed, panic
            Err(_) => panic!("Invalid input"),
        })
        .collect::<Vec<_>>();

    // Sort the dimensions so that the smallest is first
    dimensions.sort();

    // Destructure the dimensions into `l`, `w`, and `h`
    if let [l, w, h] = dimensions.as_slice() {
        // Return the dimensions
        return (*l, *w, *h);
    }

    // If the destructuring failed, panic
    panic!("Invalid input");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        assert_eq!(get_answers("2x3x4").0, 58);
        assert_eq!(get_answers("1x1x10").0, 43);
    }

    /// Test the examples given for part 2 on the puzzle page.
    #[test]
    fn test_part2_examples() {
        assert_eq!(get_answers("2x3x4").1, 34);
        assert_eq!(get_answers("1x1x10").1, 14);
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        let (total_area, total_ribbon) = get_answers(input);
        assert_eq!((total_area, total_ribbon), (1586300, 3737498));
    }
}
