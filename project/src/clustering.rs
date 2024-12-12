use std::collections::{HashMap, HashSet};
use clustering::kmeans::{self, KMeans};

pub type Node = String;

#[derive(Debug)]
pub struct Cluster {
    pub centroids: Node,
    pub members: HashSet<Node>,
}

impl Cluster {
    pub fn kmeans_clustering(adjacency_lists: &HashMap<Node, Vec<Node>>, k: usize, iterations: usize) -> Self {
        let nodes: Vec<&Node> = adjacency_lists.keys().collect();
        let mut data: Vec<Vec<f64>> = Vec::new();

        for node in &nodes {
            let neighbors = adjacency_lists.get(*node).unwrap_or(&vec![]);
            let mut neighbor_vector = vec![0.0; nodes.len()];
            for neighbor in neighbors {
                if let Some(index) = nodes.iter().position(|n| n == neighbor) {
                    neighbor_vector[index] = 1.0; // 1 indicates a connection
                }
            }
            data.push(neighbor_vector);
        }

        let (centroids, members) = kmeans(&data, k, iterations).unwrap();

        let mut clusters = vec![
            Cluster {
                centroid: "".to_string(),
                members: HashSet::new(),
            }
        ];

        for (i, node) in nodes.iter().enumerate() {
            let cluster_index = memberships[i];
            clusters[cluster_index]
                .members
                .insert((*node).clone());
        }

        for (i, centroid) in centroids.iter().enumerate() {
            let closest_index = centroid
                .iter()
                .enumerate()
                .max_by_key(|(_, &value)| value.to_bits())
                .map(|(index, _)| index)
                .unwrap_or(0);
            clusters[i].centroid = nodes[closest_index].to_string();
        }

        Self { clusters }
    }
}
