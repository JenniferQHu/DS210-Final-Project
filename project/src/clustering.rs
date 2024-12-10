use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Clusters {
    pub centroid: Vec<f64>,
    pub nodes: Vec<Vec<String>>,
}

pub fn kmeans_clustering(data: &HashMap<String, Vec<f64>>, k: usize, iterations: usize) -> Clusters {
    let mut rng = rand::thread_rng;
    let keys: Vec<&String> = data.keys().collect();
    let mut centroids: Vec<Vec<f64>> = Vec::with_capacity(k);
    for _ in 0..iterations {
        let mut new_clusters = vec![];
        for (key, values) in data {
            let closet_centroid = 
        }
    }
}