use crate::core::terrain::Terrain;
use crate::core::writer::write_all_data;
use std::time::Instant;

mod core;
mod pixpal;

fn main() {
    let now = Instant::now();

    let mut terrain = Terrain::new(2);
    terrain.init();
    terrain.upscale_n(5);
    terrain.evaluate_height((terrain.gsize / 2, terrain.gsize / 2));
    terrain.complete_pattern();

    write_all_data(terrain.values_as_tile_vector())
        .expect("Failed to create/replace and fill data file !");

    println!(
        "Tile data was generated in {}s",
        now.elapsed().as_secs_f32()
    );
}
