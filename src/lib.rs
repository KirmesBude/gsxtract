use std::{fs, io, path::Path};

pub use texture_packer;

#[derive(Debug, Clone, Copy)]
pub enum GSColor {
    RGB { red: u8, green: u8, blue: u8 },
    Transparent,
}

impl GSColor {
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::RGB { red, green, blue }
    }
}

pub struct GSSprite {
    data: Vec<GSColor>,
}

impl GSSprite {
    pub fn new(width: u8, height: u8, _scale: u16) -> Self {
        Self {
            data: vec![GSColor::Transparent; (width as usize * height as usize)],
        }
    }

    pub fn decompress0(&mut self, raw_data: &[u8], palette: &[GSColor; 0xE0]) {
        let mut offset = 0;

        for (i, byte) in raw_data.iter().enumerate() {
            if i+offset >= self.data.len() {
                break;
            }
            match byte {
                0x00 => break,                                                        //end
                0x01..=0xDF => self.data.insert(i + offset, palette[*byte as usize]), //decompress from palette
                0xE0..=0xFF => offset += (*byte-0xDF) as usize, //increase offset to compensate
            }
        }
    }
}

pub struct GSSpriteAtlas {
    identifier: String,
    sprite_width: u8,
    sprite_height: u8,
    sprite_scale: f32,
    sprites: Vec<GSSprite>,
}

impl GSSpriteAtlas {
    pub fn new(identifier: String, sprite_width: u8, sprite_height: u8, scale: u16) -> Self {
        let sprite_scale: f32 = scale as f32 / 256 as f32;

        Self {
            identifier,
            sprite_width,
            sprite_height,
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

    pub fn sprite_width(&self) -> u8 {
        self.sprite_width
    }

    pub fn sprite_height(&self) -> u8 {
        self.sprite_height
    }

    pub fn sprite_scale(&self) -> f32 {
        self.sprite_scale
    }
}
pub struct GSRom {
    data: Vec<u8>,
    c0palette: [GSColor; 0xE0],
}

impl GSRom {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let data = fs::read(path)?;
        let c0palette = GSRom::init_c0palette(&data);
        Ok(Self { data, c0palette })
    }

    // It is:  at 08017B10 to 08017CBF, encoded as 15 bit rgb with the LE, MSB is ignored
    // loweset 5 bit = RED
    // nest 5 bit = GREEEN
    // next 5 bit = BLUE
    fn init_c0palette(data: &Vec<u8>) -> [GSColor; 0xE0] {
        let mut palette: [GSColor; 224] = [GSColor::Transparent; 0xE0];
        let start = Self::convert_addr(0x08017B10);
        let end = Self::convert_addr(0x08017CD0);
        for (i, short) in data[start..end].chunks(2).enumerate().skip(1) {
            let color: u16 = Self::to_short(short[1], short[0]);

            let r: u8 = ((color & 0x1F)*255/31) as u8;
            let g: u8 = (((color >> 5) & 0x1F)*255/31)  as u8;
            let b: u8 = (((color >> 10) & 0x1F)*255/31)  as u8;
            palette[i] = GSColor::new_rgb(r, g, b);
        }

        palette
    }

    pub fn c0palette(&self) -> &[GSColor; 224] {
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

            let identifier = format!("{:#010X}", i*20 + 0x08000000);
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

                let sprite_data = &self.data[Self::convert_addr(sprite_addr)
                    ..Self::convert_addr(sprite_addr + (sprite_width as usize * sprite_height as usize))];

                let mut sprite = GSSprite::new(sprite_width, sprite_height, sprite_scale);

                match compression_format {
                    0x00 => sprite.decompress0(sprite_data, &self.c0palette),
                    _ => println!("compression format {} found at {:#010X}!", compression_format, i*20 + 0x08000000), //TODO: add other decompression formats
                }

                sprite_atlas.push(sprite);
            }

            vec.push(sprite_atlas);
        }

        vec
    }

    const fn convert_addr(addr: usize) -> usize {
        let offset = 0x08000000;

        addr - offset
    }

    const fn to_short(high: u8, low: u8) -> u16 {
        (high as u16) << 8 | low as u16
    }

    const fn to_word(hh: u8, hl: u8, lh: u8, ll: u8) -> u32 {
        (hh as u32) << 24 | (hl as u32) << 16 | (lh as u32) << 8 | ll as u32
    }
}
