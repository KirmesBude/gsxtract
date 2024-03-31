use bitvec::{order::Msb0, view::BitViewSized};


pub fn from_standard_lz(raw_data: &[u8]) -> Vec<u8> {
    let format = raw_data[0];
    let raw_data = &raw_data[1..];

    let decoded = match format {
        0x00 => from_standard_lz_00(raw_data),
        0x01 => from_standard_lz_01(raw_data),
        0x02 => from_standard_lz_02(raw_data),
        _ => vec![], /* TODO: Handle better */
    };

    decoded
}

pub fn from_standard_lz_00(raw_data: &[u8]) -> Vec<u8> {
    vec![]
}

pub fn from_standard_lz_01(raw_data: &[u8]) -> Vec<u8> {
    let mut decoded = vec![];
    let mut index = 0;

    loop {
        let byte = raw_data[index];
        index = index + 1;
        
        for bit in byte.into_bitarray::<Msb0>() {
            if !bit {
                decoded.push(raw_data[index]);
                index = index + 1;
            } else {
                let byte1 = raw_data[index];
                index = index + 1;
                let byte2 = raw_data[index];
                index = index + 1;

                let mut nbytes = byte1 as usize & 0x0F;
                let offset = ((byte1 as usize & 0xF0) << 4) | byte2 as usize;

                if nbytes == 0 {
                    if offset == 0 {
                        return decoded
                    } else {
                        nbytes = raw_data[index] as usize + 16;
                        index = index + 1;
                    }
                }

                nbytes = nbytes + 1;
                while nbytes > 0 {
                    decoded.push(decoded[decoded.len()-offset]);
                    nbytes = nbytes - 1;
                }
            }
        }
    }
}

pub fn from_standard_lz_02(raw_data: &[u8]) -> Vec<u8> {
    vec![]
}