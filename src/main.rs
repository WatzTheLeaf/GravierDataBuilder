use crate::core::terrain::Terrain;
use crate::core::writer::write_all_data;
use std::time::Instant;

mod core;
mod pixpal;

fn main() {
    let now = Instant::now();

    let terrain = Terrain::new(1).init().upscale_n(5).complete_pattern();

    write_all_data(terrain.values_as_tile_vector())
        .expect("Failed to create/replace and fill data file !");

    println!(
        "Tile data was generated in {}s",
        now.elapsed().as_secs_f32()
    );
}
