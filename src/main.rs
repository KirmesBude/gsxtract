mod lib;

use lib::*;
use std::io;

pub fn main() -> Result<(), io::Error> {
    let rom = GSRom::new("Golden Sun - The Lost Age.gba")?;
    let test = rom.decompress_sprites();
    Ok(())
}