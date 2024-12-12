mod graph;
mod clustering;
use graph::Graph;
use clustering::Cluster;
use std::error::Error;
use std::collections::HashMap;

fn page_rank(graph: &Graph, steps: usize, iterations: usize) -> HashMap<String, f64> {
    let mut terminations = HashMap::<String, usize>::new();
    for _ in 0..iterations {
        for node in graph.outedges.keys() {
            let next_node = graph.random_walk(node, steps);
            *terminations.entry(next_node).or_insert(0) += 1;
        }
    }
    let walks = iterations * graph.outedges.len() as usize;
    let mut page_rank: HashMap<String, f64> = HashMap::new();
    for (node, count) in terminations.iter() {
        page_rank.insert(node.clone(), *count as f64 / walks as f64);
    }
    page_rank
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "citations.csv"; // Path to your CSV file
    
    // Read the graph from the CSV file
    let graph = Graph::read_graph_from_csv(file_path)?;
    
    // let computed_page_rank = page_rank(&graph, 10, 50);
    // // Print the adjacency list (graph) to check the structure
    // let mut ranked_papers: Vec<(String, f64)> = computed_page_rank.iter().map(|(key, value)| (key.clone(), *value)).collect();
    // ranked_papers.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    // let top_count = 5;
    // println!("Top 5 Cited Papers:");
    // for (i, (paper, rank)) in ranked_papers.iter().take(top_count).enumerate() {
    //     println!("{}: paper {}, rank {}", i+1, paper, rank);
    // }

    let clustering = Cluster::kmeans_clustering(&graph.outedges, 10, 10);// k = 10, 10 iterations
    for (i, cluster) in clustering.iter().enumerate() {
        println!("Cluster {}:", i + 1);
        println!("Centroid: {}", cluster.centroids);
        println!("Members: {:?}", cluster.members);
    }

    Ok(())
}
