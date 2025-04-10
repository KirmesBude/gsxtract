#[cfg(test)]
mod tests {
    use gsxtract::color::GSColor;
    use gsxtract::sprite::GSSprite;

    const PALETTE: [GSColor; 0xE0] = [
        GSColor::Transparent,
        GSColor::RGB5([0, 0, 0]),
        GSColor::RGB5([255, 0, 0]),
        GSColor::RGB5([255, 115, 0]),
        GSColor::RGB5([255, 255, 0]),
        GSColor::RGB5([0, 255, 0]),
        GSColor::RGB5([57, 115, 57]),
        GSColor::RGB5([0, 255, 255]),
        GSColor::RGB5([0, 0, 255]),
        GSColor::RGB5([230, 115, 180]),
        GSColor::RGB5([255, 180, 115]),
        GSColor::RGB5([180, 115, 0]),
        GSColor::RGB5([115, 0, 0]),
        GSColor::RGB5([57, 57, 57]),
        GSColor::RGB5([156, 156, 156]),
        GSColor::RGB5([255, 255, 255]),
        GSColor::RGB5([255, 57, 0]),
        GSColor::RGB5([230, 115, 0]),
        GSColor::RGB5([57, 0, 0]),
        GSColor::RGB5([0, 180, 0]),
        GSColor::RGB5([57, 180, 0]),
        GSColor::RGB5([115, 180, 0]),
        GSColor::RGB5([180, 180, 0]),
        GSColor::RGB5([230, 180, 0]),
        GSColor::RGB5([255, 180, 0]),
        GSColor::RGB5([0, 230, 0]),
        GSColor::RGB5([57, 230, 0]),
        GSColor::RGB5([115, 230, 0]),
        GSColor::RGB5([180, 230, 0]),
        GSColor::RGB5([230, 230, 0]),
        GSColor::RGB5([255, 230, 0]),
        GSColor::RGB5([180, 0, 0]),
        GSColor::RGB5([57, 255, 0]),
        GSColor::RGB5([115, 255, 0]),
        GSColor::RGB5([180, 255, 0]),
        GSColor::RGB5([230, 255, 0]),
        GSColor::RGB5([57, 57, 0]),
        GSColor::RGB5([0, 0, 57]),
        GSColor::RGB5([57, 0, 57]),
        GSColor::RGB5([115, 0, 57]),
        GSColor::RGB5([180, 0, 57]),
        GSColor::RGB5([230, 0, 57]),
        GSColor::RGB5([255, 0, 57]),
        GSColor::RGB5([0, 57, 57]),
        GSColor::RGB5([230, 0, 0]),
        GSColor::RGB5([115, 57, 57]),
        GSColor::RGB5([180, 57, 57]),
        GSColor::RGB5([230, 57, 57]),
        GSColor::RGB5([255, 57, 57]),
        GSColor::RGB5([0, 115, 57]),
        GSColor::RGB5([0, 115, 0]),
        GSColor::RGB5([115, 115, 57]),
        GSColor::RGB5([180, 115, 57]),
        GSColor::RGB5([230, 115, 57]),
        GSColor::RGB5([255, 115, 57]),
        GSColor::RGB5([0, 180, 57]),
        GSColor::RGB5([57, 180, 57]),
        GSColor::RGB5([115, 180, 57]),
        GSColor::RGB5([180, 180, 57]),
        GSColor::RGB5([230, 180, 57]),
        GSColor::RGB5([255, 180, 57]),
        GSColor::RGB5([0, 230, 57]),
        GSColor::RGB5([57, 230, 57]),
        GSColor::RGB5([115, 230, 57]),
        GSColor::RGB5([180, 230, 57]),
        GSColor::RGB5([230, 230, 57]),
        GSColor::RGB5([255, 230, 57]),
        GSColor::RGB5([0, 255, 57]),
        GSColor::RGB5([57, 255, 57]),
        GSColor::RGB5([115, 255, 57]),
        GSColor::RGB5([180, 255, 57]),
        GSColor::RGB5([230, 255, 57]),
        GSColor::RGB5([255, 255, 57]),
        GSColor::RGB5([0, 0, 115]),
        GSColor::RGB5([57, 0, 115]),
        GSColor::RGB5([115, 0, 115]),
        GSColor::RGB5([180, 0, 115]),
        GSColor::RGB5([230, 0, 115]),
        GSColor::RGB5([255, 0, 115]),
        GSColor::RGB5([0, 57, 115]),
        GSColor::RGB5([57, 57, 115]),
        GSColor::RGB5([115, 57, 115]),
        GSColor::RGB5([180, 57, 115]),
        GSColor::RGB5([230, 57, 115]),
        GSColor::RGB5([255, 57, 115]),
        GSColor::RGB5([0, 115, 115]),
        GSColor::RGB5([57, 115, 115]),
        GSColor::RGB5([115, 115, 115]),
        GSColor::RGB5([180, 115, 115]),
        GSColor::RGB5([230, 115, 115]),
        GSColor::RGB5([255, 115, 115]),
        GSColor::RGB5([0, 180, 115]),
        GSColor::RGB5([57, 180, 115]),
        GSColor::RGB5([115, 180, 115]),
        GSColor::RGB5([180, 180, 115]),
        GSColor::RGB5([230, 180, 115]),
        GSColor::RGB5([230, 57, 0]),
        GSColor::RGB5([0, 230, 115]),
        GSColor::RGB5([57, 230, 115]),
        GSColor::RGB5([115, 230, 115]),
        GSColor::RGB5([180, 230, 115]),
        GSColor::RGB5([230, 230, 115]),
        GSColor::RGB5([255, 230, 115]),
        GSColor::RGB5([0, 255, 115]),
        GSColor::RGB5([57, 255, 115]),
        GSColor::RGB5([115, 255, 115]),
        GSColor::RGB5([180, 255, 115]),
        GSColor::RGB5([230, 255, 115]),
        GSColor::RGB5([255, 255, 115]),
        GSColor::RGB5([0, 0, 180]),
        GSColor::RGB5([57, 0, 180]),
        GSColor::RGB5([115, 0, 180]),
        GSColor::RGB5([180, 0, 180]),
        GSColor::RGB5([230, 0, 180]),
        GSColor::RGB5([255, 0, 180]),
        GSColor::RGB5([0, 57, 180]),
        GSColor::RGB5([57, 57, 180]),
        GSColor::RGB5([115, 57, 180]),
        GSColor::RGB5([180, 57, 180]),
        GSColor::RGB5([230, 57, 180]),
        GSColor::RGB5([255, 57, 180]),
        GSColor::RGB5([0, 115, 180]),
        GSColor::RGB5([57, 115, 180]),
        GSColor::RGB5([115, 115, 180]),
        GSColor::RGB5([180, 115, 180]),
        GSColor::RGB5([180, 57, 0]),
        GSColor::RGB5([255, 115, 180]),
        GSColor::RGB5([0, 180, 180]),
        GSColor::RGB5([57, 180, 180]),
        GSColor::RGB5([115, 180, 180]),
        GSColor::RGB5([180, 180, 180]),
        GSColor::RGB5([230, 180, 180]),
        GSColor::RGB5([255, 180, 180]),
        GSColor::RGB5([0, 230, 180]),
        GSColor::RGB5([57, 230, 180]),
        GSColor::RGB5([115, 230, 180]),
        GSColor::RGB5([180, 230, 180]),
        GSColor::RGB5([230, 230, 180]),
        GSColor::RGB5([255, 230, 180]),
        GSColor::RGB5([0, 255, 180]),
        GSColor::RGB5([57, 255, 180]),
        GSColor::RGB5([115, 255, 180]),
        GSColor::RGB5([180, 255, 180]),
        GSColor::RGB5([230, 255, 180]),
        GSColor::RGB5([255, 255, 180]),
        GSColor::RGB5([0, 0, 230]),
        GSColor::RGB5([57, 0, 230]),
        GSColor::RGB5([115, 0, 230]),
        GSColor::RGB5([180, 0, 230]),
        GSColor::RGB5([230, 0, 230]),
        GSColor::RGB5([255, 0, 230]),
        GSColor::RGB5([0, 57, 230]),
        GSColor::RGB5([57, 57, 230]),
        GSColor::RGB5([115, 57, 230]),
        GSColor::RGB5([180, 57, 230]),
        GSColor::RGB5([230, 57, 230]),
        GSColor::RGB5([255, 57, 230]),
        GSColor::RGB5([0, 115, 230]),
        GSColor::RGB5([57, 115, 230]),
        GSColor::RGB5([115, 115, 230]),
        GSColor::RGB5([180, 115, 230]),
        GSColor::RGB5([230, 115, 230]),
        GSColor::RGB5([255, 115, 230]),
        GSColor::RGB5([0, 180, 230]),
        GSColor::RGB5([57, 180, 230]),
        GSColor::RGB5([115, 180, 230]),
        GSColor::RGB5([180, 180, 230]),
        GSColor::RGB5([230, 180, 230]),
        GSColor::RGB5([255, 180, 230]),
        GSColor::RGB5([0, 230, 230]),
        GSColor::RGB5([57, 230, 230]),
        GSColor::RGB5([115, 230, 230]),
        GSColor::RGB5([180, 230, 230]),
        GSColor::RGB5([230, 230, 230]),
        GSColor::RGB5([255, 230, 230]),
        GSColor::RGB5([0, 255, 230]),
        GSColor::RGB5([57, 255, 230]),
        GSColor::RGB5([115, 255, 230]),
        GSColor::RGB5([180, 255, 230]),
        GSColor::RGB5([230, 255, 230]),
        GSColor::RGB5([255, 255, 230]),
        GSColor::RGB5([0, 57, 0]),
        GSColor::RGB5([57, 0, 255]),
        GSColor::RGB5([115, 0, 255]),
        GSColor::RGB5([180, 0, 255]),
        GSColor::RGB5([230, 0, 255]),
        GSColor::RGB5([255, 0, 255]),
        GSColor::RGB5([0, 57, 255]),
        GSColor::RGB5([57, 57, 255]),
        GSColor::RGB5([115, 57, 255]),
        GSColor::RGB5([180, 57, 255]),
        GSColor::RGB5([230, 57, 255]),
        GSColor::RGB5([255, 57, 255]),
        GSColor::RGB5([0, 115, 255]),
        GSColor::RGB5([57, 115, 255]),
        GSColor::RGB5([115, 115, 255]),
        GSColor::RGB5([180, 115, 255]),
        GSColor::RGB5([230, 115, 255]),
        GSColor::RGB5([255, 115, 255]),
        GSColor::RGB5([0, 180, 255]),
        GSColor::RGB5([57, 180, 255]),
        GSColor::RGB5([115, 180, 255]),
        GSColor::RGB5([180, 180, 255]),
        GSColor::RGB5([230, 180, 255]),
        GSColor::RGB5([255, 180, 255]),
        GSColor::RGB5([0, 230, 255]),
        GSColor::RGB5([57, 230, 255]),
        GSColor::RGB5([115, 230, 255]),
        GSColor::RGB5([180, 230, 255]),
        GSColor::RGB5([230, 230, 255]),
        GSColor::RGB5([255, 230, 255]),
        GSColor::RGB5([115, 57, 0]),
        GSColor::RGB5([57, 255, 255]),
        GSColor::RGB5([115, 255, 255]),
        GSColor::RGB5([180, 255, 255]),
        GSColor::RGB5([230, 255, 255]),
        GSColor::RGB5([115, 115, 0]),
        GSColor::RGB5([24, 24, 24]),
        GSColor::RGB5([41, 41, 41]),
        GSColor::RGB5([82, 82, 82]),
        GSColor::RGB5([98, 98, 98]),
        GSColor::RGB5([57, 115, 0]),
        GSColor::RGB5([131, 131, 131]),
        GSColor::RGB5([205, 205, 205]),
    ];

    #[test]
    fn felix() {
        let width = 32;
        let height = 32;
        let scale = 0x100;
        let raw_data = vec![
            0xFF, 0xFF, 0xFF, 0xE8, 0x2D, 0xFF, 0xD3, 0x24, 0xFE, 0xD3, 0x24, 0xFE, 0x24, 0x24,
            0xD9, 0xD9, 0x24, 0xDA, 0xF6, 0xDA, 0x24, 0x12, 0x12, 0x12, 0x24, 0x24, 0x24, 0x12,
            0x24, 0xF5, 0x24, 0x12, 0x24, 0x24, 0x12, 0x12, 0x12, 0x24, 0x24, 0x2D, 0xDA, 0xF3,
            0x33, 0x24, 0xD3, 0xD3, 0x24, 0xD3, 0xD3, 0x2D, 0x24, 0x24, 0x2D, 0xD9, 0xF3, 0x2D,
            0x5E, 0x5F, 0x5E, 0x34, 0x5E, 0x5E, 0x89, 0x5E, 0x5E, 0x0D, 0xD9, 0xF2, 0x24, 0x24,
            0x2D, 0x34, 0x5F, 0x34, 0x8A, 0x5F, 0x5E, 0x34, 0x5E, 0x33, 0x33, 0xF2, 0x24, 0x12,
            0xD3, 0x24, 0xD3, 0x24, 0x35, 0x0B, 0xD3, 0xD3, 0x24, 0x2D, 0xDB, 0xF2, 0xDA, 0xD9,
            0x24, 0x24, 0x24, 0xD3, 0xD8, 0x0B, 0x24, 0x24, 0x24, 0x57, 0xDB, 0xF3, 0x12, 0x24,
            0x12, 0x24, 0x2D, 0xD3, 0xD3, 0x24, 0x24, 0x12, 0x0E, 0xDE, 0xE2, 0xD3, 0xD3, 0xEE,
            0xD9, 0x12, 0x12, 0xD3, 0x24, 0xD3, 0xD3, 0x24, 0x12, 0x12, 0x5E, 0x58, 0xE0, 0x2D,
            0xD3, 0xD3, 0xD3, 0x2D, 0xEA, 0xD9, 0xDA, 0x0D, 0xAD, 0xDC, 0x01, 0x24, 0x24, 0xD3,
            0x24, 0x12, 0x24, 0x34, 0x34, 0xDB, 0xE0, 0xD3, 0xD3, 0x34, 0xD3, 0x24, 0xE7, 0x0D,
            0xDB, 0xDB, 0x58, 0x5E, 0x5E, 0x0E, 0x82, 0xD9, 0xD9, 0x12, 0x24, 0x24, 0x12, 0x33,
            0x33, 0x0B, 0x06, 0xB4, 0x83, 0x0B, 0xD3, 0x24, 0xE6, 0xDB, 0xDB, 0xDB, 0xDB, 0x0D,
            0xDA, 0xDA, 0x50, 0x7A, 0x9F, 0x9E, 0x7A, 0x50, 0x24, 0x12, 0xD9, 0x13, 0x14, 0x11,
            0x5D, 0x0F, 0xB4, 0xD3, 0x24, 0xD9, 0xE8, 0x7B, 0x9E, 0x9E, 0x9F, 0xC9, 0xC9, 0xC9,
            0x9E, 0x74, 0x7A, 0xD0, 0x24, 0x13, 0x32, 0x32, 0xDD, 0xD8, 0x5D, 0x89, 0x2D, 0x12,
            0x24, 0x24, 0xE8, 0xC9, 0xC9, 0xC9, 0xA5, 0x7A, 0x74, 0x74, 0x9E, 0xC9, 0xD0, 0x88,
            0x32, 0x13, 0x13, 0x13, 0x32, 0xB5, 0x12, 0xED, 0x7A, 0x50, 0x4F, 0x50, 0x7A, 0x9E,
            0xC9, 0xD0, 0xD6, 0xD7, 0x0D, 0xB5, 0x32, 0x32, 0x32, 0x32, 0xB5, 0xD9, 0xED, 0x50,
            0x7A, 0x9F, 0xC9, 0xC9, 0xD0, 0xD6, 0xD6, 0xD7, 0xE0, 0xDA, 0xD9, 0xD9, 0xB5, 0xB5,
            0xD9, 0x24, 0x34, 0xEE, 0xC9, 0xCF, 0xD0, 0xD6, 0xD6, 0xD6, 0xD6, 0xE1, 0x33, 0x33,
            0x34, 0x33, 0x5E, 0x65, 0x65, 0x5E, 0x33, 0xEE, 0xD6, 0xD6, 0xD6, 0xD7, 0xE4, 0x33,
            0x5F, 0x8A, 0x8A, 0x8A, 0x8A, 0x58, 0xD3, 0x0B, 0xE0, 0xD9, 0x0D, 0xDA, 0xE9, 0xD6,
            0xD6, 0xD7, 0xE5, 0x12, 0x33, 0x5E, 0x5F, 0x34, 0x2D, 0x2D, 0x5E, 0x17, 0x0B, 0x0D,
            0xDB, 0x0D, 0xF3, 0x12, 0x12, 0x12, 0xDE, 0xAD, 0x0F, 0x5F, 0x0B, 0xD8, 0xD9, 0x0D,
            0xDB, 0xF7, 0x0F, 0x8A, 0x17, 0x18, 0x0B, 0xDB, 0xDB, 0xDB, 0xF7, 0x33, 0x0B, 0x17,
            0x18, 0x3A, 0x0D, 0x0D, 0xF9, 0xD3, 0x0B, 0x0B, 0x0D, 0xDA, 0xFB, 0x24, 0xDA, 0xDA,
            0xFD, 0xDA, 0xE9, 0x00,
        ];

        let sprite = GSSprite::from_compression_format0(width, height, scale, &raw_data, &PALETTE);

        let expected_size = width as usize * height as usize;
        let actual_size = sprite.size();
        assert!(
            actual_size == expected_size,
            "Expected {}, but was {}!",
            expected_size,
            actual_size
        );
    }
}
