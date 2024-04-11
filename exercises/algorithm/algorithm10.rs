/*
	graph
	This problem requires you to implement a basic graph functio
*/
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl UndirectedGraph {
    pub fn new() -> Self {
        Self {
            adjacency_table: HashMap::new(),
        }
    }

    pub fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    pub fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
}

impl Graph for UndirectedGraph {
    fn new() -> Self {
        Self::new()
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        self.adjacency_table_mutable()
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        self.adjacency_table()
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (node1, node2, weight) = edge;

        // Ensure both nodes exist in the graph
        self.add_node(node1);
        self.add_node(node2);

        // Add edge from node1 to node2
        self.adjacency_table_mutable().entry(node1.to_string())
            .or_default()
            .push((node2.to_string(), weight));

        // Add edge from node2 to node1 (since it's undirected)
        self.adjacency_table_mutable().entry(node2.to_string())
            .or_default()
            .push((node1.to_string(), weight));
    }

    fn edges(&self) -> Vec<(String, String, i32)> {
        let mut edges = Vec::new();
        let mut seen = HashSet::new();

        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                let sorted_nodes = if from_node < to_node {
                    (from_node.clone(), to_node.clone())
                } else {
                    (to_node.clone(), from_node.clone())
                };

                if !seen.contains(&(sorted_nodes.0.clone(), sorted_nodes.1.clone(), *weight)) {
                    let edge = (sorted_nodes.0.clone(), sorted_nodes.1.clone(), *weight);
                    edges.push(edge.clone());
                    seen.insert(edge);
                }
            }
        }

        edges
    }

}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        if !self.contains(node) {
            self.adjacency_table_mutable().insert(node.to_string(), Vec::new());
            true
        } else {
            false
        }
    }
    fn add_edge(&mut self, edge: (&str, &str, i32));
    fn edges(&self) -> Vec<(String, String, i32)>;
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (String::from("a"), String::from("b"), 5),
            (String::from("a"), String::from("c"), 7),
            (String::from("b"), String::from("c"), 10),
        ];

        let actual_edges = graph.edges();

        for edge in expected_edges.iter() {
            assert!(actual_edges.contains(edge));
        }
    }
}
