use bitvec::prelude::*;
use log::{error, info, trace, debug};
use std::{fs, io, path::Path};

#[derive(Clone, Copy)]
pub enum GSColor {
    Transparent,
    RGB5([u8; 3]),
}

impl GSColor {
    pub fn with_rgba_buffer(buffer: &[u8; 4]) -> Self {
        if buffer[3] == 255 {
            //any transparency between 0 and 255 is ignored and assumed to be non-transparent
            GSColor::Transparent
        } else {
            GSColor::RGB5([
                (buffer[0] as u32 * 31 / 255) as u8,
                (buffer[1] as u32 * 31 / 255) as u8,
                (buffer[2] as u32 * 31 / 255) as u8,
            ])
        }
    }

    pub fn with_rgb5_buffer(buffer: &[u8]) -> Self {
        let color: u16 = GSRom::to_short(buffer[1], buffer[0]);

        let r: u8 = (color & 0x1F) as u8;
        let g: u8 = ((color >> 5) & 0x1F) as u8;
        let b: u8 = ((color >> 10) & 0x1F) as u8;
        GSColor::RGB5([r, g, b])
    }

    pub fn to_rgba_buffer(&self) -> Vec<u8> {
        match &self {
            GSColor::RGB5(rgb) => vec![
                (rgb[0] as u32 * 255 / 31) as u8,
                (rgb[1] as u32 * 255 / 31) as u8,
                (rgb[2] as u32 * 255 / 31) as u8,
                255,
            ],
            _ => vec![0, 0, 0, 0],
        }
    }
}

pub struct GSSprite {
    data: Vec<GSColor>,
}

impl GSSprite {
    pub fn decompress0(
        width: u8,
        height: u8,
        _scale: u16,
        raw_data: &[u8],
        palette: &[GSColor; 0xE0],
    ) -> Self {
        let mut data = vec![GSColor::Transparent; width as usize * height as usize];
        let mut offset = 0;

        for (i, byte) in raw_data.iter().enumerate() {
            if i + offset >= width as usize * height as usize {
                break;
            }
            match byte {
                0x00 => break,                                             //end
                0x01..=0xDF => data[i + offset] = palette[*byte as usize], //decompress from palette
                0xE0..=0xFF => offset += (*byte - 0xE0) as usize, //increase offset to compensate
            }
        }

        Self { data }
    }

    pub fn decompress1(width: u8, height: u8, _scale: u16, raw_data: &[u8]) -> Self {
        let mut rgb5_buffer: Vec<u8> = vec![]; //vec![0x00; width as usize * height as usize * 2];

        let mut index: usize = 0;
        'outer: loop {
            let instructions = BitSlice::<Msb0, u8>::from_element(&raw_data[index]);
            index += 1;

            println!("instructions:{:?}\n", instructions);
            //iterate through instructions beginning with highest bit
            for bit in instructions.iter() {
                println!("instruction bit:{:?}\n", bit);            
                if bit == false {
                    // In case of low/false, we take over the next byte
                    rgb5_buffer.push(raw_data[index]);
                    index += 1;

                    // TODO: early break for now
                    if rgb5_buffer.len() == height as usize * width as usize * 2 {
                        break 'outer;
                    }             
                } else {
                    let byte1 = raw_data[index];
                    index += 1;
                    let byte2 = raw_data[index];
                    index += 1;

                    let mut readcount = (byte1 & 0x0F) as u32; // lower 4 bits of first byte is readcount
                    let offset = (((byte1 as u16 & 0xF0) << 4) as u16) | (byte2 as u16); // higher 4 bits of first bytes or'd with the second byte results in a 12 bit offset

                    println!("byte1:{:?}; byte2:{:?}; readcount:{:?}; offset:{:?}\n", byte1, byte2, readcount, offset);
                    // Break out of complete loop, if both is zero
                    // whatever is in the output buffer at that point is done
                    if readcount == 0 {
                        if offset == 0 {
                            break 'outer;
                        }

                        readcount = 16 + raw_data[index] as u32;
                        index += 1;
                    }

                    for _ in 0..readcount { // Do we actually have to do this readcount+1;? No idea why :)
                        println!("buffer_len:{:?}; offset:{:?}\n", rgb5_buffer.len(), offset);
                        let out_index = rgb5_buffer.len().saturating_sub(offset as usize); // TODO: Check which behaviour to take and on what type. u16?
                        match rgb5_buffer
                            .get(out_index)
                        {
                            Some(data) => {
                                let data = data.clone();
                                rgb5_buffer.push(data);
                            },
                            _ => rgb5_buffer.push(0x00), // Assume default data of 00 in case it is outside the range
                        };

                        // TODO: early break for now
                        if rgb5_buffer.len() == height as usize * width as usize * 2 {
                            break 'outer;
                        }                       
                    }
                }
            }
        }

        // TODO: I make sure to fill it up with shit to get to the correct dimensions
        while rgb5_buffer.len() < height as usize * width as usize * 2 {
            rgb5_buffer.push(0x00);
        } 
        
        let data: Vec<GSColor> = rgb5_buffer
            .as_slice()
            .chunks_exact(2)
            .map(|chunk| GSColor::with_rgb5_buffer(chunk))
            .collect();

        Self { data }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get_buffer(&self) -> Vec<u8> {
        self.data
            .iter()
            .map(|pixel| pixel.to_rgba_buffer())
            .flatten()
            .collect()
    }
}

pub struct GSSpriteAtlas {
    identifier: String,
    sprite_width: u32,
    sprite_height: u32,
    sprite_scale: f32,
    sprites: Vec<GSSprite>,
}

impl GSSpriteAtlas {
    pub fn new(identifier: String, sprite_width: u8, sprite_height: u8, scale: u16) -> Self {
        let sprite_scale: f32 = scale as f32 / 256_f32;

        Self {
            identifier,
            sprite_width: sprite_width.into(),
            sprite_height: sprite_height.into(),
            sprite_scale,
            sprites: vec![],
        }
    }

    pub fn size(&self) -> (u32, u32) {
        //TODO: create a square texture from the sprites
        (0, 0)
    }

    pub fn push(&mut self, sprite: GSSprite) {
        self.sprites.push(sprite);
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn sprite_width(&self) -> u32 {
        self.sprite_width
    }

    pub fn sprite_height(&self) -> u32 {
        self.sprite_height
    }

    pub fn sprite_scale(&self) -> f32 {
        self.sprite_scale
    }

    pub fn get_sprites(&self) -> &Vec<GSSprite> {
        &self.sprites
    }
}
pub struct GSRom {
    data: Vec<u8>,
    c0palette: [GSColor; 0xE0],
}

impl GSRom {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        trace!("Got ROM");
        let data = fs::read(path)?;
        let c0palette = GSRom::init_c0palette(&data);
        Ok(Self { data, c0palette })
    }

    // It is:  at 08017B10 to 08017CBF, encoded as 15 bit rgb in LE, MSB is ignored
    // loweset 5 bit = RED
    // nest 5 bit = GREEEN
    // next 5 bit = BLUE
    fn init_c0palette(data: &[u8]) -> [GSColor; 0xE0] {
        let mut palette: [GSColor; 0xE0] = [GSColor::Transparent; 0xE0];
        let start = Self::convert_addr(0x08017B10);
        let end = Self::convert_addr(0x08017CD0);
        for (i, short) in data[start..end].chunks_exact(2).enumerate().skip(1) {
            palette[i] = GSColor::with_rgb5_buffer(short);
        }

        palette
    }

    pub fn c0palette(&self) -> &[GSColor; 0xE0] {
        &self.c0palette
    }

    pub fn get(&self, addr: usize) -> Option<&u8> {
        self.data.get(Self::convert_addr(addr))
    }

    pub fn decompress_sprites(&self) -> Vec<GSSpriteAtlas> {
        let mut vec = vec![];

        //TODO:
        //Take the master sprite table slice and go over it in 20Byte chunks
        //at 08300000 to 08680000
        //Take some sort of mapping file to give the textures an identifier
        let start = Self::convert_addr(0x08300000);
        let end = Self::convert_addr(0x08302918);

        for (i, raw_sprite_atlas) in self.data[start..end].chunks(20).enumerate() {
            let sprite_width = raw_sprite_atlas[0];
            let sprite_height = raw_sprite_atlas[1];
            let sprite_scale = Self::to_short(raw_sprite_atlas[3], raw_sprite_atlas[2]);
            let num_of_dir = raw_sprite_atlas[4];
            let _num_of_ani = raw_sprite_atlas[5];
            let _x_offset = raw_sprite_atlas[6];
            let _y_offset = raw_sprite_atlas[7];
            let _unknown = raw_sprite_atlas[8];
            let _collsion_radius = raw_sprite_atlas[9];
            let compression_format = raw_sprite_atlas[10];
            let _unused = raw_sprite_atlas[11];
            let sprites_addr: usize = Self::to_word(
                raw_sprite_atlas[15],
                raw_sprite_atlas[14],
                raw_sprite_atlas[13],
                raw_sprite_atlas[12],
            ) as usize;
            let _anis_addr: usize = Self::to_word(
                raw_sprite_atlas[19],
                raw_sprite_atlas[18],
                raw_sprite_atlas[17],
                raw_sprite_atlas[16],
            ) as usize;
            
            let identifier = format!("{:#010X}", i * 20 + 0x08300000);

            println!("{}: {}x{} at {:#010X} with {} or {} addrs", identifier, sprite_width, sprite_height, sprites_addr, num_of_dir, _num_of_ani);

            let mut sprite_atlas =
                GSSpriteAtlas::new(identifier, sprite_width, sprite_height, sprite_scale);

            for sprite_addr_bytes in self.data[Self::convert_addr(sprites_addr)
                ..Self::convert_addr(sprites_addr + 4 * (num_of_dir as usize))]
                .chunks(4)
            {
                let sprite_addr = Self::to_word(
                    sprite_addr_bytes[3],
                    sprite_addr_bytes[2],
                    sprite_addr_bytes[1],
                    sprite_addr_bytes[0],
                ) as usize;

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

        vec
    }

    const fn convert_addr(addr: usize) -> usize {
        let offset = 0x08000000;

        addr - offset
    }

    pub const fn to_short(high: u8, low: u8) -> u16 {
        (high as u16) << 8 | low as u16
    }

    const fn to_word(hh: u8, hl: u8, lh: u8, ll: u8) -> u32 {
        (hh as u32) << 24 | (hl as u32) << 16 | (lh as u32) << 8 | ll as u32
    }
}
