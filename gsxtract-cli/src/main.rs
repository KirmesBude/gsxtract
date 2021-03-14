use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use gsxtract::*;
use simplelog::{Config, LevelFilter, SimpleLogger};
use std::{fs::File, io};
use texture_packer::{exporter::ImageExporter, texture::Texture};

pub fn main() -> Result<(), io::Error> {
    /* COMMAND ARGUMENTS */
    /* basic app information */
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!());

    /* define the path command line option */
    let path_option = Arg::with_name("path")
        .last(true)
        .takes_value(true)
        .help("Path to a Golden Sun ROM")
        .required(true);
    let app = app.arg(path_option);

    /* define the output command line option */
    let output_option = Arg::with_name("output")
        .long("output")
        .short("o")
        .takes_value(true)
        .help("Output path")
        .required(true);
    let app = app.arg(output_option);

    /* define the verbosity command line option */
    let verbosity_option = Arg::with_name("verbosity")
        .short("v")
        .multiple(true)
        .help("Sets the level of verbosity")
        .required(false);
    let app = app.arg(verbosity_option);

    /* define the quiet command line option */
    let quiet_option = Arg::with_name("quiet")
        .short("q")
        .help("Silence all output")
        .required(false);
    let app = app.arg(quiet_option);

    /* extract matches */
    let matches = app.get_matches();

    /* extract the actual output */
    let output = matches
        .value_of("output")
        .expect("This can't be None, we said it was required");

    /* extract the actual path */
    let path = matches
        .value_of("path")
        .expect("This can't be None, we said it was required");

    /* extract the actual quiet */
    let quiet = matches.is_present("quiet");

    /* extract the actual verbosity */
    let verbosity = if quiet {
        LevelFilter::Off
    } else {
        match matches.occurrences_of("v") {
            0 => LevelFilter::Error,
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    };

    /* LOGGING */
    SimpleLogger::init(verbosity, Config::default()).unwrap();

    let rom = GSRom::new(path)?;

    extract_sprites(&rom, output);

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
    };

    for atlas in sprite_atlases {
        let mut packer = texture_packer::TexturePacker::new_skyline(packer_conf);
        let name = format!("{}.png", atlas.identifier());
        let path = format!("{}/{}", output, name);

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
}
