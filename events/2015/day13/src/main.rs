//! Day 13: Knights of the Dinner Table

use petgraph::prelude::DiGraphMap;

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (optimal_happiness, optimal_happiness_with_me) = get_answers(input);

    println!("Optimal happiness is: {}", optimal_happiness);
    println!(
        "Optimal happiness with me is: {}",
        optimal_happiness_with_me
    );
}

/// Get the answers for the puzzle.
fn get_answers(input: &str) -> (isize, isize) {
    // Parse the input into a directed graph.
    let graph = parse_input(input);

    // Get the optimal happiness for the graph.
    let optimal_happiness = get_optimal_happiness(&graph);

    // Add me to the graph.
    let graph_with_me = add_me_to_graph(&graph);

    // Get the optimal happiness for the graph with me.
    let optimal_happiness_with_me = get_optimal_happiness(&graph_with_me);

    // Return the answers.
    (optimal_happiness, optimal_happiness_with_me)
}

/// Parse the input into a directed graph.
fn parse_input(input: &str) -> DiGraphMap<&str, isize> {
    // Initialize the graph.
    let mut graph = DiGraphMap::new();

    // Parse each line of the input as a description of an edge.
    for line in input.lines() {
        if let [person1, "would", change, happiness, "happiness", "units", "by", "sitting", "next", "to", person2] =
            line.split_whitespace().collect::<Vec<_>>().as_slice()
        {
            // Parse the happiness value as an isize.
            let happiness = happiness.parse::<isize>().unwrap();

            // Add the edge to the graph.
            graph.add_edge(
                *person1,
                // Remove the trailing period from the person's name.
                &person2[0..person2.len() - 1],
                // Set the sign of the happiness value based on the change.
                if change == &"gain" {
                    happiness
                } else {
                    -happiness
                },
            );
        } else {
            panic!("Invalid input: {}", line);
        };
    }

    // Return the graph.
    graph
}

/// Get the optimal happiness for the graph.
fn get_optimal_happiness(graph: &DiGraphMap<&str, isize>) -> isize {
    // Initialize the optimal happiness to the minimum isize value.
    let mut optimal_happiness = isize::MIN;

    /* For each node in the graph, find the maximum happiness that can be
    achieved by starting at that node and traversing the graph. */
    for node in graph.nodes() {
        // Initialize the maximum happiness to zero.
        let mut max_happiness = 0;
        // Initialize the starting node to the current node.
        let mut start_node = node;
        // Clone the graph.
        let mut graph_clone = graph.clone();

        // Traverse the graph until only one node remains.
        while graph_clone.node_count() > 1 {
            // Initialize the maximum edge happiness to the minimum isize value.
            let mut max_edge_happiness = isize::MIN;
            // Initialize the next node to an empty string.
            let mut next_node = "";

            // Find the neighboring node with the maximum edge happiness.
            for neighbor in graph_clone.neighbors(start_node) {
                // Get the happiness of the edge between the nodes.
                let happiness = graph_clone.edge_weight(start_node, neighbor).unwrap()
                    + graph_clone.edge_weight(neighbor, start_node).unwrap();

                /* If the happiness is greater than the current maximum,
                update the maximum. */
                if happiness > max_edge_happiness {
                    max_edge_happiness = happiness;
                    next_node = neighbor;
                }
            }

            // Update the maximum happiness.
            max_happiness += max_edge_happiness;
            // Remove the starting node from the graph.
            graph_clone.remove_node(start_node);
            // Set the starting node to the next node.
            start_node = next_node;
        }

        // Add the happiness of the edge between the last and first nodes.
        max_happiness += graph.edge_weight(start_node, node).unwrap()
            + graph.edge_weight(node, start_node).unwrap();

        /* If the maximum happiness is greater than the current optimal
        happiness, update the optimal happiness. */
        if max_happiness > optimal_happiness {
            optimal_happiness = max_happiness;
        }
    }

    // Return the optimal happiness.
    optimal_happiness
}

/// Add me to the graph.
fn add_me_to_graph<'a>(graph: &'a DiGraphMap<&'a str, isize>) -> DiGraphMap<&'a str, isize> {
    // Clone the graph.
    let mut graph_with_me = graph.clone();

    // Add me to the graph.
    for node in graph.nodes() {
        graph_with_me.add_edge("Me", node, 0);
        graph_with_me.add_edge(node, "Me", 0);
    }

    // Return the graph with me.
    graph_with_me
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    /// Get the example input for part 1 of the puzzle.
    fn get_part1_input() -> &'static str {
        r#"
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
"#
        .trim()
    }

    /// Test the parsing of the input in the example for part 1.
    #[test]
    fn test_parse_input() {
        let input = get_part1_input();
        let graph = parse_input(input);

        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 12);

        let edge_map = HashMap::from([
            (("Alice", "Bob"), 54),
            (("Alice", "Carol"), -79),
            (("Alice", "David"), -2),
            (("Bob", "Alice"), 83),
            (("Bob", "Carol"), -7),
            (("Bob", "David"), -63),
            (("Carol", "Alice"), -62),
            (("Carol", "Bob"), 60),
            (("Carol", "David"), 55),
            (("David", "Alice"), 46),
            (("David", "Bob"), -7),
            (("David", "Carol"), 41),
        ]);

        graph.all_edges().for_each(|edge| {
            assert_eq!(edge_map.get(&(edge.0, edge.1)), Some(edge.2));
        });
    }

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        let input = get_part1_input();
        let graph = parse_input(input);

        assert_eq!(get_optimal_happiness(&graph), 330)
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (618, 601));
    }
}
