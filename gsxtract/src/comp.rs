use crate::util::NextExt;
use bitvec::{order::Msb0, view::BitViewSized};

pub fn from_standard_lz(raw_data: &[u8]) -> Vec<u8> {
    let mut iter = raw_data.iter();
    let format = iter.next_u8();

    match format {
        0x00 => from_standard_lz_00(iter),
        0x01 => from_standard_lz_01(iter),
        0x02 => from_standard_lz_02(iter),
        _ => vec![], /* TODO: Handle better */
    }
}

fn from_standard_lz_00<'a>(_iter: impl IntoIterator<Item = &'a u8>) -> Vec<u8> {
    vec![]
}

fn from_standard_lz_01<'a>(iter: impl IntoIterator<Item = &'a u8>) -> Vec<u8> {
    let mut iter = iter.into_iter();
    let mut decoded = vec![];

    loop {
        let byte = iter.next_u8();

        for bit in byte.into_bitarray::<Msb0>() {
            if !bit {
                decoded.push(iter.next_u8());
            } else {
                let byte1 = iter.next_u8();
                let byte2 = iter.next_u8();

                let mut nbytes = byte1 as usize & 0x0F;
                let offset = ((byte1 as usize & 0xF0) << 4) | byte2 as usize;

                if nbytes == 0 {
                    if offset == 0 {
                        return decoded;
                    } else {
                        nbytes = iter.next_u8() as usize + 16;
                    }
                }

                for _ in 0..=nbytes {
                    decoded.push(decoded[decoded.len() - offset]);
                }
            }
        }
    }
}

fn from_standard_lz_02<'a>(_iter: impl IntoIterator<Item = &'a u8>) -> Vec<u8> {
    vec![]
}
