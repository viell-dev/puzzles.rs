use input_reader::InputReader;
use petgraph::prelude::DiGraphMap;

type Edge<'a> = (&'a str, &'a str, i32);

fn parse_input(input: &[String]) -> DiGraphMap<&str, i32> {
    let mut graph = DiGraphMap::new();

    for line in input.iter() {
        let [person1, _, change, happiness, _, _, _, _, _, _, person2] =
            *line.split_whitespace().collect::<Vec<_>>().as_slice()
        else {
            panic!("Invalid input");
        };

        let happiness = happiness.parse::<i32>().unwrap();

        graph.add_edge(
            person1,
            &person2[0..person2.len() - 1],
            if change == "gain" {
                happiness
            } else {
                -happiness
            },
        );
    }

    graph
}

fn get_reverse_edge<'a>(
    graph: &DiGraphMap<&'a str, i32>,
    edge: Edge<'a>,
) -> Result<Edge<'a>, &'static str> {
    let (from, to, _) = edge;
    graph
        .edges(to)
        .find(|(_, b, _)| *b == from)
        .map(|(a, b, w)| (a, b, *w))
        .ok_or("reverse edge not found")
}

fn get_max_edge_both_ways<'a>(
    graph: &DiGraphMap<&'a str, i32>,
    node: &'a str,
) -> Result<(Edge<'a>, Edge<'a>), &'static str> {
    let mut max_edge: Option<Edge<'_>> = None;
    let mut max_reverse: Option<Edge<'_>> = None;
    let mut max_combined = i32::MIN;

    for edge in graph.edges(node) {
        let edge = (edge.0, edge.1, *edge.2);
        let reverse = get_reverse_edge(graph, edge)?;
        let combined = edge.2 + reverse.2;

        if combined > max_combined {
            max_combined = combined;
            max_edge = Some(edge);
            max_reverse = Some(reverse);
        }
    }

    max_edge.zip(max_reverse).ok_or("no edges found for node")
}

fn find_optimal_seating_arrangement<'a>(
    graph: &DiGraphMap<&'a str, i32>,
) -> DiGraphMap<&'a str, i32> {
    let mut optimal_graph = DiGraphMap::new();

    for node in graph.nodes() {
        let mut max_graph = DiGraphMap::new();
        let mut start_node = node;
        let mut graph_clone = graph.clone();

        while graph_clone.node_count() > 1 {
            let (max_edge, reverse_edge) = get_max_edge_both_ways(&graph_clone, start_node)
                .expect("max and/or reverse not found");

            max_graph.add_edge(max_edge.0, max_edge.1, max_edge.2);
            max_graph.add_edge(reverse_edge.0, reverse_edge.1, reverse_edge.2);

            graph_clone.remove_node(start_node);

            start_node = max_edge.1;
        }

        let final_edge = graph
            .edges(node)
            .find(|(_, other_node, _)| *other_node == start_node)
            .expect("invalid non-cyclical graph");
        let final_reverse_edge = graph
            .edges(start_node)
            .find(|(_, other_node, _)| *other_node == node)
            .expect("invalid non-cyclical graph");

        max_graph.add_edge(final_edge.0, final_edge.1, *final_edge.2);
        max_graph.add_edge(
            final_reverse_edge.0,
            final_reverse_edge.1,
            *final_reverse_edge.2,
        );

        if sum_happiness(&max_graph) > sum_happiness(&optimal_graph) {
            optimal_graph = max_graph;
        }
    }

    optimal_graph
}

fn sum_happiness(graph: &DiGraphMap<&str, i32>) -> i32 {
    graph.all_edges().map(|(_, _, weight)| weight).sum()
}

fn add_me_to_graph<'a>(graph: &DiGraphMap<&'a str, i32>) -> DiGraphMap<&'a str, i32> {
    let mut new_graph = graph.clone();

    // Add edges between "Me" and all existing nodes with weight 0 in both directions
    for node in graph.nodes() {
        new_graph.add_edge("Me", node, 0);
        new_graph.add_edge(node, "Me", 0);
    }

    new_graph
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
    let optimal_graph = find_optimal_seating_arrangement(&graph);
    let max_happiness = sum_happiness(&optimal_graph);

    let graph_with_me = add_me_to_graph(&graph);
    let optimal_graph_with_me = find_optimal_seating_arrangement(&graph_with_me);
    let max_happiness_with_me = sum_happiness(&optimal_graph_with_me);

    println!("Max Happiness without me: {}", max_happiness);
    println!("Max Happiness with me: {}", max_happiness_with_me);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "Alice would gain 54 happiness units by sitting next to Bob.".to_owned(),
            "Alice would lose 79 happiness units by sitting next to Carol.".to_owned(),
            "Alice would lose 2 happiness units by sitting next to David.".to_owned(),
            "Bob would gain 83 happiness units by sitting next to Alice.".to_owned(),
            "Bob would lose 7 happiness units by sitting next to Carol.".to_owned(),
            "Bob would lose 63 happiness units by sitting next to David.".to_owned(),
            "Carol would lose 62 happiness units by sitting next to Alice.".to_owned(),
            "Carol would gain 60 happiness units by sitting next to Bob.".to_owned(),
            "Carol would gain 55 happiness units by sitting next to David.".to_owned(),
            "David would gain 46 happiness units by sitting next to Alice.".to_owned(),
            "David would lose 7 happiness units by sitting next to Bob.".to_owned(),
            "David would gain 41 happiness units by sitting next to Carol.".to_owned(),
        ]
    }

    fn get_test_input_as_edges<'a>() -> Vec<(&'a str, &'a str, &'a i32)> {
        vec![
            ("Alice", "Bob", &54),
            ("Alice", "Carol", &-79),
            ("Alice", "David", &-2),
            ("Bob", "Alice", &83),
            ("Bob", "Carol", &-7),
            ("Bob", "David", &-63),
            ("Carol", "Alice", &-62),
            ("Carol", "Bob", &60),
            ("Carol", "David", &55),
            ("David", "Alice", &46),
            ("David", "Bob", &-7),
            ("David", "Carol", &41),
        ]
    }

    fn get_test_input_optimal_edges<'a>() -> Vec<(&'a str, &'a str, &'a i32)> {
        vec![
            ("Alice", "Bob", &54),
            ("Alice", "David", &-2),
            ("Bob", "Alice", &83),
            ("Bob", "Carol", &-7),
            ("Carol", "Bob", &60),
            ("Carol", "David", &55),
            ("David", "Alice", &46),
            ("David", "Carol", &41),
        ]
    }

    #[test]
    fn test_parse_input() {
        let input = get_test_input();
        let graph = parse_input(&input);

        assert_eq!(graph.node_count(), 4);

        let target_edges = get_test_input_as_edges();

        assert_eq!(graph.edge_count(), target_edges.len());

        for edge in graph.all_edges() {
            assert!(target_edges.contains(&edge));
        }
    }

    #[test]
    fn test_get_reverse_edge() {
        let input = get_test_input();
        let graph = parse_input(&input);

        let edge = ("Alice", "Bob", 54);
        let reverse = get_reverse_edge(&graph, edge).unwrap();
        assert_eq!(reverse, ("Bob", "Alice", 83));

        let edge = ("Bob", "Carol", -7);
        let reverse = get_reverse_edge(&graph, edge).unwrap();
        assert_eq!(reverse, ("Carol", "Bob", 60));

        let edge = ("Carol", "David", 55);
        let reverse = get_reverse_edge(&graph, edge).unwrap();
        assert_eq!(reverse, ("David", "Carol", 41));

        let edge = ("David", "Alice", 46);
        let reverse = get_reverse_edge(&graph, edge).unwrap();
        assert_eq!(reverse, ("Alice", "David", -2));
    }

    #[test]
    fn test_get_max_edge_both_ways() {
        let input = get_test_input();
        let graph = parse_input(&input);

        // Test Alice
        let (max_edge, reverse_edge) = get_max_edge_both_ways(&graph, "Alice").unwrap();
        // Bob<->Alice has highest combined happiness (54 + 83 = 137)
        assert_eq!(max_edge, ("Alice", "Bob", 54));
        assert_eq!(reverse_edge, ("Bob", "Alice", 83));

        // Test Bob
        let (max_edge, reverse_edge) = get_max_edge_both_ways(&graph, "Bob").unwrap();
        // Alice<->Bob has highest combined happiness (83 + 54 = 137)
        assert_eq!(max_edge, ("Bob", "Alice", 83));
        assert_eq!(reverse_edge, ("Alice", "Bob", 54));

        // Test Carol
        let (max_edge, reverse_edge) = get_max_edge_both_ways(&graph, "Carol").unwrap();
        // David<->Carol has highest combined happiness (55 + 41 = 96)
        assert_eq!(max_edge, ("Carol", "David", 55));
        assert_eq!(reverse_edge, ("David", "Carol", 41));

        // Test David
        let (max_edge, reverse_edge) = get_max_edge_both_ways(&graph, "David").unwrap();
        // Carol<->David has highest combined happiness (41 + 55 = 96)
        assert_eq!(max_edge, ("David", "Carol", 41));
        assert_eq!(reverse_edge, ("Carol", "David", 55));
    }

    #[test]
    fn test_find_optimal_seating_arrangement() {
        let input = get_test_input();
        let graph = parse_input(&input);
        let optimal_graph = find_optimal_seating_arrangement(&graph);

        let target_edges = get_test_input_optimal_edges();

        assert_eq!(optimal_graph.edge_count(), target_edges.len());

        for edge in optimal_graph.all_edges() {
            assert!(target_edges.contains(&edge));
        }
    }

    #[test]
    fn test_sum_happiness() {
        let input = get_test_input();
        let graph = parse_input(&input);
        let optimal_graph = find_optimal_seating_arrangement(&graph);

        let optimal_edges = get_test_input_optimal_edges();
        let optimal_edges_sum = optimal_edges
            .iter()
            .map(|(_, _, weight)| **weight)
            .sum::<i32>();

        assert_eq!(optimal_edges_sum, 330);
        assert_eq!(sum_happiness(&optimal_graph), optimal_edges_sum);
    }

    #[test]
    fn test_add_me_to_graph() {
        let input = get_test_input();
        let graph = parse_input(&input);
        let graph_with_me = add_me_to_graph(&graph);

        // Check that original nodes still exist
        for node in graph.nodes() {
            assert!(graph_with_me.contains_node(node));
        }

        // Check that "Me" node was added
        assert!(graph_with_me.contains_node("Me"));

        // Check that edges between "Me" and all other nodes exist with weight 0
        for node in graph.nodes() {
            assert_eq!(graph_with_me.edge_weight("Me", node), Some(&0));
            assert_eq!(graph_with_me.edge_weight(node, "Me"), Some(&0));
        }

        // Check that original edges and weights are preserved
        for (from, to, weight) in graph.all_edges() {
            assert_eq!(graph_with_me.edge_weight(from, to), Some(weight));
        }

        // Check total number of edges
        // Original edges + 2 edges per original node (to/from "Me")
        let expected_edge_count = graph.edge_count() + (2 * graph.node_count());
        assert_eq!(graph_with_me.edge_count(), expected_edge_count);
    }

    #[test]
    fn test_optimal_arrangement_with_me() {
        let input = get_test_input();
        let graph = parse_input(&input);
        let graph_with_me = add_me_to_graph(&graph);
        let optimal_graph = find_optimal_seating_arrangement(&graph_with_me);

        // Verify optimal graph includes me
        assert!(optimal_graph.contains_node("Me"));

        // Verify optimal graph has all original nodes
        for node in graph.nodes() {
            assert!(optimal_graph.contains_node(node));
        }

        // Verify optimal graph is cyclic and connected
        let node_count = optimal_graph.node_count();
        let edge_count = optimal_graph.edge_count();
        assert_eq!(edge_count, node_count * 2); // Each node should have 2 edges (in/out)
    }

    #[test]
    fn test_sum_happiness_with_me() {
        let input = get_test_input();
        let graph = parse_input(&input);
        let graph_with_me = add_me_to_graph(&graph);
        let optimal_graph = find_optimal_seating_arrangement(&graph_with_me);

        let happiness_with_me = sum_happiness(&optimal_graph);
        let happiness_without_me = sum_happiness(&find_optimal_seating_arrangement(&graph));

        // Happiness with me should be less since I add 0 happiness
        assert!(happiness_with_me < happiness_without_me);

        // My edges should contribute 0 to total happiness
        let my_edges_sum: i32 = optimal_graph.edges("Me").map(|(_, _, w)| *w).sum();
        assert_eq!(my_edges_sum, 0);
    }
}
