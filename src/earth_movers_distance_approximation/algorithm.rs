
pub fn approximate_emd(
    point: &[u8],
    centroid: &[f64],
    sorted_distances: &Vec<Vec<f64>>,
    ordered_clusters: &Vec<Vec<usize>>
) -> f64 {
    let point: Vec<f64> = point.iter().map(|&el| el as f64).collect();
    let centroid: Vec<f64> = centroid.iter().map(|&el| el as f64).collect();
    let n = point.len();
    let q = centroid.len();
    let mut targets = vec![1.0 / n as f64; n];
    let mut mean_remaining = centroid.to_vec();
    let mut done = vec![false; n];
    let mut tot_cost = 0.0;

    for i in 0..q {
        for j in 0..n {
            if done[j] {
                continue;
            }
            let point_cluster = (point[j] * (n as f64)) as usize;
            if point_cluster >= ordered_clusters.len() {
                continue;
            }
            let mean_cluster = ordered_clusters[point_cluster][i];
            if mean_cluster >= q {
                continue;
            }
            let amt_remaining = mean_remaining[mean_cluster];
            if amt_remaining == 0.0 {
                continue;
            }
            let d = sorted_distances[point_cluster][i];
            if amt_remaining < targets[j] {
                tot_cost += amt_remaining * d;
                targets[j] -= amt_remaining;
                mean_remaining[mean_cluster] = 0.0;
            } else {
                tot_cost += targets[j] * d;
                mean_remaining[mean_cluster] -= targets[j];
                targets[j] = 0.0;
                done[j] = true;
            }
        }
    }
    tot_cost
}
