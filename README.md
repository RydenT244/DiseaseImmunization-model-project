Evaluating Immunization Strategies in Network-Based Disease Spread Using Threshold Models

A. Project Overview
Goal:
Evaluate and compare the effectiveness of random versus targeted immunization strategies in reducing disease spread across a graph bfs based contact network spreading.

Question:
How does targeted immunization (using node degree) perform compared to random immunization in terms of minimizing the infected number of nodes?

Dataset:
Source: "High-Resolution Human Contact Network for Infectious Disease Transmission"
Size: records over 760,000 close-proximity interactions among 788 individuals at an American high school during one day.
Location: ./data/weightedEdgeList.txt

B. Data Processing
Loading Into Rust: 
Created a graph.rs file that loads the data from txt file using the Graph::load_from_file() method. Each line the the file represents an edge in the form (node1 node2 weight)

Transformations:
Data is parsed into a HashMap<u32, Vec<(u32, u32)>> structure. Duplicate and symmetric edges are normalized.

C. Code Structure
Modules:
Graph - used for loading and representing graph network
Disease - used for simulating disease spread over time
Immunization - used for implementing immunization strategies: random and high degree targeting 

Key Function & Types:
Struct: Graph 
Purpose: Represents the contact network
Fields:  edges: HashMap<u32, Vec<(u32, u32)>>
Core methods: 
load_from_file(path) - parse and loads weighted edge list
analyze_contact_times(): test tool to inspect weight (time) distribution

	(Disease.rs) Function: simulate_infection()
Purpose: runs disease spread simulation from a source node
Inputs:
 &graph_edges, source_node, threshold, infection_probability, rng
Outputs: 
HashMap<u32, u32> of infected nodes with timestamp
Logic:
Breadth first simulation only affects does once a threshold number of contact duration is passed and a random probability filter

	(Immunization.rs) Function: apply_immunization()
Purpose: removes nodes from graph to simulate immunization
Inputs: mutable reference to graph edges and set of immune does
Effect: Delete immune does and their edges

	(Immunization.rs) Function: select_high_degree_nodes()
Purpose: chooses top N (count) nodes with highest degree immunization 
Logic: sorts all nodes by degree and returns top n

	(Main.rs) Function: compare_immunization_strategies()
Purpose: simulate both random and targeted immunization strategies and compares infected counts
Input: 
Graph, source, threshold, prob, num_immune, RNG seed.
Outputs:
(random_infected_count, targeted_infected_count)

	Main Workflow Summary:
Load graph 
Select 5 source node (top-degree)
For each of 10 seeds per source(50 total trials):
Run baseline infection (no immunization)
Run with random immunization
Run with targeted immunization
Average the infected counts and print at end

D. Test
I used the test built in the rust, but also throughout my project I commented out a lot of the test function for finding optimal disease spreading and immunization (commented out with explanation in project):  

Cargo test functions:
test_simple_infection
Purpose: validate correct spread though a small network with prob = 1.0
Confirms function respects duration threshold and time step propagation
test_no_infection _spread
Purpose: ensure no nodes are infected if all edge duration below threshold
Verifies filtering threshold prevents spread
test_full_infection
Purpose: checks complete infection of all nodes in a fully connected graph under max probability and adequate duration 
Ensures the algorithm performs correctly in ideal transmission conditions
test_isolated_nodes
Purpose: validates that isolated or disconnected nodes never get infected 
Confirms algorithm handles disconnected components properly 
test_probabilistic_infection
Purpose: checks stochastic infection logic by using a seeded RNG and a prob less than 1
Ensures consistent, reproducible partial infection using probability rules  

	Cargo test output:
	running 5 tests
test disease::tests::test_full_infection ... ok
test disease::tests::test_isolated_nodes ... ok
test disease::tests::test_no_infection_spread ... ok
test disease::tests::test_simple_infection ... ok
test disease::tests::test_probabilistic_infection ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

E. Results
For:
num_immune (# of immune nodes): 50
Threshold (duration time (sec)): 10
Prob (getting infected after threshold met): 0.5
Output:
=== AVERAGE RESULTS OVER 50 TOTAL TRIALS ===
No Immunization Avg Infected: 777
Random Immunization Avg Infected: 451
Targeted Immunization Avg Infected: 291

For (less immunities) 
num_immune (# of immune nodes): 30
Threshold (duration time (sec)): 10
Prob (getting infected after threshold met): 0.5
Output:
=== AVERAGE RESULTS OVER 50 TOTAL TRIALS ===
No Immunization Avg Infected: 777
Random Immunization Avg Infected: 583
Targeted Immunization Avg Infected: 448

For (more aggressive spread higher prob)
num_immune (# of immune nodes): 50
Threshold (duration time (sec)): 10
Prob (getting infected after threshold met): 0.95
Output:
=== AVERAGE RESULTS OVER 50 TOTAL TRIALS ===
No Immunization Avg Infected: 779
Random Immunization Avg Infected: 452
Targeted Immunization Avg Infected: 292

Interpretation: 
Targeted immunization consistently reduces infection more effectively than random immunization, supporting the hypothesis that targeting high-degree nodes is more efficient in network immunization.

F. Usage instructions
	Building and Running:
	Console commands:
		cargo build –release
		cargo run –release
	Requirements:
Rust
Data file (at ./data/weightedEdgeList.txt)

	Runtime:
~2-3 seconds on my machine and with my graph size

No command-line arguments 
configuration is hardcoded in main()
Infection threshold: 10 (u32)
Infection probability: 0.5 (f64)
Immune count: 50 (usize)
	



 



	
