use std::time::Instant;
use crate::core::writer::write_all_data;
use crate::generation::terrain::Terrain;

mod core;
mod generation;

fn main() {

    let now = Instant::now();

    let terrain = Terrain::new(500)
        .dla(10000)
        .complete_pattern();

    write_all_data(terrain.values_as_tile_vector())
    .expect("Failed to create/replace and fill data file !");

    println!("Tile data was generated in {}s", now.elapsed().as_secs());
}
