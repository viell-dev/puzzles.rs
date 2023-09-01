//! Day 9: All in a Single Night

use petgraph::prelude::UnGraphMap;

/// Print the answers for the puzzle to the console.
fn main() {
    let input = include_str!("input.txt").trim();
    let (shortest_distance, longest_distance) = get_answers(input);

    println!(
        "The shortest distance Santa can travel is: {}",
        shortest_distance
    );
    println!(
        "The longest distance Santa can travel is: {}",
        longest_distance
    );
}

/// Get the answers for the puzzle.
fn get_answers(input: &str) -> (usize, usize) {
    let graph = get_graph(input);

    let shortest_distance = get_shortest_distance(&graph);
    let longest_distance = get_longest_distance(&graph);

    (shortest_distance, longest_distance)
}

/// Convert the input into a undirected graph with the distances as weights.
fn get_graph(input: &str) -> UnGraphMap<&str, usize> {
    // Initialize the graph.
    let mut graph = UnGraphMap::<&str, usize>::new();

    // For each line in the input, add an edge to the graph.
    for line in input.lines() {
        // Split the line into the from, to, and distance.
        let (from, to, distance) = if let [from, _, to, _, distance] =
            line.split_whitespace().collect::<Vec<_>>().as_slice()
        {
            /* Parse the distance into a usize.
            And return the from, to, and distance. */
            (*from, *to, distance.parse::<usize>().unwrap())
        } else {
            panic!("Invalid input.");
        };

        // Add the edge to the graph.
        graph.add_edge(from, to, distance);
    }

    // Return the graph.
    graph
}

/// Get the shortest distance Santa can travel.
fn get_shortest_distance(graph: &UnGraphMap<&str, usize>) -> usize {
    // Initialize the overall shortest distance.
    let mut overall_shortest_distance = usize::MAX;

    // For each node in the graph, get the shortest distance to all other nodes.
    for node in graph.nodes() {
        // Initialize the shortest distance.
        let mut shortest_distance = 0;
        // Initialize the start node.
        let mut start_node: Option<&str> = Some(node);

        // Clone the graph.
        let mut graph_clone = graph.clone();

        /* While there are still nodes in the graph,
        get the shortest distance to the next node. */
        while let Some(start) = start_node {
            // If there is only one node left, break.
            if graph_clone.node_count() == 1 {
                break;
            }

            // Initialize the shortest edge.
            let mut shortest_edge = usize::MAX;
            // Initialize the next node.
            let mut next_node = "";

            // For each node in the graph, get the shortest distance to every other node.
            for other_node in graph_clone.edges(start) {
                /* If the node is not the start node and the distance is shorter
                than the shortest edge, update the shortest edge and next node. */
                if other_node.1 != start && other_node.2 < &shortest_edge {
                    next_node = other_node.1;
                    shortest_edge = *other_node.2;
                }
            }

            // Add the shortest edge to the shortest distance.
            shortest_distance += shortest_edge;
            // Update the start node to the next node.
            start_node = (!next_node.is_empty()).then(|| next_node);
            // Remove the start node from the graph.
            graph_clone.remove_node(start);
        }

        /* If the shortest distance is shorter than the overall shortest
        distance, update the overall shortest distance. */
        if shortest_distance < overall_shortest_distance {
            overall_shortest_distance = shortest_distance;
        }
    }

    // Return the overall shortest distance.
    overall_shortest_distance
}

/// Get the longest distance Santa can travel.
fn get_longest_distance(graph: &UnGraphMap<&str, usize>) -> usize {
    // Initialize the overall longest distance.
    let mut overall_longest_distance = 0;

    // For each node in the graph, get the longest distance to all other nodes.
    for node in graph.nodes() {
        // Initialize the longest distance.
        let mut longest_distance = 0;
        // Initialize the start node.
        let mut start_node: Option<&str> = Some(node);

        // Clone the graph.
        let mut graph_clone = graph.clone();

        /* While there are still nodes in the graph,
        get the longest distance to the next node. */
        while let Some(start) = start_node {
            // If there is only one node left, break.
            if graph_clone.node_count() == 1 {
                break;
            }

            // Initialize the longest edge.
            let mut longest_edge = 0;
            // Initialize the next node.
            let mut next_node = "";

            // For each node in the graph, get the longest distance to every other node.
            for other_node in graph_clone.edges(start) {
                /* If the node is not the start node and the distance is longer
                than the longest edge, update the longest edge and next node. */
                if other_node.1 != start && other_node.2 > &longest_edge {
                    next_node = other_node.1;
                    longest_edge = *other_node.2;
                }
            }

            // Add the longest edge to the longest distance.
            longest_distance += longest_edge;
            // Update the start node to the next node.
            start_node = (!next_node.is_empty()).then(|| next_node);
            // Remove the start node from the graph.
            graph_clone.remove_node(start);
        }

        /* If the longest distance is longer than the overall longest
        distance, update the overall longest distance. */
        if longest_distance > overall_longest_distance {
            overall_longest_distance = longest_distance;
        }
    }

    // Return the overall longest distance.
    overall_longest_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Get the test graph.
    fn get_test_graph() -> UnGraphMap<&'static str, usize> {
        // The test input.
        let input = r#"
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
"#
        .trim();

        // Generate and return the graph.
        get_graph(input)
    }

    /// Test the examples given for part 1 on the puzzle page.
    #[test]
    fn test_part1_examples() {
        assert_eq!(get_shortest_distance(&get_test_graph()), 605);
    }

    /// Test the examples given for part 2 on the puzzle page.
    #[test]
    fn test_part2_examples() {
        assert_eq!(get_longest_distance(&get_test_graph()), 982);
    }

    /// Test the answers against the correct answers.
    #[test]
    fn test_answers() {
        let input = include_str!("input.txt").trim();
        assert_eq!(get_answers(input), (141, 736));
    }
}
