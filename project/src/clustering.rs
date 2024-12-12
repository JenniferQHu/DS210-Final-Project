use std::collections::{HashMap, HashSet};
use clustering::kmeans;

pub type Node = String;

#[derive(Debug)]
pub struct Cluster {
    pub centroids: Node,
    pub members: HashSet<Node>,
}

impl Cluster {
    pub fn kmeans_clustering(
        adjacency_lists: &HashMap<Node, Vec<Node>>, 
        k: usize, 
        iterations: usize
    ) -> Vec<Cluster> {
        //collect all nodes
        let nodes: Vec<&Node> = adjacency_lists.keys().collect();
        //convert adajacency lists into feature vectors
        let mut data: Vec<Vec<f64>> = Vec::new();
        for node in &nodes {
            let empty_vec: Vec<Node> = Vec::new();
            let neighbors = adjacency_lists.get(*node).unwrap_or(&empty_vec);
            let mut neighbor_vector = vec![0.0; nodes.len()];
            for neighbor in neighbors {
                if let Some(index) = nodes.iter().position(|n| *n == neighbor) {
                    neighbor_vector[index] = 1.0; // 1 indicates a connection
                }
            }
            data.push(neighbor_vector);
        }
        //k-means clustering
        let clustering = kmeans(k, &data, iterations);//this return a Clustering object
        let centroids = &clustering.centroids;  // This gives a Vec<Vec<f64>> of centroids
        let members = &clustering.membership; // This gives a Vec<usize> of cluster assignments

        //organize clusters
        let mut clusters: Vec<Cluster> = Vec::new();
        for _ in 0..k {
            clusters.push(Cluster {
                centroids: "".to_string(),
                members: HashSet::new(),
            });
        }

        //assign each node to clusters
        for (i, &cluster_i) in members.iter().enumerate() {
            clusters[cluster_i]
                .members
                .insert(nodes[i].to_string());
        }

        //assign centroid nodes to each cluster
        for (i, centroid) in centroids.iter().enumerate() {
            let closest_index = centroid.0.iter()
                .enumerate()
                .max_by(|(_, &a), (_, &b)| a.partial_cmp(&b).unwrap())
                .map(|(index, _)| index)
                .unwrap_or(0);
            clusters[i].centroids = nodes[closest_index].to_string();
        }

        clusters
    }
}
