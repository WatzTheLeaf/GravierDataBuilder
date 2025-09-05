use crate::core::tile::Tile;
use byteorder::{LittleEndian, WriteBytesExt};
use std::fs::File;
use std::io::BufWriter;

const PATH: &str = "../Data/tdata.bin";

pub fn create_data_bin() -> std::io::Result<File> {
    File::create(PATH)
}

pub fn write_all_data(tiles: Vec<Tile>) -> std::io::Result<()> {
    let file = create_data_bin()?;
    let mut writer = BufWriter::new(file);

    for tile in tiles {
        writer.write_i32::<LittleEndian>(tile.posx())?;
        writer.write_i32::<LittleEndian>(tile.posy())?;
        writer.write_f32::<LittleEndian>(tile.u())?;
        writer.write_f32::<LittleEndian>(tile.v())?;
        writer.write_f32::<LittleEndian>(tile.height())?;
    }

    Ok(())
}
