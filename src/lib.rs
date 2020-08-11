use std::{fs, io, path::Path};

#[derive(Clone, Copy)]
pub enum Color {
    RGB { red: u8, green: u8, blue: u8 },
    Transparent,
}

impl Color {
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::RGB { red, green, blue }
    }
}

pub struct Sprite {
    height: u8,
    width: u8,
    data: Vec<u8>,
}

impl Sprite {
    pub fn new(height: u8, width: u8, data: &[u8]) -> Self {
        Self {
            height,
            width,
            data: data.to_vec(), //decrompress data
        }
    }
}

pub struct GSRom {
    data: Vec<u8>,
    c0palette: [Color; 0xE0],
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
    fn init_c0palette(data: &Vec<u8>) -> [Color; 0xE0] {
        let mut palette: [Color; 224] = [Color::Transparent; 0xE0];
        for (i, short) in data[0x00017B10..=0x00017CBF].chunks(2).enumerate().skip(1) {
            let color: u16 = (short[1] as u16) << 8 | short[0] as u16;
            let r: u8 = (color & 0x05) as u8;
            let g: u8 = ((color >> 5) & 0x05) as u8;
            let b: u8 = ((color >> 10) & 0x05) as u8;
            palette[i] = Color::new_rgb(r, g, b);
        }

        palette
    }

    pub fn c0palette(&self) -> &[Color; 224] {
        &self.c0palette
    }

    pub fn get(&self, offset: usize) -> Option<&u8> {
        let offset = offset - 0x08000000;

        self.data.get(offset)
    }
}

pub fn decompress0(data: &[u8], width: u8, height: u8, palette: &[Color; 0xE0]) -> Vec<Color> {
    let mut sprite_data: Vec<Color> = vec![Color::Transparent; (width * height) as usize];
    let mut offset = 0;

    for (i, byte) in data.iter().enumerate() {
        match byte {
            0x00 => break,                                                          //end
            0x01..=0xDF => sprite_data.insert(i + offset, palette[*byte as usize]), //decompress from palette
            0xE0..=0xFF => offset += *byte as usize, //increase offset to compensate
        }
    }

    sprite_data
}
