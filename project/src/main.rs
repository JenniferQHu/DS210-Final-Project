mod graph;
mod clustering;
use graph::Graph;
use clustering::Cluster;
use std::error::Error;
use std::collections::HashMap;

pub fn page_rank(graph: &Graph, steps: usize, iterations: usize) -> HashMap<String, f64> {
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
    for (node, neighbors) in &graph.outedges { 
        println!("{:?} -> {:?}", node, neighbors); 
    }

    let k = 10;
    let iterations = 50;
    println!("\nComputing PageRank for {} steps and {} iterations, will take approximately 10 mins", k, iterations); // Print the adjacency list to get an idea of the structure

    let computed_page_rank = page_rank(&graph, k, iterations);
    
    let mut ranked_papers: Vec<(String, f64)> = computed_page_rank.iter().map(|(key, value)| (key.clone(), *value)).collect();
    ranked_papers.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let top_count = 5;
    println!("Top 5 Cited Papers:");
    for (i, (paper, rank)) in ranked_papers.iter().take(top_count).enumerate() {
        println!("{}: paper {}, rank {}", i+1, paper, rank);
    }

    println!("\nPerforming Kmeans Clustering for 3 centroids for 10 iterations, will take approximately 2 mins");
    let clustering = Cluster::kmeans_clustering(&graph.outedges, 3, 10);// k = 3, 10 iterations
    for (i, cluster) in clustering.iter().enumerate() {
        println!("Cluster {}:", i + 1);
        println!("Centroid: {:?}", cluster.centroids);
        println!("Members: {:?}", cluster.members);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagerank_sum_to_one() {
        let file_path = "citations.csv";
        let graph = Graph::read_graph_from_csv(file_path).expect("Failed to read graph from CSV file");
        let page_rank_values = page_rank(&graph, 5, 2);

        let sum: f64 = page_rank_values.values().sum();
        let tolerance = 1e-9; // Allowable error margin

        assert!((sum - 1.0).abs() < tolerance, "Sum of PageRank values is not approximately 1.0. Actual sum: {:.6}", sum);
    }

    #[test]
    fn test_kmeans_clustering() {
    let edges = vec![
        ("A".to_string(), "B".to_string()),
        ("A".to_string(), "C".to_string()),
        ("B".to_string(), "D".to_string()),
        ("C".to_string(), "D".to_string()),
        ("D".to_string(), "E".to_string()),
    ];

    // Create a small graph with these edges
    let graph = Graph::create_directed_graph(edges);

    // Perform clustering (e.g., k=2 clusters, 5 iterations)
    let clusters = Cluster::kmeans_clustering(&graph.outedges, 2, 5);

    // Check that the number of clusters is as expected (k=2)
    assert_eq!(clusters.len(), 2, "The number of clusters should be 2.");

    // Check that each cluster has at least one member
    for (i, cluster) in clusters.iter().enumerate() {
        assert!(!cluster.members.is_empty(), "Cluster {} should have members.", i + 1);
    }

    // Ensure that centroids are assigned correctly
    for (i, cluster) in clusters.iter().enumerate() {
        assert!(!cluster.centroids.is_empty(), "Cluster {} should have a centroid.", i + 1);
    }
}

}