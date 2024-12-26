use input_reader::read_input;

#[derive(Clone, Debug, PartialEq)]
struct Present {
    length: u32,
    width: u32,
    height: u32,
}

fn parse_input(input: Vec<String>) -> impl Iterator<Item = Present> + Clone {
    input.into_iter().map(|line| {
        if let [Ok(length), Ok(width), Ok(height)] =
            line.split("x").map(str::parse::<u32>).collect::<Vec<_>>()[..]
        {
            Present {
                length,
                width,
                height,
            }
        } else {
            panic!("invalid input")
        }
    })
}

fn wrapping_paper_area(iter: impl Iterator<Item = Present>) -> u32 {
    iter.map(
        |Present {
             length,
             width,
             height,
         }| {
            let a = length * width;
            let b = width * height;
            let c = height * length;

            // Double the sides plus the smallest side
            2 * (a + b + c) + a.min(b.min(c))
        },
    )
    .sum()
}

fn ribbon_length(iter: impl Iterator<Item = Present>) -> u32 {
    iter.map(
        |Present {
             length,
             width,
             height,
         }| {
            let mut dimensions = [length, width, height];
            dimensions.sort();
            let [a, b, c] = dimensions;

            // Double the two smallest dimensions and the product of the dimensions.
            2 * (a + b) + a * b * c
        },
    )
    .sum()
}

fn main() {
    let input = match read_input(Some("./input.txt"), None) {
        Ok(lines) => lines
            .iter()
            .filter_map(|line| match line.trim() {
                line if !line.is_empty() => Some(line.to_owned()),
                _ => None,
            })
            .collect::<Vec<_>>(),
        Err(error) => panic!("Error reading input: {:#?}", error),
    };

    let presents = parse_input(input);
    let wrapping_paper_area = wrapping_paper_area(presents.clone());
    let ribbon_length = ribbon_length(presents);

    println!(
        "Total wrapping paper area: {}, Total ribbon length: {}",
        wrapping_paper_area, ribbon_length
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(vec!["2x3x4".to_owned(), "1x1x10".to_owned()]).collect::<Vec<_>>(),
            vec![
                Present {
                    length: 2,
                    width: 3,
                    height: 4
                },
                Present {
                    length: 1,
                    width: 1,
                    height: 10
                }
            ]
        );
    }

    #[test]
    fn test_wrapping_paper_area() {
        assert_eq!(
            wrapping_paper_area(parse_input(vec!["2x3x4".to_owned()])),
            58
        );
        assert_eq!(
            wrapping_paper_area(parse_input(vec!["1x1x10".to_owned()])),
            43
        );
    }

    #[test]
    fn test_ribbon_length() {
        assert_eq!(ribbon_length(parse_input(vec!["2x3x4".to_owned()])), 34);
        assert_eq!(ribbon_length(parse_input(vec!["1x1x10".to_owned()])), 14);
    }
}
