//! Day 16: Aunt Sue

use core::panic;

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (sue_number, real_sue_number) = get_answers(input);

    println!(
        "The number of the Sue that got you the gift is: {}",
        sue_number
    );
    println!("The number of the real Aunt Sue is: {}", real_sue_number);
}

/// The target Sue.
const TARGET_SUE: Sue = Sue {
    number: 0,
    children: Some(3),
    cats: Some(7),
    samoyeds: Some(2),
    pomeranians: Some(3),
    akitas: Some(0),
    vizslas: Some(0),
    goldfish: Some(5),
    trees: Some(3),
    cars: Some(2),
    perfumes: Some(1),
};

/// Get the answers for the puzzle.
fn get_answers(input: &str) -> (usize, usize) {
    // Parse the input into a vector of Sues.
    let sues = input.lines().map(Sue::new).collect::<Vec<_>>();

    // Initialize the Sue number.
    let mut sue_number = 0;
    // Initialize the real Sue number.
    let mut real_sue_number = 0;

    // Iterate over the Sues.
    for sue in sues.iter() {
        // Check if the Sue matches the target.
        let number_matches = [
            sue.children.is_some() && sue.children == TARGET_SUE.children,
            sue.cats.is_some() && sue.cats == TARGET_SUE.cats,
            sue.samoyeds.is_some() && sue.samoyeds == TARGET_SUE.samoyeds,
            sue.pomeranians.is_some() && sue.pomeranians == TARGET_SUE.pomeranians,
            sue.akitas.is_some() && sue.akitas == TARGET_SUE.akitas,
            sue.vizslas.is_some() && sue.vizslas == TARGET_SUE.vizslas,
            sue.goldfish.is_some() && sue.goldfish == TARGET_SUE.goldfish,
            sue.trees.is_some() && sue.trees == TARGET_SUE.trees,
            sue.cars.is_some() && sue.cars == TARGET_SUE.cars,
            sue.perfumes.is_some() && sue.perfumes == TARGET_SUE.perfumes,
        ];

        // If the Sue matches the target, set the Sue number.
        if number_matches.iter().filter(|&m| *m).count() == 3 {
            sue_number = sue.number;
        }

        // Check if the Sue matches the target with the real number of things.
        let real_number_matches = [
            sue.children.is_some() && sue.children == TARGET_SUE.children,
            sue.cats.is_some() && sue.cats > TARGET_SUE.cats,
            sue.samoyeds.is_some() && sue.samoyeds == TARGET_SUE.samoyeds,
            sue.pomeranians.is_some() && sue.pomeranians < TARGET_SUE.pomeranians,
            sue.akitas.is_some() && sue.akitas == TARGET_SUE.akitas,
            sue.vizslas.is_some() && sue.vizslas == TARGET_SUE.vizslas,
            sue.goldfish.is_some() && sue.goldfish < TARGET_SUE.goldfish,
            sue.trees.is_some() && sue.trees > TARGET_SUE.trees,
            sue.cars.is_some() && sue.cars == TARGET_SUE.cars,
            sue.perfumes.is_some() && sue.perfumes == TARGET_SUE.perfumes,
        ];

        // If the Sue matches the target, set the real Sue number.
        if real_number_matches.iter().filter(|&m| *m).count() == 3 {
            real_sue_number = sue.number;
        }
    }

    // Return the answers.
    (sue_number, real_sue_number)
}

/// A Sue.
#[derive(Default)]
struct Sue {
    number: usize,
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

impl Sue {
    /// Create a new Sue from a line.
    fn new(line: &str) -> Self {
        if let [number, property_1, value_1, property_2, value_2, property_3, value_3] = line
            .split(": ")
            .map(|s| s.split(", "))
            .flatten()
            .map(|s| s.split(' ').last().unwrap())
            .collect::<Vec<_>>()
            .as_slice()
        {
            // Create a new blank Sue.
            let mut sue = Self::default();

            // Set the Sue's properties.
            sue.number = number.parse().unwrap();
            sue.set(property_1, value_1);
            sue.set(property_2, value_2);
            sue.set(property_3, value_3);

            // Return the Sue.
            sue
        } else {
            panic!("Invalid line: {}", line);
        }
    }

    /// Set a property of the Sue.
    fn set(&mut self, property: &str, value: &str) {
        let value = Some(value.parse::<usize>().unwrap());

        match property {
            "children" => self.children = value,
            "cats" => self.cats = value,
            "samoyeds" => self.samoyeds = value,
            "pomeranians" => self.pomeranians = value,
            "akitas" => self.akitas = value,
            "vizslas" => self.vizslas = value,
            "goldfish" => self.goldfish = value,
            "trees" => self.trees = value,
            "cars" => self.cars = value,
            "perfumes" => self.perfumes = value,
            _ => panic!("Unknown property: {}", property),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (373, 260));
    }
}
