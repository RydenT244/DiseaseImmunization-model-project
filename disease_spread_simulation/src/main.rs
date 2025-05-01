//main fucntion runs diease spread and immunization strategies

mod graph;
mod disease;
mod immunization;
use disease::simulate_infection;
//use disease::plot_infection_timeline;
//use disease::test_multiple_thresholds_and_probs;
use graph::Graph;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::collections::HashSet;
use rand::seq::{SliceRandom, IteratorRandom}; 
//use std::collections::HashMap;
use std::cmp::Reverse;
/*
//runs tirals across combinations of threshold, probabilties, and immune counts
//to compare random and target immunization
fn analyze_immunization_effectiveness(
    graph: &Graph,
    num_trials: usize,
    source: u32,
    thresholds: &[u32],
    probs: &[f64],
    immune_counts: &[usize],
) {
    println!("\n=== IMMUNIZATION STRATEGY COMPARISON ===");

    for &threshold in thresholds {
        for &prob in probs {
            for &num_immune in immune_counts {
                let mut deltas = vec![];

                for trial_seed in 0..num_trials {
                    let (infected_random, infected_targeted) = compare_immunization_strategies(
                        graph,
                        source,
                        threshold,
                        prob,
                        num_immune,
                        trial_seed as u64,
                    );
                let delta = infected_random as isize - infected_targeted as isize;
                deltas.push(delta); 

                }

                let avg_delta: f64 =
                    deltas.iter().copied().sum::<isize>() as f64 / deltas.len() as f64;

                println!(
                    "Threshold: {:<2}, Prob: {:.2}, Immune: {:<3} → Avg Δ (Random - Targeted): {:.2}",
                    threshold, prob, num_immune, avg_delta
                );
            }
        }
    }
}
*/

/*
//prints degree conneciton for a set of nodes - used for testing
fn print_node_degrees(edges: &HashMap<u32, Vec<(u32, u32)>>, nodes: &HashSet<u32>) {
    let mut degree_info: Vec<(u32, usize)> = nodes
        .iter()
        .map(|&n| (n, edges.get(&n).map_or(0, |v| v.len())))
        .collect();
    degree_info.sort_by(|a, b| b.1.cmp(&a.1));
    for (node, degree) in degree_info {
        println!("  Node {} → degree {}", node, degree);
    }
}

//finds all nodes reached from a starting node - testing
fn reachable_nodes(
    graph: &HashMap<u32, Vec<(u32, u32)>>,
    start: u32,
) -> HashSet<u32> {
    let mut visited = HashSet::new();
    let mut queue = vec![start];

    while let Some(node) = queue.pop() {
        if visited.insert(node) {
            if let Some(neighbors) = graph.get(&node) {
                for (neighbor, _) in neighbors {
                    queue.push(*neighbor);
                }
            }
        }
    }

    visited
}
//counts how many immune nodes a rereachable from source 
fn count_reachable_immune(
    graph: &HashMap<u32, Vec<(u32, u32)>>,
    source: u32,
    immune_nodes: &HashSet<u32>,
) -> usize {
    let reachable = reachable_nodes(graph, source);
    immune_nodes.intersection(&reachable).count()
}
*/
// compares and simulates disease spread after random immunization and targeted
fn compare_immunization_strategies(
    graph: &Graph,
    source: u32,
    infection_threshold: u32,
    infection_prob: f64,
    num_immune: usize,
    seed: u64,
) -> (usize, usize) {
    let mut rng = StdRng::seed_from_u64(seed);

    // True random immunization (uniform sampling)
    let top_n_percent = 0.15; // top 15% degree
    let mut nodes_by_degree: Vec<(u32, usize)> = graph.edges.iter()
        .map(|(node, neighbors)| (*node, neighbors.len()))
        .collect();

    nodes_by_degree.sort_by_key(|&(_, deg)| Reverse(deg));
    let cutoff = (nodes_by_degree.len() as f64 * top_n_percent) as usize;

    let high_degree_candidates: Vec<u32> = nodes_by_degree
        .iter()
        .take(cutoff)
        .map(|&(node, _)| node)
        .collect();

    let random_immune: HashSet<u32> = high_degree_candidates
        .choose_multiple(&mut rng, num_immune)
        .cloned()
        .collect();


    //random immunizaiton
    let mut random_graph = graph.edges.clone();
    immunization::apply_immunization(&mut random_graph, &random_immune);
    let mut rng1 = StdRng::seed_from_u64(seed + 1);
    let infected_random = simulate_infection(&random_graph, source, infection_threshold, infection_prob, &mut rng1);

    //targeted immunization
    let targeted_immune = immunization::select_high_degree_nodes(&graph.edges, num_immune);

    let mut targeted_graph = graph.edges.clone();
    immunization::apply_immunization(&mut targeted_graph, &targeted_immune);
    let mut rng2 = StdRng::seed_from_u64(seed + 2);
    let infected_targeted = simulate_infection(&targeted_graph, source, infection_threshold, infection_prob, &mut rng2);
/*
    // Print for debugging
    println!("\n[Trial {}] Immune Count: {}", seed, num_immune);
    println!("Random Immune Nodes:   {:?}", random_immune);
    println!("Targeted Immune Nodes: {:?}", targeted_immune);
    println!("Infected (Random):     {}", infected_random.len());
    println!("Infected (Targeted):   {}", infected_targeted.len());
    println!("Degrees (Random):");
    print_node_degrees(&graph.edges, &random_immune);
    println!("Degrees (Targeted):");
    print_node_degrees(&graph.edges, &targeted_immune);
    let reachable_random = count_reachable_immune(&graph.edges, source, &random_immune);
    println!("Random immune nodes reachable from source: {}/{}", reachable_random, random_immune.len());
*/
    (
        infected_random.len(),
        infected_targeted.len(),
    )
}

//runs fifty trials total, 10 different seeds for each of 5 differnt soruces then 
//computes and prints avg number of infected per strategy 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut graph = Graph::new();
    graph.load_from_file("./data/weightedEdgeList.txt")?;

    let num_trials: usize = 10;
    let num_sources: usize = 5;
    let num_immune = 50;
    let threshold = 10;
    let prob = 0.5;


    let mut rng = StdRng::seed_from_u64(123); // fixed seed for reproducibility
    
    // select strong (high-degree) source nodes
    let mut node_degrees: Vec<(u32, usize)> = graph
        .edges
        .iter()
        .map(|(&node, neighbors)| (node, neighbors.len()))
        .collect();
    node_degrees.sort_by(|a, b| b.1.cmp(&a.1)); // Descending by degree

    let top_k = 100;
    let top_nodes: Vec<u32> = node_degrees.iter().take(top_k).map(|&(node, _)| node).collect();
    let source_nodes: Vec<u32> = top_nodes
        .iter()
        .cloned()
        .choose_multiple(&mut rng, num_sources);

    //println!("Selected source nodes: {:?}\n", source_nodes);

    let mut total_random = 0;
    let mut total_targeted = 0;
    let mut total_baseline = 0;
    let mut total_trials = 0;

    for &source in &source_nodes {
        //println!("=== Source Node: {} ===", source);
        for trial_seed in 0..num_trials {
            let mut trial_rng = StdRng::seed_from_u64(trial_seed as u64);

            let infected = simulate_infection(&graph.edges, source, threshold, prob, &mut trial_rng);
            //println!("Trial {}, No Immunization -> Infected: {}", trial_seed, infected.len());
            total_baseline += infected.len();

            let (random, targeted) = compare_immunization_strategies(
                &graph,
                source,
                threshold,
                prob,
                num_immune,
                trial_seed as u64,
            );

            //println!("Random Immunization -> Infected: {}", random);
            //println!("Targeted Immunization -> Infected: {}", targeted);

            total_random += random;
            total_targeted += targeted;
            total_trials += 1;
        }
    }

    println!("\n=== AVERAGE RESULTS OVER {} TOTAL TRIALS ===", total_trials);
    println!("No Immunization Avg Infected: {}", total_baseline / total_trials);
    println!("Random Immunization Avg Infected: {}", total_random / total_trials);
    println!("Targeted Immunization Avg Infected: {}", total_targeted / total_trials);

/* mroe testing 
    let thresholds = vec![6, 9, 12];
    let probs = vec![0.1, 0.3, 0.6];
    let immune_counts = vec![10, 20, 30, 40, 50];

    analyze_immunization_effectiveness(&graph, 10, 1, &thresholds, &probs, &immune_counts);

    // graph.print_graph(); // test graph

    // analyze contact times in data
    graph.analyze_contact_times();
    let probs = vec![0.1, 0.2, 0.5];
    let thresholds = vec![1, 5, 10, 12, 15];
    test_multiple_thresholds_and_probs(&graph.edges, 1, &thresholds, &probs, &mut rng);
    */

    /*
    //  single run plot with st seed
    let seed = 64;
    let mut rng = StdRng::seed_from_u64(seed);
    let infected = simulate_infection(&graph.edges, 1, 6, 0.60, &mut rng);
    println!("\nInfection Results:");
    for (node, time) in &infected {
        println!("Node {} infected at time {}", node, time);
    }
    plot_infection_timeline(&infected);
    */

    Ok(())
}
