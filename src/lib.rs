use std::{fs, io, path::Path};

#[derive(Clone, Copy)]
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
    width: u8,
    height: u8,
    scale: f32,
    data: Vec<GSColor>,
}

impl GSSprite {
    pub fn new(height: u8, width: u8, scale: u16) -> Self {
        let scale = match scale {
            0x0200 => 2.0,
            0x0080 => 0.5,
            _ => 1.0,
        };

        Self {
            width,
            height,
            scale,
            data: vec![GSColor::Transparent; (width * height) as usize],
        }
    }

    pub fn decompress0(&mut self, raw_data: &[u8], palette: &[GSColor; 0xE0]) {
        let mut offset = 0;

        for (i, byte) in raw_data.iter().enumerate() {
            match byte {
                0x00 => break,                                                        //end
                0x01..=0xDF => self.data.insert(i + offset, palette[*byte as usize]), //decompress from palette
                0xE0..=0xFF => offset += *byte as usize, //increase offset to compensate
            }
        }
    }
}

pub struct GSTexture {
    identifier: String,
    sprites: Vec<GSSprite>,
}

impl GSTexture {
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
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
            let r: u8 = (color & 0x05) as u8;
            let g: u8 = ((color >> 5) & 0x05) as u8;
            let b: u8 = ((color >> 10) & 0x05) as u8;
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

    pub fn decompress_sprites(&self) -> Vec<GSTexture> {
        let vec = vec![];

        //TODO:
        //Take the master sprite table slice and go over it in 20Byte chunks
        //at 08300000 to 08680000
        //Take some sort of mapping file to give the textures an identifier
        let start = Self::convert_addr(0x08300000);
        let end = Self::convert_addr(0x08680000);

        for (i, raw_texture) in self.data[start..end].chunks(20).enumerate() {
            let identifier = format!("{:#010X}", i + 0x08000000);
            let texture = GSTexture::new(identifier);
            let sprite_width = raw_texture[0];
            let sprite_height = raw_texture[1];
            let sprite_scale = Self::to_short(raw_texture[3], raw_texture[2]);
            let num_of_dir = raw_texture[4];
            let num_of_ani = raw_texture[5];
            let x_offset = raw_texture[6];
            let y_offset = raw_texture[7];
            let _unknown = raw_texture[8];
            let collsion_radius = raw_texture[9];
            let compression_format = raw_texture[10];
            let _unused = raw_texture[11];
            let sprite_addr = Self::to_word(
                raw_texture[15],
                raw_texture[14],
                raw_texture[13],
                raw_texture[12],
            );
            let ani_addr = Self::to_word(
                raw_texture[19],
                raw_texture[18],
                raw_texture[17],
                raw_texture[16],
            );
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
