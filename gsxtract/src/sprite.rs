//! Definitions and functions for GSSprites

use crate::{color::GSColor, comp::from_standard_lz};

pub struct GSSprite {
    data: Vec<GSColor>,
}

impl GSSprite {
    pub fn from_compression_format0(
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
                0x00 => continue,                                          //end
                0x01..=0xDF => data[i + offset] = palette[*byte as usize], //decompress from palette
                0xE0..=0xFF => offset += (*byte - 0xE0) as usize, //increase offset to compensate
            }
        }

        Self { data }
    }

    pub fn from_compression_format1(
        width: u8,
        height: u8,
        _scale: u16,
        raw_data: &[u8],
        palette: &[GSColor; 0xE0],
    ) -> Self {
        let decoded = from_standard_lz(raw_data);

        Self::from_compression_format0(width, height, _scale, &decoded, palette)
    }

    pub fn to_rgba_buffer(&self) -> Vec<u8> {
        self.data
            .iter()
            .flat_map(|pixel| pixel.to_rgba().to_vec())
            .collect()
    }

    pub fn size(&self) -> usize {
        self.data.len()
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
