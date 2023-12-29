use clap::Parser;
use gsxtract::rom::GSRom;
use simplelog::{Config, LevelFilter, SimpleLogger};
use std::{fs::File, io};
use texture_packer::{exporter::ImageExporter, texture::Texture};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    output: String,
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[arg(short, long)]
    quiet: bool,
}

pub fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();

    /* extract the actual verbosity */
    let verbosity = if cli.quiet {
        LevelFilter::Off
    } else {
        match cli.verbose {
            0 => LevelFilter::Error,
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    };

    /* LOGGING */
    SimpleLogger::init(verbosity, Config::default()).unwrap();

    let rom = GSRom::new(&cli.path)?;

    extract_sprites(&rom, &cli.output);

    Ok(())
}

fn extract_sprites(rom: &GSRom, output: &str) {
    let sprite_atlases = rom.decompress_sprites();

    let packer_conf = texture_packer::TexturePackerConfig {
        max_width: 1024,
        max_height: 1024,
        allow_rotation: false,
        border_padding: 2,
        texture_padding: 2,
        trim: false,
        texture_outlines: true,
        ..Default::default()
    };

    for atlas in sprite_atlases {
        let mut packer = texture_packer::TexturePacker::new_skyline(packer_conf);
        let name = format!("{}.png", atlas.identifier());
        let path = format!("{}/{}", output, name);

        for (i, sprite) in atlas.get_sprites().iter().enumerate() {
            let buffer = sprite.to_rgba_buffer();
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
}
