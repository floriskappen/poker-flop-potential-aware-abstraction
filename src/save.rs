
use std::error::Error;
use std::io::{BufWriter, Write};
use std::fs::File;

use prost::Message;

use crate::proto::{EmdMatrix, Row};

pub fn save_potential_aware_emd_matrix(emd_matrix: Vec<Vec<f64>>) -> Result<(), Box<dyn Error>> {
    let filepath = "./data_out/potential_aware_emd_matrix.bin";
    let mut buf_writer = BufWriter::new(File::create(filepath)?);

    let proto_matrix = EmdMatrix {
        rows: emd_matrix.into_iter().map(|row| {
            Row {
                emd_values: row
            }
        }).collect()
    };

    let mut buf = Vec::new();
    proto_matrix.encode(&mut buf)?;

    buf_writer.write_all(&buf)?;

    log::info!("Saved EMD matrix to {}", filepath);

    return Ok(());
}
