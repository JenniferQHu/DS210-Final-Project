use std::fs::File;
use std::io::{BufRead};
use rand::Rng;


pub type Vertex = usize;
pub type ListOfEdges = Vec<(Vertex,Vertex)>;
pub type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
pub struct Graph {
    pub n: usize,
    pub outedges: AdjacencyLists,
}

impl Graph {
    pub fn add_directed_edges(&mut self, edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    pub fn create_directed(n:usize, edges:&ListOfEdges) -> Graph {
        let mut g = Graph{n, outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g //return total number of verticies, the adjacency list                                        
    }
    
    pub fn random_walk(&self, current: Vertex, steps: usize) -> Vertex {
        let mut current_v = current;
        for _ in 0..steps {
            let mut rng = rand::thread_rng();
            if self.outedges[current_v].is_empty() { //the vertex has no outgoing edges
                current_v = rng.gen_range(0..self.n); //jump to a random vertex in the entire graph
            } else { 
                let random_number = rng.gen_range(1..=10);
                if random_number == 1 { // 10% chance for this branch
                    current_v = rng.gen_range(0..self.n); //jump to a random vertex in the entire graph
                } else {// 90% chance for this branch
                    let random_edge = rng.gen_range(0..self.outedges[current_v].len());
                    current_v = self.outedges[current_v][random_edge];//jump to a random vertex in the list of outgoing edges
                }
            }
        }
        return current_v;
    }
    
}

pub fn read_data(filename: &str) -> Graph {
    let file = File::open(filename).expect("Could not open file");
    let mut buf_reader = std::io::BufReader::new(file).lines();
    // Parse the first line to get `n`
    let n: usize = buf_reader
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut edges = Vec::new();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let vertices: Vec<usize> = line_str
            .split_whitespace()
            .map(|x| x.parse::<usize>().expect("Expected a number"))
            .collect();
        if vertices.len() == 2 {
            edges.push((vertices[0], vertices[1]));
        }
    }
    
    // Create the graph and return it
    let graph = Graph::create_directed(n, &edges);
    return graph;