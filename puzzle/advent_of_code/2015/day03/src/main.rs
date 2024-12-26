use std::collections::HashSet;

use input_reader::InputReader;

#[derive(Clone, Debug, PartialEq)]
enum Move {
    North,
    South,
    East,
    West,
}

fn parse_input(input: &str) -> impl Iterator<Item = Move> + Clone + '_ {
    input.chars().map(|c| match c {
        '^' => Move::North,
        'v' => Move::South,
        '>' => Move::East,
        '<' => Move::West,
        _ => panic!("invalid input"),
    })
}

fn houses(iter: impl Iterator<Item = Move>) -> usize {
    let mut santa = (0, 0);
    let mut houses = HashSet::from([santa]);

    iter.for_each(|movement| {
        match movement {
            Move::North => santa.0 += 1,
            Move::South => santa.0 -= 1,
            Move::East => santa.1 += 1,
            Move::West => santa.1 -= 1,
        };

        houses.insert(santa);
    });

    houses.len()
}

fn robo_houses(iter: impl Iterator<Item = Move>) -> usize {
    let mut santa = (0, 0);
    let mut robo_santa = (0, 0);
    let mut houses = HashSet::from([santa]);

    iter.enumerate().for_each(|(index, movement)| {
        if index % 2 == 0 {
            match movement {
                Move::North => santa.0 += 1,
                Move::South => santa.0 -= 1,
                Move::East => santa.1 += 1,
                Move::West => santa.1 -= 1,
            };

            houses.insert(santa);
        } else {
            match movement {
                Move::North => robo_santa.0 += 1,
                Move::South => robo_santa.0 -= 1,
                Move::East => robo_santa.1 += 1,
                Move::West => robo_santa.1 -= 1,
            };

            houses.insert(robo_santa);
        }
    });

    houses.len()
}

fn main() {
    let input_reader = InputReader::new().with_path("./input.txt");
    let input = match input_reader.read() {
        Ok(lines) => match lines.first() {
            Some(line) if !line.trim().is_empty() => line.trim().to_owned(),
            _ => panic!("Error reading input: Input was empty"),
        },
        Err(error) => panic!("Error reading input: {:#?}", error),
    };

    let movements = parse_input(&input);
    let houses = houses(movements.clone());
    let robo_houses = robo_houses(movements);

    println!("Year 1 Houses: {}, Year 2 Houses: {}", houses, robo_houses);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(">").collect::<Vec<_>>(), vec![Move::East]);
        assert_eq!(
            parse_input("^v").collect::<Vec<_>>(),
            vec![Move::North, Move::South]
        );
        assert_eq!(
            parse_input("^>v<").collect::<Vec<_>>(),
            vec![Move::North, Move::East, Move::South, Move::West]
        );
        assert_eq!(
            parse_input("^v^v^v^v^v").collect::<Vec<_>>(),
            vec![
                Move::North,
                Move::South,
                Move::North,
                Move::South,
                Move::North,
                Move::South,
                Move::North,
                Move::South,
                Move::North,
                Move::South
            ]
        );
    }

    #[test]
    fn test_houses() {
        assert_eq!(houses(parse_input(">")), 2);
        assert_eq!(houses(parse_input("^v")), 2);
        assert_eq!(houses(parse_input("^>v<")), 4);
        assert_eq!(houses(parse_input("^v^v^v^v^v")), 2);
    }

    #[test]
    fn test_robo_houses() {
        assert_eq!(robo_houses(parse_input(">")), 2);
        assert_eq!(robo_houses(parse_input("^v")), 3);
        assert_eq!(robo_houses(parse_input("^>v<")), 3);
        assert_eq!(robo_houses(parse_input("^v^v^v^v^v")), 11);
    }
}
