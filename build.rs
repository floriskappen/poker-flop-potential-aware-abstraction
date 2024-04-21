fn main() {
    prost_build::Config::new()
        .out_dir("src/proto/build")
        .compile_protos(&[
            "src/proto/hand_strength_histograms.proto",
            "src/proto/clustered_data_centroids.proto",
            "src/proto/potential_aware_emd_matrix.proto",
            ], &["src/"])
        .unwrap();
}
