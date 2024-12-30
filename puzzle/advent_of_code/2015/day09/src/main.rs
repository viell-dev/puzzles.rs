use input_reader::InputReader;
use itertools::Itertools;
use petgraph::prelude::UnGraphMap;

#[derive(Debug, Clone, PartialEq)]
enum ParseError {
    Distance(String),
}

#[derive(Debug, Clone, PartialEq)]
struct Distance(u32);

impl TryFrom<String> for Distance {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.parse::<u32>() {
            Ok(value) => Ok(Distance(value)),
            Err(_) => Err(ParseError::Distance(value)),
        }
    }
}

fn parse_input(input: &[String]) -> UnGraphMap<&str, Distance> {
    let mut graph = UnGraphMap::new();

    for line in input.iter() {
        if let [from, _, to, _, distance] = *line.split_whitespace().collect::<Vec<_>>().as_slice()
        {
            let Ok(distance) = Distance::try_from(distance.to_owned()) else {
                panic!("invalid input")
            };

            graph.add_edge(from, to, distance);
        }
    }

    graph
}

fn get_shortest_distance(graph: &UnGraphMap<&str, Distance>) -> u32 {
    let nodes = graph.nodes().collect::<Vec<_>>();

    let mut shortest_distance = u32::MAX;

    for perm in nodes.iter().permutations(nodes.len()) {
        let mut current_distance = 0;

        for window in perm.windows(2) {
            let current_node = window[0];
            let next_node = window[1];

            let edge_weight = graph
                .edge_weight(current_node, next_node)
                .expect("always set");

            current_distance += edge_weight.0;
        }

        if current_distance < shortest_distance {
            shortest_distance = current_distance;
        }
    }

    shortest_distance
}

fn get_longest_distance(graph: &UnGraphMap<&str, Distance>) -> u32 {
    let nodes = graph.nodes().collect::<Vec<_>>();

    let mut longest_distance = 0;

    for perm in nodes.iter().permutations(nodes.len()) {
        let mut current_distance = 0;

        for window in perm.windows(2) {
            let current_node = window[0];
            let next_node = window[1];

            let edge_weight = graph
                .edge_weight(current_node, next_node)
                .expect("always set");

            current_distance += edge_weight.0;
        }

        if current_distance > longest_distance {
            longest_distance = current_distance;
        }
    }

    longest_distance
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

    let graph = parse_input(&input);
    let shortest_distance = get_shortest_distance(&graph);
    let longest_distance = get_longest_distance(&graph);

    println!(
        "Shortest: {}, Longest: {}",
        shortest_distance, longest_distance
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_try_from_string() {
        assert_eq!(Distance::try_from("123".to_owned()), Ok(Distance(123)));
        assert_eq!(
            Distance::try_from("INVALID".to_owned()),
            Err(ParseError::Distance("INVALID".to_owned()))
        );
    }

    // Helper
    fn get_test_input() -> Vec<String> {
        vec![
            "London to Dublin = 464".to_owned(),
            "London to Belfast = 518".to_owned(),
            "Dublin to Belfast = 141".to_owned(),
        ]
    }

    #[test]
    fn test_parse_input() {
        let input = get_test_input();
        let parsed_graph = parse_input(&input);
        let parsed_edges = parsed_graph.all_edges().collect::<Vec<_>>();

        let target_graph = UnGraphMap::from_edges([
            ("London", "Dublin", Distance(464)),
            ("London", "Belfast", Distance(518)),
            ("Dublin", "Belfast", Distance(141)),
        ]);
        let target_edges = target_graph.all_edges().collect::<Vec<_>>();

        assert_eq!(parsed_edges.len(), target_edges.len());

        for edge in parsed_edges {
            assert!(target_edges.contains(&edge));
        }
    }

    #[test]
    fn test_get_shortest_distance() {
        let input = get_test_input();
        let parsed_graph = parse_input(&input);

        assert_eq!(get_shortest_distance(&parsed_graph), 605);
    }

    #[test]
    fn test_get_longest_distance() {
        let input = get_test_input();
        let parsed_graph = parse_input(&input);

        assert_eq!(get_longest_distance(&parsed_graph), 982);
    }
}
