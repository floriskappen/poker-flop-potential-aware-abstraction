use crate::earth_movers_distance::earth_movers_distance;

pub fn get_sorted_distances_and_ordered_clusters(
    centroids: &Vec<Vec<f32>>) -> (Vec<Vec<f64>>, Vec<Vec<usize>>) {
    
    let num_clusters = centroids.len();
    let mut sorted_distances = vec![vec![]; num_clusters];
    let mut ordered_clusters = vec![vec![]; num_clusters];

    for i in 0..num_clusters {
        // Collect EMD distances and indices for all clusters
        let mut distances: Vec<(usize, f64)> = (0..num_clusters)
            .map(|j| (j, earth_movers_distance(&centroids[i].iter().map(|&el| el as f64).collect(), &centroids[j].iter().map(|&el| el as f64).collect())))
            .collect();

        // Sort based on distance
        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Unzip sorted distances and indices, ensuring we do not index beyond the available centroids
        let (indices, dists): (Vec<usize>, Vec<f64>) = distances.into_iter().unzip();
        sorted_distances[i] = dists;
        ordered_clusters[i] = indices.iter().filter(|&&index| index < num_clusters).copied().collect();
    }

    (sorted_distances, ordered_clusters)
}