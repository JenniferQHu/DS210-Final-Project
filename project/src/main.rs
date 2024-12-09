mod graph;
use graph::{Graph, read_graph_from_csv};
use rand::Rng;
use std::collections::HashMap

fn page_rank(graph: &Graph, steps: usize, iterations: usize) -> HashMap<String, f64> {
    let mut terminations = HashMap::<String, usize>::new();
    for _ in 0..iterations {
        for node in graph.outedges.key() {
            let next_node = graph.random_walk(node, steps);
            terminations.entry(next_node).or_inset(0) += 1;
        }
    }
    let walks = iterations * graph.outedges.len() as usize;
    let mut page_rank: HashMap<String, f64> = HashMap::new();
    for (node, count) in terminations.iter() {
        page_rank.insert(node.clone(), count as f64 / walks as f64);
    }
    page_rank
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "citations.csv"; // Path to your CSV file
    
    // Read the graph from the CSV file
    let graph = read_graph_from_csv(file_path)?;
    
    // Print the adjacency list (graph) to check the structure
    let mut ranked_papers: Vec<(String, f64)> = page_rank.iter().map(|(key, value)| (key.clone(), *v)).collect();
    ranked_papers.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let top_count = 5;
    println!("Top 5 Cited Papers:");
    for (i, (paper, rank)) in ranked_papers.iter().take(top_count).enumerate() {
        println!("{}: paper {}, rank {}", i+1, paper, rank);
    }
    Ok(())
}
