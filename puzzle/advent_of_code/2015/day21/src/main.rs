use std::collections::HashSet;

use input_reader::InputReader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ItemKind {
    Weapon,
    Armour,
    Ring,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Item<'a> {
    kind: ItemKind,
    name: &'a str,
    cost: i32,
    damage: i32,
    armour: i32,
}

impl<'a> Item<'a> {
    const fn new_weapon(name: &'a str, cost: i32, damage: i32) -> Self {
        Self {
            kind: ItemKind::Weapon,
            name,
            cost,
            damage,
            armour: 0,
        }
    }

    const fn new_armour(name: &'a str, cost: i32, armour: i32) -> Self {
        Self {
            kind: ItemKind::Armour,
            name,
            cost,
            damage: 0,
            armour,
        }
    }

    const fn new_ring(name: &'a str, cost: i32, damage: i32, armour: i32) -> Self {
        Self {
            kind: ItemKind::Ring,
            name,
            cost,
            damage,
            armour,
        }
    }

    fn kind(&self) -> ItemKind {
        self.kind
    }

    #[allow(dead_code)]
    fn name(&self) -> &'a str {
        self.name
    }

    fn cost(&self) -> i32 {
        self.cost
    }

    fn damage(&self) -> i32 {
        self.damage
    }

    fn armour(&self) -> i32 {
        self.armour
    }
}

const SHOP_ITEMS: [Item<'_>; 16] = [
    Item::new_weapon("Dagger", 8, 4),
    Item::new_weapon("Shortsword", 10, 5),
    Item::new_weapon("Warhammer", 25, 6),
    Item::new_weapon("Longsword", 40, 7),
    Item::new_weapon("Greataxe", 74, 8),
    Item::new_armour("Leather", 13, 1),
    Item::new_armour("Chainmail", 31, 2),
    Item::new_armour("Splintmail", 53, 3),
    Item::new_armour("Bandedmail", 75, 4),
    Item::new_armour("Platemail", 102, 5),
    Item::new_ring("Damage +1", 25, 1, 0),
    Item::new_ring("Damage +2", 50, 2, 0),
    Item::new_ring("Damage +3", 100, 3, 0),
    Item::new_ring("Defense +1", 20, 0, 1),
    Item::new_ring("Defense +2", 40, 0, 2),
    Item::new_ring("Defense +3", 80, 0, 3),
];

#[derive(Debug)]
struct Shop<'a> {
    items: Vec<Item<'a>>,
}

impl<'a> Shop<'a> {
    fn new() -> Self {
        Self {
            items: Vec::from(SHOP_ITEMS),
        }
    }

    fn weapons(&self) -> Vec<&Item<'a>> {
        self.items
            .iter()
            .filter(|item| item.kind() == ItemKind::Weapon)
            .collect()
    }

    fn armours(&self) -> Vec<&Item<'a>> {
        self.items
            .iter()
            .filter(|item| item.kind() == ItemKind::Armour)
            .collect()
    }

    fn rings(&self) -> Vec<&Item<'a>> {
        self.items
            .iter()
            .filter(|item| item.kind() == ItemKind::Ring)
            .collect()
    }

    #[allow(dead_code)]
    fn buy(&mut self, item: &Item<'a>) -> Result<(), String> {
        if let Some(index) = self.items.iter().position(|i| i == item) {
            self.items.remove(index);
            Ok(())
        } else {
            Err("Item not found in shop".to_string())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CharacterKind {
    Player,
    Boss,
}

#[derive(Debug, Clone, PartialEq)]
struct Character<'a> {
    kind: CharacterKind,
    hit_points: i32,
    damage: i32,
    armour: i32,
    item_weapon: Option<Item<'a>>,
    item_armour: Option<Item<'a>>,
    item_ring1: Option<Item<'a>>,
    item_ring2: Option<Item<'a>>,
}

impl<'a> Character<'a> {
    fn new_player() -> Self {
        Character {
            kind: CharacterKind::Player,
            hit_points: 100,
            damage: 0,
            armour: 0,
            item_weapon: None,
            item_armour: None,
            item_ring1: None,
            item_ring2: None,
        }
    }

    fn new_boss(hit_points: i32, damage: i32, armour: i32) -> Self {
        Self {
            kind: CharacterKind::Boss,
            hit_points,
            damage,
            armour,
            item_weapon: None,
            item_armour: None,
            item_ring1: None,
            item_ring2: None,
        }
    }

    #[allow(dead_code)]
    fn kind(&self) -> &CharacterKind {
        &self.kind
    }

    fn hit_points(&self) -> i32 {
        self.hit_points
    }

    fn damage(&self) -> i32 {
        // Sum item damage + self.damage
        [&self.item_weapon, &self.item_ring1, &self.item_ring2]
            .iter()
            .filter_map(|item| item.as_ref())
            .map(|item| item.damage())
            .sum::<i32>()
            + self.damage
    }

    fn armour(&self) -> i32 {
        // Sum item armour + self.armour
        [&self.item_armour, &self.item_ring1, &self.item_ring2]
            .iter()
            .filter_map(|item| item.as_ref())
            .map(|item| item.armour())
            .sum::<i32>()
            + self.armour
    }

    #[allow(dead_code)]
    fn item_weapon(&self) -> Option<&Item<'_>> {
        self.item_weapon.as_ref()
    }

    fn set_item_weapon(&mut self, item: Option<Item<'a>>) -> Result<(), &str> {
        // Guard against invalid item types.
        if item.is_some() && item.as_ref().unwrap().kind() != ItemKind::Weapon {
            return Err("Invalid item kind");
        }

        self.item_weapon = item;

        Ok(())
    }

    #[allow(dead_code)]
    fn item_armour(&self) -> Option<&Item<'_>> {
        self.item_armour.as_ref()
    }

    fn set_item_armour(&mut self, item: Option<Item<'a>>) -> Result<(), &str> {
        // Guard against invalid item types.
        if item.is_some() && item.as_ref().unwrap().kind() != ItemKind::Armour {
            return Err("Invalid item kind");
        }

        self.item_armour = item;

        Ok(())
    }

    #[allow(dead_code)]
    fn item_ring1(&self) -> Option<&Item<'_>> {
        self.item_ring1.as_ref()
    }

    fn set_item_ring1(&mut self, item: Option<Item<'a>>) -> Result<(), &str> {
        // Guard against invalid item types.
        if item.is_some() && item.as_ref().unwrap().kind() != ItemKind::Ring {
            return Err("Invalid item kind");
        }

        // Guard against duplicate rings
        if item.is_some() && self.item_ring2 == item {
            return Err("Duplicate items are not allowed");
        }

        self.item_ring1 = item;

        Ok(())
    }

    #[allow(dead_code)]
    fn item_ring2(&self) -> Option<&Item<'_>> {
        self.item_ring2.as_ref()
    }

    fn set_item_ring2(&mut self, item: Option<Item<'a>>) -> Result<(), &str> {
        // Guard against invalid item types.
        if item.is_some() && item.as_ref().unwrap().kind() != ItemKind::Ring {
            return Err("Invalid item kind");
        }

        // Guard against duplicate rings
        if item.is_some() && self.item_ring1 == item {
            return Err("Duplicate items are not allowed");
        }

        self.item_ring2 = item;

        Ok(())
    }

    fn take_attack(&mut self, from: &Character<'_>) {
        self.hit_points -= (from.damage() - self.armour()).max(1);
    }
}

fn read_input<'a>() -> Character<'a> {
    let mut hit_points = 0;
    let mut damage = 0;
    let mut armour = 0;

    InputReader::new()
        .with_path("./input.txt")
        .read_streaming()
        .expect("Failed to read input")
        .for_each(|line| {
            let line = line.expect("Failed to read line");
            match line.split_once(": ") {
                Some(("Hit Points", value)) => {
                    hit_points = value.parse::<i32>().expect("Failed to parse value");
                }
                Some(("Damage", value)) => {
                    damage = value.parse::<i32>().expect("Failed to parse value");
                }
                Some(("Armor", value)) => {
                    armour = value.parse::<i32>().expect("Failed to parse value");
                }
                _ => panic!("Invalid input"),
            }
        });

    Character::new_boss(hit_points, damage, armour)
}

/// Simulate a battle between player and boss, returns true if player wins.
fn simulate_battle<'a>(player: &Character<'a>, boss: &Character<'a>) -> bool {
    let mut player = player.clone();
    let mut boss = boss.clone();

    let mut is_players_turn = true;

    while player.hit_points() > 0 && boss.hit_points() > 0 {
        if is_players_turn {
            boss.take_attack(&player);
        } else {
            player.take_attack(&boss);
        }

        is_players_turn = !is_players_turn;
    }

    player.hit_points() > 0 && boss.hit_points() <= 0
}

fn find_least_amount_of_gold(boss: &Character<'_>) -> (i32, i32) {
    let mut min_cost = i32::MAX;
    let mut max_cost = i32::MIN;
    let shop = Shop::new();

    // Generate item combinations.
    let mut item_combinations = HashSet::new();
    for weapon in shop.weapons() {
        // Add weapon.
        item_combinations.insert((Some(weapon), None, None, None));

        for armour in shop.armours() {
            // Add weapon and armour.
            item_combinations.insert((Some(weapon), Some(armour), None, None));

            for ring1 in shop.rings() {
                // Add weapon, armour and one ring.
                item_combinations.insert((Some(weapon), Some(armour), Some(ring1), None));

                for ring2 in shop.rings() {
                    // Skip duplicate rings
                    if ring1 == ring2 {
                        continue;
                    }

                    // Add weapon, armour and two rings.
                    item_combinations.insert((
                        Some(weapon),
                        Some(armour),
                        Some(ring1),
                        Some(ring2),
                    ));
                }
            }
        }

        for ring1 in shop.rings() {
            // Add weapon and one ring.
            item_combinations.insert((Some(weapon), None, Some(ring1), None));

            for ring2 in shop.rings() {
                // Skip duplicate rings
                if ring1 == ring2 {
                    continue;
                }

                // Add weapon and two rings.
                item_combinations.insert((Some(weapon), None, Some(ring1), Some(ring2)));
            }
        }
    }

    for (weapon, armour, ring1, ring2) in item_combinations.iter() {
        // Create player with item combination.
        let mut player = Character::new_player();
        player.set_item_weapon(weapon.cloned()).unwrap();
        player.set_item_armour(armour.cloned()).unwrap();
        player.set_item_ring1(ring1.cloned()).unwrap();
        player.set_item_ring2(ring2.cloned()).unwrap();

        let cost = [weapon, armour, ring1, ring2]
            .iter()
            .filter_map(|item| item.as_ref())
            .map(|item| item.cost())
            .sum::<i32>();

        // If the player would win with this item combination.
        if simulate_battle(&player, boss) {
            if min_cost > cost {
                min_cost = cost;
            }
        } else if max_cost < cost {
            max_cost = cost;
        }
    }

    (min_cost, max_cost)
}

fn main() {
    let boss = read_input();

    let (least_amount_of_gold, most_amount_of_gold) = find_least_amount_of_gold(&boss);
    println!(
        "The least amount of gold you can spend is {}",
        least_amount_of_gold
    );
    println!(
        "The most amount of gold you can spend is {}",
        most_amount_of_gold
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shop_operations() {
        let mut shop = Shop::new();
        let dagger = shop
            .weapons()
            .iter()
            .cloned()
            .find(|item| item.name() == "Dagger")
            .expect("Dagger should exist in shop")
            .clone();

        // Test buying an item
        assert!(shop.buy(&dagger).is_ok());

        // Test buying same item again should fail
        assert!(shop.buy(&dagger).is_err());
    }

    #[test]
    fn test_character_equipment() {
        let mut player = Character::new_player();
        let shop = Shop::new();

        // Get one of each type of item
        let weapon = shop
            .weapons()
            .first()
            .cloned()
            .expect("Should find a weapon");
        let armour = shop.armours().first().cloned().expect("Should find armour");
        let ring = shop.rings().first().cloned().expect("Should find a ring");

        // Test equipping items
        assert!(player.set_item_weapon(Some(weapon.clone())).is_ok());
        assert!(player.set_item_armour(Some(armour.clone())).is_ok());
        assert!(player.set_item_ring1(Some(ring.clone())).is_ok());

        // Test wrong item types
        assert!(player.set_item_weapon(Some(armour.clone())).is_err());
        assert!(player.set_item_armour(Some(ring.clone())).is_err());
        assert!(player.set_item_weapon(Some(ring.clone())).is_err());

        // Test unequipping and equipping an item
        assert!(player.set_item_weapon(None).is_ok());
        assert!(player.set_item_weapon(Some(weapon.clone())).is_ok());
    }

    #[test]
    fn test_combat_mechanics() {
        let mut player = Character::new_player();
        let mut boss = Character::new_boss(10, 2, 1);

        // Basic combat test
        let initial_boss_hp = boss.hit_points();
        player.take_attack(&boss);
        boss.take_attack(&player);

        assert!(player.hit_points() < 100); // Player should take damage
        assert!(boss.hit_points() < initial_boss_hp); // Boss should take damage
    }

    #[test]
    fn test_duplicate_rings() {
        let mut player = Character::new_player();
        let ring = Item::new_ring("Test Ring", 10, 1, 1);

        assert!(player.set_item_ring1(Some(ring.clone())).is_ok());
        assert!(player.set_item_ring2(Some(ring.clone())).is_err());
    }

    #[test]
    fn test_damage_and_armor_calculations() {
        let mut player = Character::new_player();

        // Add some equipment
        let weapon = Item::new_weapon("Test Weapon", 10, 2);
        let armor = Item::new_armour("Test Armor", 10, 3);
        let ring1 = Item::new_ring("Test Ring 1", 10, 1, 1);
        let ring2 = Item::new_ring("Test Ring 2", 10, 2, 2);

        player.set_item_weapon(Some(weapon)).unwrap();
        player.set_item_armour(Some(armor)).unwrap();
        player.set_item_ring1(Some(ring1)).unwrap();
        player.set_item_ring2(Some(ring2)).unwrap();

        // Test total calculations
        assert_eq!(player.damage(), 5); // Base(0) + Weapon(2) + Ring1(1) + Ring2(2)
        assert_eq!(player.armour(), 6); // Base(0) + Armor(3) + Ring1(1) + Ring2(2)
    }

    #[test]
    fn test_simulate_battle() {
        let mut player = Character {
            kind: CharacterKind::Player,
            hit_points: 8,
            damage: 5,
            armour: 5,
            item_weapon: None,
            item_armour: None,
            item_ring1: None,
            item_ring2: None,
        };
        let mut boss = Character::new_boss(12, 7, 2);

        // Try simulating the full battle.
        let player_wins = simulate_battle(&player, &boss);
        assert!(player_wins);

        // Manually simulate the steps of the battle as well for good measure:
        boss.take_attack(&player);
        assert_eq!(boss.hit_points(), 9);

        player.take_attack(&boss);
        assert_eq!(player.hit_points(), 6);

        boss.take_attack(&player);
        assert_eq!(boss.hit_points(), 6);

        player.take_attack(&boss);
        assert_eq!(player.hit_points(), 4);

        boss.take_attack(&player);
        assert_eq!(boss.hit_points(), 3);

        player.take_attack(&boss);
        assert_eq!(player.hit_points(), 2);

        boss.take_attack(&player);
        assert_eq!(boss.hit_points(), 0);
    }
}
