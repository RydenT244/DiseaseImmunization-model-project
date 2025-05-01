//loads contact data from data file as a weighted undiret graph using an adjnacnecy list 
// as a Hashmap, each key is the node ID with the value being a list of (neighbor, weight)

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Graph {
    pub edges: HashMap<u32, Vec<(u32, u32)>>, // Node -> list of (Neighbor, Weight)
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }
    // file loader
    pub fn load_from_file(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() != 3 {
                continue; // skip badly formatted lines
            }
            let node1: u32 = parts[0].parse()?;
            let node2: u32 = parts[1].parse()?;
            let weight: u32 = parts[2].parse()?; 

            self.edges.entry(node1).or_insert_with(Vec::new).push((node2, weight));
            self.edges.entry(node2).or_insert_with(Vec::new).push((node1, weight));
        }

        Ok(())
    }
    /* testing grpah structure
    pub fn print_graph(&self) {
        println!("Graph structure:");
        for (node, neighbors) in &self.edges {
            print!("Node {} connects to: ", node);
            for (neighbor, weight) in neighbors {
                print!("(Node {}, Duration {}s) ", neighbor, weight);
            }
            println!();
        }
    }
    
    //summarize contacts times 
    pub fn analyze_contact_times(&self) {
        let mut all_times = vec![];
        for edges in self.edges.values() {
            for &(_, time_spent) in edges {
                all_times.push(time_spent);
            }
        }

        let min_time = all_times.iter().min().unwrap();
        let max_time = all_times.iter().max().unwrap();
        let avg_time = all_times.iter().sum::<u32>() as f32 / all_times.len() as f32;

        println!("Contact times - min: {}s, max: {}s, avg: {:.2}s", min_time, max_time, avg_time);
    }
    */
    
}
