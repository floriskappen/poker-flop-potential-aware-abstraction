# Poker Flop Potential-Aware Abstraction

This project implements the “potential-aware” information abstraction used in the
Pluribus poker AI.  The program clusters poker states by the similarity of their
future potential using Earth Mover’s Distance (EMD).

## Overview

When searching in real time, Pluribus keeps the current round exact but uses a
lossy abstraction for future rounds. Each information situation is assigned to
one of 500 buckets. These buckets are determined for every possible flop (the
first three board cards) using the following approach:

1. **Hand-strength Histograms** – For each flop, the code loads a histogram
   describing how each private hand can evolve on future streets
   (`src/proto/hand_strength_histograms.proto`).

2. **Turn Centroid Clusters** – The second round’s abstraction is represented by
   cluster centroids stored in `src/proto/clustered_data_centroids.proto`.

3. **Earth Mover’s Distance** – `src/earth_movers_distance.rs` computes the EMD
   between two distributions. To accelerate repeated calculations,  
   `src/earth_movers_distance_approximation/precomputation.rs` builds lookup
   tables of distances between centroids.

4. **Approximate EMD** – During matrix computation,  
   `src/earth_movers_distance_approximation/algorithm.rs` uses the precomputed
   ordering to approximate the EMD between a flop histogram and each centroid.

5. **Output** – The resulting matrix of EMD values is saved via the protobuf
   defined in `src/proto/potential_aware_emd_matrix.proto` (see
   `src/save.rs`), which can be used to assign flop situations to one of the
   500 buckets.

## Running

This project expects binary protobuf data in `data_in/`:
- `round_1_hand_strength_histograms.bin`
- `round_2_centroids.bin`

After placing these files, build and run:

```bash
cargo run --release
```