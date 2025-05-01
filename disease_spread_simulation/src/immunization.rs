//fucntions to simulate immunization strategies by removing nodes from a graph
// and identifying high contact individuals based on node degree.
use std::collections::{HashMap, HashSet};
//use rand::seq::IteratorRandom;
//use std::collections::VecDeque;

pub fn apply_immunization(//removes specific "immune nodes from graph and refrences to them from neighbors"
    graph: &mut HashMap<u32, Vec<(u32, u32)>>,
    immune_nodes: &HashSet<u32>,
) {
    for &immune_node in immune_nodes {
        graph.remove(&immune_node); 
    }

    for neighbors in graph.values_mut() {
        neighbors.retain(|(neighbor, _)| !immune_nodes.contains(neighbor));// removes neibors refernce to them (takes them out of disease spread)
    }
}



//selects count number of nodes with highest degree connection from my hashmap
pub fn select_high_degree_nodes(
    edges: &HashMap<u32, Vec<(u32, u32)>>,
    count: usize,
) -> HashSet<u32> {
    let mut degrees: Vec<(u32, usize)> = edges
        .iter()
        .map(|(&node, neighbors)| (node, neighbors.len()))
        .collect();

    degrees.sort_by(|a, b| b.1.cmp(&a.1)); // Descending degree

    degrees.into_iter().take(count).map(|(n, _)| n).collect()
}
