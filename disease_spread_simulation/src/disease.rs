//simulates disease spreading over a weighted contact graph and includes 
//other fucntions for testing and visualization
//aso includes tests for disease spreading

use std::collections::{HashMap, HashSet, VecDeque};
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
//use plotters::prelude::*;
//use plotters::style::colors::BLUE;

pub fn simulate_infection(//spreads infection from a source node acroos the grpah using contact duration thresholds and transmission probs
    graph: &HashMap<u32, Vec<(u32, u32)>>, // node id and list of (neighbor, contact_duration)
    source: u32,
    infection_threshold: u32,
    infection_prob: f64,
    rng: &mut impl Rng, //for seeding
) -> HashMap<u32, u32> {
    let mut infected: HashMap<u32, u32> = HashMap::new();
    let mut visited: HashSet<u32> = HashSet::new();
    let mut queue: VecDeque<(u32, u32)> = VecDeque::new();

    infected.insert(source, 0);
    visited.insert(source);
    queue.push_back((source, 0));

    while let Some((current, time)) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&current) {
            for &(neighbor, duration) in neighbors {
                if !visited.contains(&neighbor) && duration >= infection_threshold {
                    if rng.gen_bool(infection_prob) {
                        infected.insert(neighbor, time + 1);
                        visited.insert(neighbor);
                        queue.push_back((neighbor, time + 1));
                    }
                }
            }
        }
    }

    infected
}
// test multiple thresholds and probabilites at a time with a given seed 
/*
pub fn test_multiple_thresholds_and_probs(
    graph: &HashMap<u32, Vec<(u32, u32)>>,
    source: u32,
    thresholds: &[u32],
    probabilities: &[f64],
    mut rng: impl Rng,
) {
    println!("--- Threshold & Probability Testing ---");
    for &prob in probabilities {
        println!(">> Infection probability: {:.2}", prob);
        for &threshold in thresholds {
            let infected = simulate_infection(graph, source, threshold, prob, &mut rng);
            let max_steps = infected.values().copied().max().unwrap_or(0);

            println!(
                "   Threshold {:>3} sec: {:>4} infected nodes | Max steps: {:>2}",
                threshold,
                infected.len(),
                max_steps
            );
        }
        println!();
    }
}
*/

/*
//plot single disease spreading on a chart across steps
pub fn plot_infection_timeline(infected: &HashMap<u32, u32>) {
    let root = BitMapBackend::new("single_infection_timeline.png", (640, 480))
        .into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Single Infection Timeline", ("Arial", 30))
        .build_cartesian_2d(0..100, 0..*infected.values().max().unwrap())
        .unwrap();

    chart.configure_mesh().x_desc("Time Step").y_desc("Number of Infected").draw().unwrap();

    let mut times = vec![0; *infected.values().max().unwrap() as usize + 1];
    
    for &time in infected.values() {
        times[time as usize] += 1;
    }

    chart
        .draw_series(
            times.into_iter().enumerate().map(|(x, y)| {
                Rectangle::new(
                    [(x as i32, 0), (x as i32 + 1, y)],
                    BLUE.filled(),
                )
            }),
        )
        .unwrap();
    
}
*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_infection() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![(2, 100), (3, 30)]);
        graph.insert(2, vec![(1, 100), (4, 70)]);
        graph.insert(3, vec![(1, 30)]);
        graph.insert(4, vec![(2, 70)]);

        let mut rng = StdRng::seed_from_u64(42);
        let infected = simulate_infection(&graph, 1, 60, 1.0, &mut rng);

        let mut expected = HashMap::new();
        expected.insert(1, 0);
        expected.insert(2, 1);
        expected.insert(4, 2);

        assert_eq!(infected, expected);
    }

    #[test]
    fn test_no_infection_spread() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![(2, 10), (3, 5)]);
        graph.insert(2, vec![(1, 10)]);
        graph.insert(3, vec![(1, 5)]);

        let mut rng = StdRng::seed_from_u64(42);
        let infected = simulate_infection(&graph, 1, 60, 1.0, &mut rng);
    
        let mut expected = HashMap::new();
        expected.insert(1, 0); // Only patient zero infected
    
        assert_eq!(infected, expected);
    }

    #[test]
    fn test_full_infection() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![(2, 100), (3, 100)]);
        graph.insert(2, vec![(1, 100), (3, 100)]);
        graph.insert(3, vec![(1, 100), (2, 100)]);
    
        let mut rng = StdRng::seed_from_u64(42);
        let infected = simulate_infection(&graph, 1, 60, 1.0, &mut rng);
    
        let mut expected = HashMap::new();
        expected.insert(1, 0);
        expected.insert(2, 1);
        expected.insert(3, 1);
    
        assert_eq!(infected, expected);
    }

    #[test]
    fn test_isolated_nodes() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![(2, 100)]);
        graph.insert(2, vec![(1, 100)]);
        graph.insert(3, vec![]); // Node 3 and 4 are isolated
        graph.insert(4, vec![]); 
    
        let mut rng = StdRng::seed_from_u64(42);
        let infected = simulate_infection(&graph, 1, 60, 1.0, &mut rng);
    
        let mut expected = HashMap::new();
        expected.insert(1, 0);
        expected.insert(2, 1);
    
        assert_eq!(infected, expected);
    }

    #[test]
    fn test_probabilistic_infection() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![(2, 100), (3, 100)]);
        graph.insert(2, vec![(1, 100), (4, 100)]);
        graph.insert(3, vec![(1, 100)]);
        graph.insert(4, vec![(2, 100)]);

        let mut rng = StdRng::seed_from_u64(42); // fixed seed for consistent results
        let infected = simulate_infection(&graph, 1, 60, 0.5, &mut rng);

        
        assert!(infected.contains_key(&1)); 
}

}
