//! Defines Golden Suns default ColorType

use crate::util;

/// Golden Sun ColorTypes.
///
/// Golden Sun internally represented by 2 bytes.
/// MSB is ignored.
/// From low to high, every 5 bits correspong to red, blue, green.
/// In some cases transparent (TODO).
#[derive(Clone, Copy)]
pub enum GSColor {
    Transparent,
    RGB5([u8; 3]),
}

impl GSColor {
    /// Constructs GSColor from a rgba byte stream
    pub fn from_rgba(buffer: &[u8; 3]) -> Self {
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

    /// Constructs GSColor from a rgb555 byte stream
    pub fn from_rgb5(buffer: &[u8; 2]) -> Self {
        let color: u16 = util::as_u16(buffer);

        let r: u8 = (color & 0x1F) as u8;
        let g: u8 = ((color >> 5) & 0x1F) as u8;
        let b: u8 = ((color >> 10) & 0x1F) as u8;
        GSColor::RGB5([r, g, b])
    }

    /// Returns an rgba byte stream
    pub fn to_rgba(&self) -> [u8; 4] {
        match &self {
            GSColor::RGB5(rgb) => [
                (rgb[0] as u32 * 255 / 31) as u8,
                (rgb[1] as u32 * 255 / 31) as u8,
                (rgb[2] as u32 * 255 / 31) as u8,
                255,
            ],
            _ => [0, 0, 0, 0],
        }
    }
}
