use input_reader::InputReader;

const MFCSAM_READINGS: Aunt = Aunt {
    number: 0, // This field isn't relevant for the readings
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

#[derive(Clone)]
struct Aunt {
    number: u32,
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl std::fmt::Debug for Aunt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("Aunt");
        debug.field("number", &self.number);

        macro_rules! debug_field {
            ($field:ident) => {
                if let Some(v) = self.$field {
                    debug.field(stringify!($field), &v);
                }
            };
        }

        debug_field!(children);
        debug_field!(cats);
        debug_field!(samoyeds);
        debug_field!(pomeranians);
        debug_field!(akitas);
        debug_field!(vizslas);
        debug_field!(goldfish);
        debug_field!(trees);
        debug_field!(cars);
        debug_field!(perfumes);

        debug.finish()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum AuntError {
    Parse(std::num::ParseIntError),
    InvalidKey(String),
}

impl TryFrom<&String> for Aunt {
    type Error = AuntError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let mut aunt = Aunt {
            number: 0,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        };

        let mut parts = value.split([':', ',', ' ']).filter(|s| !s.is_empty());

        while let Some(key) = parts.next() {
            let value = parts
                .next()
                .ok_or_else(|| AuntError::InvalidKey("Missing value".to_string()))?
                .parse::<u32>()
                .map_err(AuntError::Parse)?;

            match key {
                "Sue" => aunt.number = value,
                "children" => aunt.children = Some(value),
                "cats" => aunt.cats = Some(value),
                "samoyeds" => aunt.samoyeds = Some(value),
                "pomeranians" => aunt.pomeranians = Some(value),
                "akitas" => aunt.akitas = Some(value),
                "vizslas" => aunt.vizslas = Some(value),
                "goldfish" => aunt.goldfish = Some(value),
                "trees" => aunt.trees = Some(value),
                "cars" => aunt.cars = Some(value),
                "perfumes" => aunt.perfumes = Some(value),
                _ => return Err(AuntError::InvalidKey(key.to_string())),
            }
        }
        Ok(aunt)
    }
}

fn parse_input(input: &[String]) -> impl Iterator<Item = Aunt> + '_ {
    input.iter().filter_map(|line| line.try_into().ok())
}

macro_rules! check_field {
    ($aunt:expr, $field:ident) => {
        $aunt
            .$field
            .map_or(true, |v| v == MFCSAM_READINGS.$field.unwrap())
    };
}

fn find_aunt(mut aunts: impl Iterator<Item = Aunt>) -> Aunt {
    aunts
        .find(|aunt| {
            check_field!(aunt, children)
                && check_field!(aunt, cats)
                && check_field!(aunt, samoyeds)
                && check_field!(aunt, pomeranians)
                && check_field!(aunt, akitas)
                && check_field!(aunt, vizslas)
                && check_field!(aunt, goldfish)
                && check_field!(aunt, trees)
                && check_field!(aunt, cars)
                && check_field!(aunt, perfumes)
        })
        .expect("No matching aunt found")
}

macro_rules! check_field_real {
    ($aunt:expr, $field:ident, $op:tt) => {
        $aunt.$field.map_or(true, |v| v $op MFCSAM_READINGS.$field.unwrap())
    };
}

fn find_real_aunt(mut aunts: impl Iterator<Item = Aunt>) -> Aunt {
    aunts
        .find(|aunt| {
            check_field!(aunt, children)
                && check_field_real!(aunt, cats, >)
                && check_field!(aunt, samoyeds)
                && check_field_real!(aunt, pomeranians, <)
                && check_field!(aunt, akitas)
                && check_field!(aunt, vizslas)
                && check_field_real!(aunt, goldfish, <)
                && check_field_real!(aunt, trees, >)
                && check_field!(aunt, cars)
                && check_field!(aunt, perfumes)
        })
        .expect("No matching aunt found")
}

fn main() {
    let input_reader = InputReader::new().with_path("./input.txt");
    let input: Vec<String> = input_reader
        .read()
        .expect("Error reading input")
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .collect();

    let aunts = parse_input(&input);
    let aunts: Vec<_> = aunts.collect();

    let fake_sue = find_aunt(aunts.iter().cloned());
    let real_sue = find_real_aunt(aunts.iter().cloned());

    println!(
        "Fake Sue: {}, Real Sue: {}",
        fake_sue.number, real_sue.number
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aunt_parsing() {
        let input = "Sue 1: cars: 9, akitas: 3, goldfish: 0".to_string();
        let aunt: Aunt = (&input).try_into().unwrap();

        assert_eq!(aunt.number, 1);
        assert_eq!(aunt.cars, Some(9));
        assert_eq!(aunt.akitas, Some(3));
        assert_eq!(aunt.goldfish, Some(0));
        assert_eq!(aunt.children, None);
    }

    #[test]
    fn test_find_aunt() {
        let input = vec![
            "Sue 1: children: 2, cats: 7, samoyeds: 2".to_string(),
            "Sue 2: children: 3, cats: 6, samoyeds: 2".to_string(),
            "Sue 3: children: 3, cats: 7, samoyeds: 2".to_string(),
        ];

        let aunts = parse_input(&input);
        let found = find_aunt(aunts);
        assert_eq!(found.number, 3);
    }

    #[test]
    fn test_find_real_aunt() {
        let input = vec![
            "Sue 1: cats: 8, trees: 4, goldfish: 3".to_string(),
            "Sue 2: cats: 6, trees: 2, goldfish: 6".to_string(),
            "Sue 3: cats: 8, trees: 4, goldfish: 4".to_string(),
        ];

        let aunts = parse_input(&input);
        let found = find_real_aunt(aunts);
        assert_eq!(found.number, 1);
    }
}
