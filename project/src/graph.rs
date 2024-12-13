use std::collections::HashMap;
use rand::Rng;
use csv::ReaderBuilder;

pub type Node = String;
pub type AdjacencyLists = HashMap<Node, Vec<Node>>;

#[derive(Debug)]
pub struct Graph {
    pub outedges: AdjacencyLists,
}

impl Graph {
    pub fn create_directed_graph(edges: Vec<(Node, Node)>) -> Graph {
        let mut graph = Graph {outedges: HashMap::new()};
        for (from, to) in edges {
            graph.outedges.entry(from).or_insert_with(Vec::new).push(to);
        }
        graph
    }
    //if no neighbor, jump to random node in graph, if has neighbor 90% go to a neighboring node, 10% jump to random node in graph
    pub fn random_walk(&self, current: &Node, steps: usize) -> Node {
        let mut current_node = current.clone();
        let mut rng = rand::thread_rng();
        for _ in 0..steps {
            let neighbors = match self.outedges.get(&current_node) {
                Some(neighbors) => neighbors,
                None => &vec![],//handels when there's no neighbor
            };
            if neighbors.is_empty() { //the vertex has no outgoing edges
                let keys: Vec<_> = self.outedges.keys().collect();
                let rand_jump_i = rng.gen_range(0..keys.len());
                current_node = keys[rand_jump_i].clone(); //jump to a random node in the entire graph
            } else { 
                let random_number = rng.gen_range(1..=10);
                if random_number == 1 { // 10% chance for this branch
                    let keys: Vec<_> = self.outedges.keys().collect();
                    let rand_jump_i = rng.gen_range(0..keys.len());
                    current_node = keys[rand_jump_i].clone(); //jump to a random node in the entire graph
                } else {// 90% chance for this branch
                    let random_edge = rng.gen_range(0..neighbors.len());
                    current_node = neighbors[random_edge].clone();//jump to a random node in the list of neighbors
                }
            }
        }
        return current_node;
    }

    pub fn read_graph_from_csv(filename: &str) -> Result<Graph, Box<dyn std::error::Error>> {
        let mut graph: Vec<(Node, Node)> = Vec::new();
        // Open the file (data are separated by \t)
        let mut rdr = ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(true)
            .from_path(filename)?;
        
        // Read through the rows in the CSV
        for (line_number, result) in rdr.records().enumerate() {
            match result {
                Ok(line) => {
                    if line.len() < 2 {
                        println!("Skipping invalid line {}: {:?}", line_number + 1, line);
                        continue;
                    }
                    let from: &str = &line[0];
                    let to: &str = &line[1];
                    // Insert the edge into the adjacency list for the node
                    graph.push((from.to_string(), to.to_string()));
                },
                Err(e) => {
                    println!("Error reading record {}: {}", line_number + 1, e);
                    continue;
                }
            }
        }
        let graph_struct = Graph::create_directed_graph(graph);
        Ok(graph_struct)
    }
} 