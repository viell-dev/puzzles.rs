use input_reader::read_input;

#[derive(Clone, Debug, PartialEq)]
enum Movement {
    Up,
    Down,
}

fn parse_input(input: &str) -> impl Iterator<Item = Movement> + Clone + '_ {
    input.chars().map(|c| match c {
        '(' => Movement::Up,
        ')' => Movement::Down,
        _ => panic!("invalid input"),
    })
}

fn final_floor(iter: impl Iterator<Item = Movement>) -> i32 {
    let mut floor = 0;

    for movement in iter {
        match movement {
            Movement::Up => floor += 1,
            Movement::Down => floor -= 1,
        }
    }

    floor
}

fn basement_instruction(iter: impl Iterator<Item = Movement>) -> Option<usize> {
    let mut instruction = None;
    let mut floor = 0;

    for (index, movement) in iter.enumerate() {
        match movement {
            Movement::Up => floor += 1,
            Movement::Down => floor -= 1,
        }

        if floor == -1 {
            instruction = Some(index + 1);
            break;
        }
    }

    instruction
}

fn main() {
    let input = match read_input(Some("./input.txt"), None) {
        Ok(lines) => match lines.first() {
            Some(line) if !line.trim().is_empty() => line.trim().to_owned(),
            _ => panic!("Error reading input: Input was empty"),
        },
        Err(error) => panic!("Error reading input: {:#?}", error),
    };

    let movement = parse_input(&input);
    let final_floor = final_floor(movement.clone());
    let basement_instruction = basement_instruction(movement);

    println!(
        "Final floor: {}, Basement instruction: {}",
        final_floor,
        basement_instruction.map_or("never".to_string(), |n| n.to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("(())").collect::<Vec<_>>(),
            vec![Movement::Up, Movement::Up, Movement::Down, Movement::Down]
        );
        assert_eq!(
            parse_input("()()").collect::<Vec<_>>(),
            vec![Movement::Up, Movement::Down, Movement::Up, Movement::Down]
        );
        assert_eq!(
            parse_input("(((").collect::<Vec<_>>(),
            vec![Movement::Up, Movement::Up, Movement::Up]
        );
        assert_eq!(
            parse_input("(()(()(").collect::<Vec<_>>(),
            vec![
                Movement::Up,
                Movement::Up,
                Movement::Down,
                Movement::Up,
                Movement::Up,
                Movement::Down,
                Movement::Up
            ]
        );
        assert_eq!(
            parse_input("))(((((").collect::<Vec<_>>(),
            vec![
                Movement::Down,
                Movement::Down,
                Movement::Up,
                Movement::Up,
                Movement::Up,
                Movement::Up,
                Movement::Up
            ]
        );
        assert_eq!(
            parse_input("())").collect::<Vec<_>>(),
            vec![Movement::Up, Movement::Down, Movement::Down]
        );
        assert_eq!(
            parse_input("))(").collect::<Vec<_>>(),
            vec![Movement::Down, Movement::Down, Movement::Up]
        );
        assert_eq!(
            parse_input(")))").collect::<Vec<_>>(),
            vec![Movement::Down, Movement::Down, Movement::Down]
        );
        assert_eq!(
            parse_input(")())())").collect::<Vec<_>>(),
            vec![
                Movement::Down,
                Movement::Up,
                Movement::Down,
                Movement::Down,
                Movement::Up,
                Movement::Down,
                Movement::Down
            ]
        );
        assert_eq!(parse_input(")").collect::<Vec<_>>(), vec![Movement::Down,]);
        assert_eq!(
            parse_input("()())").collect::<Vec<_>>(),
            vec![
                Movement::Up,
                Movement::Down,
                Movement::Up,
                Movement::Down,
                Movement::Down
            ]
        );
    }

    #[test]
    fn test_final_floor() {
        assert_eq!(final_floor(parse_input("(())")), 0);
        assert_eq!(final_floor(parse_input("()()")), 0);
        assert_eq!(final_floor(parse_input("(((")), 3);
        assert_eq!(final_floor(parse_input("(()(()(")), 3);
        assert_eq!(final_floor(parse_input("))(((((")), 3);
        assert_eq!(final_floor(parse_input("())")), -1);
        assert_eq!(final_floor(parse_input("))(")), -1);
        assert_eq!(final_floor(parse_input(")))")), -3);
        assert_eq!(final_floor(parse_input(")())())")), -3);
        assert_eq!(final_floor(parse_input(")")), -1);
        assert_eq!(final_floor(parse_input("()())")), -1);
    }

    #[test]
    fn test_basement_instruction() {
        assert_eq!(basement_instruction(parse_input("(())")), None);
        assert_eq!(basement_instruction(parse_input("()()")), None);
        assert_eq!(basement_instruction(parse_input("(((")), None);
        assert_eq!(basement_instruction(parse_input("(()(()(")), None);
        assert_eq!(basement_instruction(parse_input("))(((((")), Some(1));
        assert_eq!(basement_instruction(parse_input("())")), Some(3));
        assert_eq!(basement_instruction(parse_input("))(")), Some(1));
        assert_eq!(basement_instruction(parse_input(")))")), Some(1));
        assert_eq!(basement_instruction(parse_input(")())())")), Some(1));
        assert_eq!(basement_instruction(parse_input(")")), Some(1));
        assert_eq!(basement_instruction(parse_input("()())")), Some(5));
    }
}
