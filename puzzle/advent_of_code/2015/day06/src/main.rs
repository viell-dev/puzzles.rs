use std::collections::HashSet;

use input_reader::InputReader;

// Only 0 through 999 allowed.
const MAX_COORDINATE_SIZE: usize = 999;

#[derive(Clone, Debug, PartialEq)]
enum InstructionKind {
    TurnOn,
    Toggle,
    TurnOff,
}

#[derive(Clone, Debug, PartialEq)]
struct Instruction {
    kind: InstructionKind,
    indices: HashSet<usize>,
}

#[derive(Debug, PartialEq)]
enum ParseError {
    InvalidKind,
    InvalidCoordinates,
}

fn parse_kind(kind: &str) -> Result<InstructionKind, ParseError> {
    match kind {
        "turn on" => Ok(InstructionKind::TurnOn),
        "toggle" => Ok(InstructionKind::Toggle),
        "turn off" => Ok(InstructionKind::TurnOff),
        _ => Err(ParseError::InvalidKind),
    }
}

fn parse_coordinates(coords: &str) -> Result<(usize, usize), ParseError> {
    if let [Some(x), Some(y)] = coords
        .splitn(2, ',')
        .map(|n| {
            n.parse::<usize>()
                .ok()
                .filter(|n| n <= &MAX_COORDINATE_SIZE)
        })
        .collect::<Vec<_>>()
        .as_slice()
    {
        Ok((*x, *y))
    } else {
        Err(ParseError::InvalidCoordinates)
    }
}

fn convert_coordinates_to_indices(start: (usize, usize), end: (usize, usize)) -> HashSet<usize> {
    let mut indices = HashSet::new();

    for x in start.0..=end.0 {
        for y in start.1..=end.1 {
            indices.insert(x * 1000 + y);
        }
    }

    indices
}

fn parse_input(input: &[String]) -> impl Iterator<Item = Instruction> + Clone + use<'_> {
    // Split input lines into kind, start and end.
    input
        .iter()
        // Split string
        .map(|line| {
            if let [end, _, start, kind] = line.rsplitn(4, ' ').collect::<Vec<_>>().as_slice() {
                (*kind, *start, *end)
            } else {
                panic!("Invalid input")
            }
        })
        .map(|(kind, start, end)| {
            let kind = parse_kind(kind).unwrap();

            let start = parse_coordinates(start).unwrap();
            let end = parse_coordinates(end).unwrap();

            let indices = convert_coordinates_to_indices(start, end);

            Instruction { kind, indices }
        })
}

fn total_lit_lights(instructions: impl Iterator<Item = Instruction>) -> usize {
    let mut light_grid = vec![false; 1000000];

    for instruction in instructions {
        match instruction.kind {
            InstructionKind::TurnOn => {
                #[allow(unsafe_code)]
                instruction.indices.iter().for_each(|index| unsafe {
                    let light = light_grid.get_unchecked_mut(*index);
                    *light = true;
                });
            }
            InstructionKind::Toggle => {
                #[allow(unsafe_code)]
                instruction.indices.iter().for_each(|index| unsafe {
                    let light = light_grid.get_unchecked_mut(*index);
                    *light = !*light;
                });
            }
            InstructionKind::TurnOff => {
                #[allow(unsafe_code)]
                instruction.indices.iter().for_each(|index| unsafe {
                    let light = light_grid.get_unchecked_mut(*index);
                    *light = false;
                });
            }
        }
    }

    light_grid.iter().filter(|b| **b).count()
}

fn total_brightness(instructions: impl Iterator<Item = Instruction>) -> usize {
    let mut light_grid = vec![0; 1000000];

    for instruction in instructions {
        match instruction.kind {
            InstructionKind::TurnOn => {
                #[allow(unsafe_code)]
                instruction.indices.iter().for_each(|index| unsafe {
                    let light = light_grid.get_unchecked_mut(*index);
                    *light += 1;
                });
            }
            InstructionKind::Toggle => {
                #[allow(unsafe_code)]
                instruction.indices.iter().for_each(|index| unsafe {
                    let light = light_grid.get_unchecked_mut(*index);
                    *light += 2;
                });
            }
            InstructionKind::TurnOff => {
                #[allow(unsafe_code)]
                instruction.indices.iter().for_each(|index| unsafe {
                    let light = light_grid.get_unchecked_mut(*index);
                    if light > &mut 0 {
                        *light -= 1;
                    }
                });
            }
        }
    }

    light_grid.iter().sum()
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

    let instructions = parse_input(&input);
    let total_lit_lights = total_lit_lights(instructions.clone());
    let total_brightness = total_brightness(instructions);

    println!(
        "Total light lights: {}, Total brightness: {}",
        total_lit_lights, total_brightness
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_kind() {
        assert_eq!(parse_kind("turn on"), Ok(InstructionKind::TurnOn));
        assert_eq!(parse_kind("toggle"), Ok(InstructionKind::Toggle));
        assert_eq!(parse_kind("turn off"), Ok(InstructionKind::TurnOff));
        assert_eq!(
            // invalid string
            parse_kind("invalid"),
            Err(ParseError::InvalidKind)
        );
    }

    #[test]
    fn test_parse_coordinates() {
        assert_eq!(parse_coordinates("0,0"), Ok((0, 0)));
        assert_eq!(parse_coordinates("999,999"), Ok((999, 999)));
        assert_eq!(parse_coordinates("999,0"), Ok((999, 0)));
        assert_eq!(parse_coordinates("499,499"), Ok((499, 499)));
        assert_eq!(parse_coordinates("500,500"), Ok((500, 500)));
        assert_eq!(
            // coordinate above max
            parse_coordinates("1500,0"),
            Err(ParseError::InvalidCoordinates)
        );
        assert_eq!(
            // invalid string
            parse_coordinates("invalid"),
            Err(ParseError::InvalidCoordinates)
        );
    }

    #[test]
    fn test_convert_coordinates_to_indices() {
        assert_eq!(
            convert_coordinates_to_indices((0, 0), (999, 999)).len(),
            1000000
        );
        assert_eq!(
            convert_coordinates_to_indices((999, 0), (999, 999)).len(),
            1000
        );
        assert_eq!(
            convert_coordinates_to_indices((499, 499), (500, 500)).len(),
            4
        );
        assert_eq!(
            convert_coordinates_to_indices((499, 499), (500, 500)),
            HashSet::from([499499, 499500, 500499, 500500])
        );
    }

    #[test]
    fn test_parse_input() {
        let instructions = parse_input(&[
            "turn on 0,0 through 999,999".to_owned(),
            "toggle 0,0 through 999,0".to_owned(),
            "turn off 499,499 through 500,500".to_owned(),
        ])
        .collect::<Vec<_>>();

        assert_eq!(instructions.len(), 3);
        assert_eq!(instructions[0].kind, InstructionKind::TurnOn);
        assert_eq!(instructions[0].indices.len(), 1000000);
        assert_eq!(instructions[1].kind, InstructionKind::Toggle);
        assert_eq!(instructions[1].indices.len(), 1000);
        assert_eq!(instructions[2].kind, InstructionKind::TurnOff);
        assert_eq!(instructions[2].indices.len(), 4);
        assert_eq!(
            instructions[2].indices,
            HashSet::from([499499, 499500, 500499, 500500])
        );
    }

    #[test]
    fn test_total_lit_lights() {
        let input = vec![
            "turn on 0,0 through 999,999".to_owned(),
            "toggle 0,0 through 999,0".to_owned(),
            "turn off 499,499 through 500,500".to_owned(),
        ];
        let instructions = parse_input(&input);

        assert_eq!(total_lit_lights(instructions), 998996);
    }

    #[test]
    fn test_total_brightness() {
        let input = vec![
            "turn on 0,0 through 999,999".to_owned(),
            "toggle 0,0 through 999,0".to_owned(),
            "turn off 499,499 through 500,500".to_owned(),
        ];
        let instructions = parse_input(&input);

        assert_eq!(total_brightness(instructions), 1001996);
    }
}
