//! Some util functions, such as converting from byte slice to a word

/// Convert an appropriately sized byte slice into a u16 word
///
/// Assumed Little-Endian for conversion (Endianess of the GBA)
pub(crate) const fn as_u16(bytes: &[u8]) -> u16 {
    // TODO: Check for correct input size
    (bytes[1] as u16) << 8 | bytes[0] as u16
}

/// Convert an appropriately sized byte slice into a u32 word
///
/// Assumed Little-Endian for conversion (Endianess of the GBA)
pub(crate) const fn as_u32(bytes: &[u8]) -> u32 {
    // TODO: Check for correct input size
    (bytes[3] as u32) << 24 | (bytes[2] as u32) << 16 | (bytes[1] as u32) << 8 | bytes[0] as u32
}
