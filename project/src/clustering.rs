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
        let nodes: Vec<&Node> = adjacency_lists.keys().collect();
        // Convert adjacency lists into feature vectors
        let data = Self::convert_to_feature_vectors(adjacency_lists, &nodes);
        // Perform k-means clustering
        let clustering = kmeans(k, &data, iterations);
        let centroids = clustering.centroids.iter().map(|c| &c.0).collect::<Vec<_>>(); // Extract inner Vec<f64>
        let members = clustering.membership;
        // Organize clusters
        let mut clusters: Vec<Cluster> = Vec::new();
        for _ in 0..k {
            clusters.push(Cluster {
                centroids: "".to_string(),
                members: HashSet::new(),
            });
        }

        // Assign nodes to clusters
        for (i, &cluster_i) in members.iter().enumerate() {
            clusters[cluster_i].members.insert(nodes[i].to_string());
        }

        // Assign closest nodes to centroids
        for (i, centroid) in centroids.iter().enumerate() {
            clusters[i].centroids = Self::find_closest_node(centroid, &data, &nodes);
        }

        clusters
    }

    // Converts adjacency lists into feature vectors for clustering
    fn convert_to_feature_vectors(adjacency_lists: &HashMap<Node, Vec<Node>>, nodes: &[&Node]) -> Vec<Vec<f64>> {
        let mut data: Vec<Vec<f64>> = Vec::new();
        for node in nodes {
            let empty_vec: Vec<Node> = Vec::new();
            let neighbors = adjacency_lists.get(*node).unwrap_or(&empty_vec);
            let mut neighbor_vector = vec![0.0; nodes.len()];
            for neighbor in neighbors {
                if let Some(index) = nodes.iter().position(|n| *n == neighbor) {
                    neighbor_vector[index] = 1.0; // Mark connection with 1.0
                }
            }
            data.push(neighbor_vector);
        }
        data
    }
    fn find_closest_node(centroid: &Vec<f64>, data: &[Vec<f64>], nodes: &[&Node]) -> String {
        let closest_index = nodes.iter().enumerate()
            .min_by(|(i_a, _), (i_b, _)| {
                let dist_a = centroid.iter().zip(&data[*i_a])//Pairs each feature of the centroid with the corresponding feature of node a
                    .map(|(c, n)| (c - n).powi(2))//squared distance b/w centroid and current node
                    .sum::<f64>();
                let dist_b = centroid.iter().zip(&data[*i_b])//Pairs each feature of the centroid with the corresponding feature of node b
                    .map(|(c, n)| (c - n).powi(2))//squared distance b/w centroid and current node
                    .sum::<f64>();
                dist_a.partial_cmp(&dist_b).unwrap()//compare which is smaller
            })
            .map(|(index, _)| index)//return result of comparison
            .unwrap_or(0);
        nodes[closest_index].to_string()//return the node name
    }
}