//! Day 15: Science for Hungry People

use itertools::Itertools;

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (max_score, max_score_with_limited_calories) = get_answers(input);

    println!(
        "The total score of the highest-scoring cookie you can make is: {}",
        max_score
    );
    println!("The total score of the highest-scoring cookie you can make with a calorie total of {} is: {}", CALORIES, max_score_with_limited_calories);
}

/// The number of tablespoons of ingredients to use.
const TABLESPOONS: usize = 100;

/// The number of calories the cookies must have.
const CALORIES: isize = 500;

/// Get the answers for the puzzle.
fn get_answers(input: &str) -> (isize, isize) {
    // Parse the input into a vector of ingredients.
    let ingredients = input
        .lines()
        .map(|line| Ingredient::new(line))
        .collect::<Vec<_>>();

    // Get the maximum score for the ingredients.
    let max_score = get_max_score(&ingredients, false);

    // Get the maximum score for the ingredients with a calorie limit.
    let max_score_with_limited_calories = get_max_score(&ingredients, true);

    // Return the answers.
    (max_score, max_score_with_limited_calories)
}

/// An ingredient.
#[derive(Clone)]
struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

impl Ingredient {
    /// Create a new ingredient from the given line.
    fn new(line: &str) -> Self {
        if let [capacity, durability, flavor, texture, calories] = line
            .split(", ")
            .map(|s| s.split(" ").last().unwrap())
            .collect::<Vec<_>>()
            .as_slice()
        {
            Self {
                capacity: capacity.parse().unwrap(),
                durability: durability.parse().unwrap(),
                flavor: flavor.parse().unwrap(),
                texture: texture.parse().unwrap(),
                calories: calories.parse().unwrap(),
            }
        } else {
            panic!("Invalid input: {}", line);
        }
    }
}

/// Get the maximum score for the given ingredients.
fn get_max_score(ingredients: &Vec<Ingredient>, limit_calories: bool) -> isize {
    // Initialize the max score to the lowest possible value.
    let mut max_score = isize::MIN;

    // Iterate over all possible combinations of ingredients.
    for set in ingredients
        .iter()
        .combinations_with_replacement(TABLESPOONS)
    {
        if limit_calories {
            // Sum the calories of the ingredients in the set.
            let calories = set.iter().map(|i| i.calories).sum::<isize>();

            // Skip this set if the calories don't match the limit.
            if calories != CALORIES {
                continue;
            }
        }

        // Sum the properties of the ingredients in the set.
        let capacity = set.iter().map(|i| i.capacity).sum::<isize>().max(0);
        let durability = set.iter().map(|i| i.durability).sum::<isize>().max(0);
        let flavor = set.iter().map(|i| i.flavor).sum::<isize>().max(0);
        let texture = set.iter().map(|i| i.texture).sum::<isize>().max(0);

        // Calculate the score.
        let score = capacity * durability * flavor * texture;

        // Update the max score if necessary.
        if score > max_score {
            max_score = score;
        }
    }

    // Panic if no max score was found.
    if max_score == isize::MIN {
        panic!("No max score found!");
    }

    // Return the max score.
    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Get the example input.
    fn get_example_input() -> &'static str {
        r#"
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
"#
        .trim()
    }

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        let input = get_example_input();

        let ingredients = input
            .lines()
            .map(|line| Ingredient::new(line))
            .collect::<Vec<_>>();

        assert_eq!(get_max_score(&ingredients, false), 62_842_880);
    }

    /// Test the examples given for part 2 on the puzzle page.
    #[test]
    fn test_part2_examples() {
        let input = get_example_input();

        let ingredients = input
            .lines()
            .map(|line| Ingredient::new(line))
            .collect::<Vec<_>>();

        assert_eq!(get_max_score(&ingredients, true), 57_600_000);
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (222_870, 117_936));
    }
}
