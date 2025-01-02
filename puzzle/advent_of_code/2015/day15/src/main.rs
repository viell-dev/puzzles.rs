use std::collections::HashMap;

use input_reader::InputReader;

const TABLESPOONS: usize = 100;
const MAX_CALORIES: i32 = 500;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

type Recipe = HashMap<Ingredient, u32>;

fn parse_input(input: &[String]) -> impl Iterator<Item = Ingredient> + Clone + '_ {
    input.iter().map(|line| {
        let parts: Vec<&str> = line.split(": ").collect();
        let name = parts[0].to_string();

        let properties: Vec<i32> = parts[1]
            .split(", ")
            .map(|prop| {
                let value_part = prop.split(' ').nth(1).expect("part not found");
                value_part.parse().expect("not a number")
            })
            .collect();

        Ingredient {
            name,
            capacity: properties[0],
            durability: properties[1],
            flavor: properties[2],
            texture: properties[3],
            calories: properties[4],
        }
    })
}

fn sum_recipe(recipe: &Recipe) -> i32 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;

    for (ingredient, amount) in recipe {
        capacity += ingredient.capacity * (*amount as i32);
        durability += ingredient.durability * (*amount as i32);
        flavor += ingredient.flavor * (*amount as i32);
        texture += ingredient.texture * (*amount as i32);
    }

    if capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0 {
        0
    } else {
        capacity * durability * flavor * texture
    }
}

fn find_best_recipe(ingredients: impl Iterator<Item = Ingredient>, limit_calories: bool) -> Recipe {
    use itertools::Itertools;

    let ingredients: Vec<Ingredient> = ingredients.collect();
    let mut best_recipe = HashMap::new();
    let mut best_score = 0;

    // Generate all combinations of ingredient indices that sum to 100
    for combination in (0..ingredients.len()).combinations_with_replacement(TABLESPOONS) {
        let mut recipe = HashMap::new();

        // Count occurrences of each ingredient
        for ingredient_idx in combination {
            *recipe
                .entry(ingredients[ingredient_idx].clone())
                .or_insert(0) += 1;
        }

        // Skip if calories limit is enabled and recipe doesn't have exactly MAX_CALORIES
        if limit_calories {
            let total_calories: i32 = recipe
                .iter()
                .map(|(ingredient, amount)| ingredient.calories * (*amount as i32))
                .sum();
            if total_calories != MAX_CALORIES {
                continue;
            }
        }

        let score = sum_recipe(&recipe);

        if score > best_score {
            best_score = score;
            best_recipe = recipe;
        }
    }

    best_recipe
}

fn main() {
    let input_reader = InputReader::new().with_path("./input.txt");
    let input = match input_reader.read() {
        Ok(lines) => lines
            .iter()
            .filter_map(|line| match line.trim() {
                line if !line.is_empty() => Some(line.to_owned()),
                _ => None,
            })
            .collect::<Vec<_>>(),
        Err(error) => panic!("Error reading input: {:#?}", error),
    };

    let ingredients = parse_input(&input);

    let best_recipe = find_best_recipe(ingredients.clone(), false);
    let score = sum_recipe(&best_recipe);

    println!("Score of best recipe: {}", score);

    let best_recipe_with_max_calories = find_best_recipe(ingredients, true);
    let score_with_max_calories = sum_recipe(&best_recipe_with_max_calories);

    println!(
        "Score of best recipe with max calories: {}",
        score_with_max_calories
    );
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = vec![
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8".to_string(),
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3".to_string(),
        ];

        let ingredients: Vec<Ingredient> = parse_input(&input).collect();

        assert_eq!(ingredients.len(), 2);

        assert_eq!(ingredients[0].name, "Butterscotch");
        assert_eq!(ingredients[0].capacity, -1);
        assert_eq!(ingredients[0].durability, -2);
        assert_eq!(ingredients[0].flavor, 6);
        assert_eq!(ingredients[0].texture, 3);
        assert_eq!(ingredients[0].calories, 8);

        assert_eq!(ingredients[1].name, "Cinnamon");
        assert_eq!(ingredients[1].capacity, 2);
        assert_eq!(ingredients[1].durability, 3);
        assert_eq!(ingredients[1].flavor, -2);
        assert_eq!(ingredients[1].texture, -1);
        assert_eq!(ingredients[1].calories, 3);
    }

    #[test]
    fn test_sum_recipe() {
        let input = vec![
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8".to_string(),
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3".to_string(),
        ];

        let ingredients: Vec<Ingredient> = parse_input(&input).collect();
        let mut recipe = HashMap::new();
        recipe.insert(ingredients[0].clone(), 44);
        recipe.insert(ingredients[1].clone(), 56);

        let sum = sum_recipe(&recipe);

        assert_eq!(sum, 62842880);
    }
    #[test]
    fn test_find_best_recipe() {
        let input = vec![
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8".to_owned(),
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3".to_owned(),
        ];

        let ingredients = parse_input(&input);
        let best_recipe = find_best_recipe(ingredients, false);

        // The optimal recipe should use all 100 teaspoons
        let total_amount: u32 = best_recipe.values().sum();
        assert_eq!(total_amount, 100);

        // The optimal score for this input should be 62842880
        let total_score = sum_recipe(&best_recipe);
        assert_eq!(total_score, 62842880);
    }

    #[test]
    fn test_find_best_recipe_with_max_calories() {
        let input = vec![
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8".to_owned(),
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3".to_owned(),
        ];

        let ingredients = parse_input(&input);
        let best_recipe = find_best_recipe(ingredients, true);

        // The optimal recipe should use all 100 teaspoons
        let total_amount: u32 = best_recipe.values().sum();
        assert_eq!(total_amount, 100);

        // The optimal score for this input with 500 calorie limit should be 57600000
        let total_score = sum_recipe(&best_recipe);
        assert_eq!(total_score, 57600000);
    }
}
