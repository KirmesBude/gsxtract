use gsxtract::*;
use std::{
    fs::{self, File},
    io,
};
use texture_packer::{texture::Texture, exporter::ImageExporter};

pub fn main() -> Result<(), io::Error> {
    fs::create_dir_all("output").unwrap();

    let rom = GSRom::new("references/Golden Sun - The Lost Age.gba")?;
    let test = rom.decompress_sprites();

    let packer_conf = texture_packer::TexturePackerConfig {
        max_width: 1024,
        max_height: 1024,
        allow_rotation: false,
        border_padding: 2,
        texture_padding: 2,
        trim: false,
        texture_outlines: true,
    };

    for atlas in test {
        let mut packer = texture_packer::TexturePacker::new_skyline(packer_conf);
        let name = format!("{}.png", atlas.identifier());
        let path = format!("output/{}", name);

        for (i, sprite) in atlas.get_sprites().iter().enumerate() {
            let buffer = sprite.get_buffer();
            let image_buffer =
                image::ImageBuffer::from_raw(atlas.sprite_width(), atlas.sprite_height(), buffer)
                    .unwrap();
            let texture = image::DynamicImage::ImageRgba8(image_buffer);
            packer.pack_own(i.to_string(), texture).unwrap();
        }

        if packer.width() == 0 {
            continue;
        }
        let exporter = ImageExporter::export(&packer).unwrap();
        let mut file = File::create(path).unwrap();
        exporter
            .write_to(&mut file, image::ImageFormat::Png)
            .unwrap();
    }
    Ok(())
}
