//! Definitions and functions for GSSprites

use crate::color::GSColor;
use bitvec::prelude::*;
use log::debug;

pub struct GSSprite {
    data: Vec<GSColor>,
}

impl GSSprite {
    pub(crate) fn from_compression_format0(
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

    pub(crate) fn from_compression_format1(
        width: u8,
        height: u8,
        _scale: u16,
        raw_data: &[u8],
    ) -> Self {
        let mut rgb5_buffer: Vec<u8> = vec![]; //vec![0x00; width as usize * height as usize * 2];

        let mut index: usize = 0;
        'outer: loop {
            let instructions = BitSlice::<Msb0, u8>::from_element(&raw_data[index]);
            index += 1;

            debug!("instructions:{:?}\n", instructions);
            //iterate through instructions beginning with highest bit
            for bit in instructions.iter() {
                debug!("instruction bit:{:?}\n", bit);
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

                    debug!(
                        "byte1:{:?}; byte2:{:?}; readcount:{:?}; offset:{:?}\n",
                        byte1, byte2, readcount, offset
                    );
                    // Break out of complete loop, if both is zero
                    // whatever is in the output buffer at that point is done
                    if readcount == 0 {
                        if offset == 0 {
                            break 'outer;
                        }

                        readcount = 16 + raw_data[index] as u32;
                        index += 1;
                    }

                    for _ in 0..readcount {
                        // Do we actually have to do this readcount+1;? No idea why :)
                        debug!("buffer_len:{:?}; offset:{:?}\n", rgb5_buffer.len(), offset);
                        let out_index = rgb5_buffer.len().saturating_sub(offset as usize); // TODO: Check which behaviour to take and on what type. u16?
                        match rgb5_buffer.get(out_index) {
                            Some(&data) => {
                                rgb5_buffer.push(data);
                            }
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
            .map(|chunk| GSColor::from_rgb5(chunk))
            .collect();

        Self { data }
    }

    pub fn to_rgba_buffer(&self) -> Vec<u8> {
        self.data
            .iter()
            .map(|pixel| pixel.to_rgba().to_vec())
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
    pub(crate) fn new(identifier: String, sprite_width: u8, sprite_height: u8, scale: u16) -> Self {
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
        //TODO: create a "squarish" texture from the sprites
        (0, 0)
    }

    pub(crate) fn push(&mut self, sprite: GSSprite) {
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
