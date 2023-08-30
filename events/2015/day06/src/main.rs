//! Day 6: Probably a Fire Hazard

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (lights_on, brightness) = get_answers(input);

    println!("The number of lights that are lit is: {}", lights_on);
    println!("The total brightness is: {}", brightness);
}

fn get_answers(input: &str) -> (usize, usize) {
    // Initialize a vectors to serve as our grids.
    let mut lights_grid = vec![false; 1_000_000];
    let mut brightness_grid = vec![0usize; 1_000_000];

    // Loop through the lines in the input.
    for line in input.lines() {
        // Parse the instruction.
        match parse_instruction(line) {
            // If the instruction is "turn on".
            Instruction::TurnOn(indices) => {
                // For every index.
                for index in indices {
                    // Turn on the light.
                    lights_grid[index] = true;
                    // Increase the brightness by one.
                    brightness_grid[index] += 1;
                }
            }
            // If the instruction is "turn off".
            Instruction::TurnOff(indices) => {
                // For every index.
                for index in indices {
                    // Turn off the light.
                    lights_grid[index] = false;
                    // Decrease the brightness by one to a minimum of zero.
                    brightness_grid[index] = if brightness_grid[index] > 0 {
                        brightness_grid[index] - 1
                    } else {
                        0
                    };
                }
            }
            // If the instruction is "toggle".
            Instruction::Toggle(indices) => {
                // For every index.
                for index in indices {
                    // Toggle the current state of the light.
                    lights_grid[index] = !lights_grid[index];
                    // Increase the brightness by two.
                    brightness_grid[index] += 2;
                }
            }
        }
    }

    // Count the number of lights that are on.
    let lights_on = lights_grid.iter().filter(|b| **b).count();
    // Sum the brightness of the individual lights.
    let brightness = brightness_grid.iter().sum();

    // Return the answers.
    (lights_on, brightness)
}

/// Instructions the input can give.
enum Instruction {
    TurnOn(Vec<usize>),
    TurnOff(Vec<usize>),
    Toggle(Vec<usize>),
}

/// Parse an instruction line.
fn parse_instruction(line: &str) -> Instruction {
    // Extract the relevant parts from the line.
    let [instruction, start, end] =
        if let [end, _, start, instruction] = line.rsplitn(4, ' ').collect::<Vec<_>>().as_slice() {
            [*instruction, *start, *end]
        } else {
            panic!("Invalid input.")
        };

    // Parse start coords.
    let start = if let [Ok(start1), Ok(start2)] = start
        .split(',')
        .map(|s| s.parse::<usize>())
        .collect::<Vec<_>>()
        .as_slice()
    {
        (*start1, *start2)
    } else {
        panic!("Invalid input.")
    };

    // Parse end coords.
    let end = if let [Ok(end1), Ok(end2)] = end
        .split(',')
        .map(|s| s.parse::<usize>())
        .collect::<Vec<_>>()
        .as_slice()
    {
        (*end1, *end2)
    } else {
        panic!("Invalid input.")
    };

    // Convert the coords to a vector of indices.
    let indices = coords_to_indices(start, end);

    // Return the appropriate instruction.
    match instruction {
        "turn on" => Instruction::TurnOn(indices),
        "turn off" => Instruction::TurnOff(indices),
        "toggle" => Instruction::Toggle(indices),
        _ => panic!("Invalid input."),
    }
}

/// Convert the coords to a vector of indices.
fn coords_to_indices(start: (usize, usize), end: (usize, usize)) -> Vec<usize> {
    // Initialize the vector of indices.
    let mut indices = Vec::<usize>::new();

    // Destructure the coords.
    let (a, b) = start;
    let (c, d) = end;

    // Loop through each x and y coord in the area of the coords.
    for x in a..=c {
        for y in b..=d {
            // Add the index to the vector.
            indices.push(x * 1_000 + y); // 1 000 is the number of columns.
        }
    }

    // Return the vector of indices.
    indices
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        assert_eq!(get_answers("turn on 0,0 through 999,999").0, 1_000_000);
        assert_eq!(get_answers("toggle 0,0 through 999,0").0, 1_000);
        assert_eq!(get_answers("turn off 499,499 through 500,500").0, 0);
        assert_eq!(
            get_answers(
                "turn on 0,0 through 999,999\n\
                toggle 0,0 through 999,0\n\
                turn off 499,499 through 500,500"
            )
            .0,
            998_996
        );
    }

    /// Test the examples given for part 2 on the puzzle page.
    #[test]
    fn test_part2_examples() {
        assert_eq!(get_answers("turn on 0,0 through 0,0").1, 1);
        assert_eq!(get_answers("toggle 0,0 through 999,999").1, 2_000_000);
        assert_eq!(
            get_answers(
                "turn on 0,0 through 0,0\n\
                toggle 0,0 through 999,999"
            )
            .1,
            2_000_001
        )
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (377_891, 14_110_788));
    }
}
