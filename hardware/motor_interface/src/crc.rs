mod constants;

use constants::*;

pub fn crc16(data: &[u8]) -> u16 {

    let mut uch_crc_hi = 0xFF;
    let mut uch_crc_lo = 0xFF;

    for &byte in data {
        let u_index = uch_crc_hi ^ byte;
        let hi = MAGIC_CRC_HI[u_index as usize];
        let lo = MAGIC_CRC_LO[u_index as usize];

        uch_crc_hi = uch_crc_lo ^ hi;
        uch_crc_lo = lo;
    }

    ((uch_crc_hi as u16) << 8) | (uch_crc_lo as u16)
}
