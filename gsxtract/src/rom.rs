//! GS Rom

use crate::color::GSColor;
use crate::sprite::{GSSprite, GSSpriteAtlas};
use crate::util;

use log::{debug, error, trace};
use std::fmt;
use std::{fs, io, path::Path};

enum GSRomTitle {
    TheBrokenSeal,
    TheLostAge,
}

impl fmt::Display for GSRomTitle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Self::TheBrokenSeal => "GS1",
            Self::TheLostAge => "GS2",
        };
        write!(f, "{}", name)
    }
}

impl GSRomTitle {
    pub(crate) fn c0palette_addr(&self) -> (usize, usize) {
        match self {
            GSRomTitle::TheBrokenSeal => {
                let start = 0x0800779C;
                let end = 0x0800795C;

                (start, end)
            }
            GSRomTitle::TheLostAge => {
                let start = 0x08017B10;
                let end = 0x08017CD0;

                (start, end)
            }
        }
    }

    pub(crate) fn sprite_table_addr(&self) -> (usize, usize) {
        match self {
            GSRomTitle::TheBrokenSeal => {
                let start = 0x08185024;
                let end = 0x08187824;

                (start, end)
            }
            GSRomTitle::TheLostAge => {
                let start = 0x08300000;
                let end = 0x083037F0;

                (start, end)
            }
        }
    }

    pub(crate) fn is_addr_valid(&self, addr: usize) -> bool {
        let min_addr = 0x08000000;
        let max_addr = match self {
            GSRomTitle::TheBrokenSeal => 0x087FFFFF,
            GSRomTitle::TheLostAge => 0x08FFFFFF,
        };

        addr >= min_addr && addr <= max_addr
    }
}

pub struct GSRom {
    data: Vec<u8>,
    c0palette: [GSColor; 0xE0],
    title: GSRomTitle,
}

impl GSRom {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        trace!("Got ROM");
        let data = fs::read(path)?;
        let title = {
            let gamecode = String::from_utf8(data[0xAC..=0xAF].to_vec()).unwrap(); /* The 4 Bytes at that location are the game code */
            // Extrac function for this
            match gamecode.as_str() {
                "AGSE" => GSRomTitle::TheBrokenSeal,
                "AGFE" => GSRomTitle::TheLostAge,
                _ => panic!(), //TODO: handle gracefully
            }
        };
        let c0palette = GSRom::init_c0palette(&data, &title);

        Ok(Self {
            data,
            c0palette,
            title,
        })
    }

    // It is: encoded as 15 bit rgb in LE, MSB is ignored
    // loweset 5 bit = RED
    // next 5 bit = GREEN
    // next 5 bit = BLUE
    fn init_c0palette(data: &[u8], title: &GSRomTitle) -> [GSColor; 0xE0] {
        let mut palette: [GSColor; 0xE0] = [GSColor::Transparent; 0xE0];
        let (start, end) = title.c0palette_addr();
        let (start, end) = (Self::convert_addr(start), Self::convert_addr(end));

        for (i, short) in data[start..end].chunks_exact(2).enumerate().skip(1) {
            palette[i] = GSColor::from_rgb5(short.try_into().unwrap());
        }

        palette
    }

    pub fn c0palette(&self) -> &[GSColor; 0xE0] {
        &self.c0palette
    }

    pub fn get(&self, addr: usize) -> Option<&u8> {
        self.data.get(Self::convert_addr(addr))
    }

    pub fn sprite_table(&self) -> &[u8] {
        let (start, end) = self.title.sprite_table_addr();
        let (start, end) = (Self::convert_addr(start), Self::convert_addr(end));

        &self.data[start..end]
    }

    pub fn decompress_sprites(&self) -> Vec<GSSpriteAtlas> {
        let mut vec = vec![];

        //Take the master sprite table slice and go over it in 20Byte chunks
        //TOOD: Take some sort of mapping file to give the textures an identifier
        for (i, raw_sprite_atlas) in self.sprite_table().chunks(20).enumerate() {
            /* Map bytes */
            let sprite_width = raw_sprite_atlas[0];
            let sprite_height = raw_sprite_atlas[1];
            let sprite_scale = util::as_u16(&raw_sprite_atlas[2..=3].try_into().unwrap());
            let num_of_dir = raw_sprite_atlas[4];
            let _num_of_ani = raw_sprite_atlas[5];
            let _x_offset = raw_sprite_atlas[6];
            let _y_offset = raw_sprite_atlas[7];
            let _unknown = raw_sprite_atlas[8];
            let _collsion_radius = raw_sprite_atlas[9];
            let compression_format = raw_sprite_atlas[10];
            let _unused = raw_sprite_atlas[11];
            let sprites_addr: usize =
                util::as_u32(&raw_sprite_atlas[12..=15].try_into().unwrap()) as usize;
            let _anis_addr: usize =
                util::as_u32(&raw_sprite_atlas[16..=19].try_into().unwrap()) as usize;

            let identifier = format!(
                "{}_{:#010X}_{}",
                self.title,
                i * 20 + self.title.sprite_table_addr().0,
                compression_format,
            );

            debug!(
                "{}: {}x{} at {:#010X} with {} addrs",
                identifier, sprite_width, sprite_height, sprites_addr, num_of_dir
            );

            /* Especially for GS1 there are a lot of dummy/padding entries in the sprite table */
            /* They are all zero, but for our purpose it is enough to check width and height */
            if sprite_width == 0 && sprite_height == 0 {
                continue;
            }

            // TODO: duplicated code
            if self.title.is_addr_valid(sprites_addr) {
                let dir_identifier = format!("{}_dir", identifier);
                let mut sprite_atlas =
                    GSSpriteAtlas::new(dir_identifier, sprite_width, sprite_height, sprite_scale);

                for sprite_addr_bytes in self.data[Self::convert_addr(sprites_addr)
                    ..Self::convert_addr(sprites_addr + 4 * (num_of_dir as usize))]
                    .chunks(4)
                {
                    let sprite_addr =
                        util::as_u32(&sprite_addr_bytes[0..=3].try_into().unwrap()) as usize;

                    /* TODO: why is this necessary for GS1??? */
                    if !self.title.is_addr_valid(sprite_addr) {
                        continue;
                    }
                    debug!("Final addr: {:#010X}", sprite_addr);

                    // I do not want to pass a pointer, so I will just pass a slice of maximum length.
                    // For uncompressed images in RGB5, we would have 2 Bytes per pixel
                    // TODO: Does not seem to be enough?
                    let sprite_data = &self.data[Self::convert_addr(sprite_addr)
                        ..Self::convert_addr(
                            sprite_addr
                                + (sprite_width as usize * sprite_height as usize * 2_usize),
                        )];

                    match compression_format {
                        0x00 => {
                            debug!(
                                "compression format {} found at {:#010X}!",
                                compression_format,
                                i * 20 + 0x08300000
                            );
                            let sprite = GSSprite::from_compression_format0(
                                sprite_width,
                                sprite_height,
                                sprite_scale,
                                sprite_data,
                                &self.c0palette,
                            );
                            sprite_atlas.push(sprite);
                        }
                        0x01 => {
                            debug!(
                                "compression format {} found at {:#010X}!",
                                compression_format,
                                i * 20 + 0x08300000
                            );
                            let sprite = GSSprite::from_compression_format1(
                                sprite_width,
                                sprite_height,
                                sprite_scale,
                                sprite_data,
                                &self.c0palette,
                            );
                            sprite_atlas.push(sprite);
                        }
                        _ => error!(
                            "unsupported compression format {} found at {:#010X}!",
                            compression_format,
                            i * 20 + 0x08300000
                        ), //TODO: add other decompression formats
                    }

                    //sprite_atlas.push(sprite);
                }

                vec.push(sprite_atlas);
            }

            /* TODO: anims are not what I think they are */
            /*
            if self.is_addr_valid(anis_addr) {
                let anim_identifier = format!("{}_anim", identifier);
                let mut sprite_atlas =
                GSSpriteAtlas::new(anim_identifier, sprite_width, sprite_height, sprite_scale);

                for sprite_addr_bytes in self.data[Self::convert_addr(anis_addr)
                    ..Self::convert_addr(anis_addr + 4 * (num_of_ani as usize))]
                    .chunks(4)
                {
                    let sprite_addr = Self::to_word(
                        sprite_addr_bytes[3],
                        sprite_addr_bytes[2],
                        sprite_addr_bytes[1],
                        sprite_addr_bytes[0],
                    ) as usize;

                    /* TODO: why is this necessary for GS1??? */
                    if !self.is_addr_valid(sprite_addr) {
                        continue
                    }
                    println!("Final addr: {:#010X}", sprite_addr);

                    // I do not want to pass a pointer, so I will just pass a slice of maximum length.
                    // For uncompressed images in RGB5, we would have 2 Bytes per pixel
                    // TODO: Does not seem to be enough?
                    let sprite_data = &self.data[Self::convert_addr(sprite_addr)
                        ..Self::convert_addr(
                            sprite_addr + (sprite_width as usize * sprite_height as usize * 2 as usize),
                        )];

                    match compression_format {
                        0x00 => {
                            println!(
                                "compression format {} found at {:#010X}!",
                                compression_format,
                                i * 20 + 0x08300000
                            );
                            let sprite = GSSprite::decompress0(
                                sprite_width,
                                sprite_height,
                                sprite_scale,
                                sprite_data,
                                &self.c0palette,
                            );
                            sprite_atlas.push(sprite);
                        }
                        0x01 => {
                            println!(
                                "compression format {} found at {:#010X}!",
                                compression_format,
                                i * 20 + 0x08300000
                            );
                            let sprite = GSSprite::decompress1(
                                sprite_width,
                                sprite_height,
                                sprite_scale,
                                sprite_data,
                            );
                            sprite_atlas.push(sprite);
                        }
                        _ => error!(
                            "unsupported compression format {} found at {:#010X}!",
                            compression_format,
                            i * 20 + 0x08000000
                        ), //TODO: add other decompression formats
                    }

                    //sprite_atlas.push(sprite);
                }

                vec.push(sprite_atlas);
            }
            */
        }

        vec
    }

    const fn convert_addr(addr: usize) -> usize {
        let offset = 0x08000000;

        addr - offset
    }
}
