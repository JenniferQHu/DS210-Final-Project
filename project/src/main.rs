use std::collections::{HashMap};
use std::error::Error;
use csv::ReaderBuilder;

fn read_graph_from_csv(file_path: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    // Initialize a HashMap to store the graph as an adjacency list
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    
    // Open the file (data are separated by \t)
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_path(file_path)?;
    
    // Read through the rows in the CSV
    for result in rdr.records() {
        match result {
            Ok(line) => {
                if line.len() < 2 {
                    println!("Skipping invalid line: {:?}", line);
                    continue;
                }
                let node: &str = &line[0];
                let edge: &str = &line[1];

                // Insert the edge into the adjacency list for the node
                graph.entry(node.to_string())
                    .or_insert_with(Vec::new)
                    .push(edge.to_string());
            },
            Err(e) => {
                println!("Error reading record: {}", e);
                continue;
            }
        }
    }
    
    Ok(graph)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "citations.csv"; // Path to your CSV file
    
    // Read the graph from the CSV file
    let graph = read_graph_from_csv(file_path)?;
    
    // Print the adjacency list (graph) to check the structure
    for (node, edges) in &graph {
        println!("Node {} has edges: {:?}", node, edges);
    }
    Ok(())
}
