mod proto {
    include!("proto/build/_.rs");
}
mod load;
mod save;
mod earth_movers_distance;
mod earth_movers_distance_approximation {
    pub mod precomputation;
    pub mod algorithm;
}

use rayon::prelude::*;

use load::{load_flop_hand_strength_histograms, load_turn_round_centroids};
use save::save_potential_aware_emd_matrix;
use crate::earth_movers_distance_approximation::{algorithm::approximate_emd, precomputation::get_sorted_distances_and_ordered_clusters};


fn main() {
    let flop_hand_strength_histograms = load_flop_hand_strength_histograms().expect("Failed to load flop_hand_strength_histograms");
    let turn_centroids = load_turn_round_centroids().expect("Failed to load turn_centroids");

    let (sorted_distances, ordered_clusters) = get_sorted_distances_and_ordered_clusters(&turn_centroids);

    let emd_matrix: Vec<Vec<f64>> = flop_hand_strength_histograms
        .par_iter()
        .map(|histogram| {
            turn_centroids.iter()
                .map(|centroid| {
                    approximate_emd(histogram, centroid, &sorted_distances, &ordered_clusters)
                })
                .collect()
        })
        .collect();

    log::info!("Computed EMD matrix");
    save_potential_aware_emd_matrix(emd_matrix).expect("Failed to save EMD matrix");
}
