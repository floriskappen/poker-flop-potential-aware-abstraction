use std::error::Error;
use std::io::{BufReader, Read};
use std::fs::File;

use prost::Message;

use crate::proto::{ClusteredDataCentroids, HandStrengthHistograms};


pub fn load_turn_round_centroids() -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
    let filepath = "./data_in/round_2_centroids.bin";
    let file = File::open(filepath)?;
    let mut buf_reader = BufReader::new(file);
    let mut buf = Vec::new();
    buf_reader.read_to_end(&mut buf)?;

    let hand_strength_histograms = ClusteredDataCentroids::decode(&*buf)?;
    log::info!("Loaded data from {}; len() = {}", filepath, hand_strength_histograms.data.len());

    let data = hand_strength_histograms.data.into_iter()
        .map(|float_list| float_list.values)
        .collect();

    Ok(data)
}

pub fn load_flop_hand_strength_histograms() -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    let filepath = "./data_in/round_1_hand_strength_histograms.bin";
    let mut buf_reader = BufReader::new(File::open(filepath)?);
    let mut buf = Vec::new();
    buf_reader.read_to_end(&mut buf)?;

    let hand_strength_histograms = HandStrengthHistograms::decode(&*buf)?;
    // Convert the protobuf data into the expected format if necessary

    log::info!("Loaded data from {}; len() = {}", filepath, hand_strength_histograms.data.len());
    return Ok(hand_strength_histograms.data);
}
