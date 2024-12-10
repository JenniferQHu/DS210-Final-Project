use rand::Rng;
use std::collections::{HashMap, HashSet};

pub type Node = String;

#[derive(Debug)]
pub struct Cluster {
    pub centroids: Node,
    pub members: HashSet<Node>,
}

impl Cluster {
    pub fn kmeans_clustering(adjacency_lists: &HashMap<Node, Vec<Node>>, k: usize, iterations: usize) -> Self {
        let mut rng = rand::thread_rng;
        let nodes: Vec<&Node> = adjacency_lists.keys().collect();
        let mut centroids = Vec::new();
        let mut selected_indices = HashSet::new();

        while centroids.len() < k {
            let random_index = rng.gen_range(0..nodes.len());
            if selected_indices.insert(random_index) { // Ensure uniqueness
                centroids.push(nodes[random_index].clone());
            }
        }

        let mut clusters = vec![HashSet::new(); k];
        for _ in 0..iterations {
            clusters.iter_mut().for_each(|cluster| cluster.clear()); // Clear previous assignments
            for node in adjacency_lists.keys() {
                let nearest_centroid_index = centroids.iter().enumerate()
                    .min_by_key(|(_, centroid)| Self::distance(adjacency_lists, node, centroid))
                    .map(|(index, _)| index)
                    .unwrap();
                clusters[nearest_centroid_index].insert(node.clone());
        }
        //update centroids
            for (i, cluster_nodes) in clusters.iter().enumerate() {
                if !cluster_nodes.is_empty() {
                    centroids[i] = cluster_nodes.iter()
                        .max_by_key(|node| adjacency_lists.get(*node).map(|neighbors| neighbors.len()).unwrap_or(0))
                        .unwrap()
                        .clone();
                }
            }
        }
        Self {clusters, centroids}
    }

    pub fn distance(adjacency_lists: &HashMap<Node, Vec<Node>>, node_1: &Node, node_2: &Node,) -> usize {
        let neighbors_1: HashSet<_> = adjacency_lists.get(node_1).unwrap_or(&vec![]).iter().collect();
        let neighbors_2: HashSet<_> = adjacency_lists.get(node_2).unwrap_or(&vec![]).iter().collect();
        let intersection = neighbors_1.intersection(&neighbors_2).count();
        let union = neighbors_1.union(&neighbors_2).count();
        let distance = union - intersection // Jaccard distance
        return distance;
    }
}
