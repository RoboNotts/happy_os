const fn generate_crc_table() -> [[u8;2]; 256] {
    const GENERATOR: u16 = 0x1021;
    let mut crctable16 = [[0u8; 2]; 256];

    let mut dividend : usize = 0;
    while dividend < 256 {
        let mut cur_byte: u16 = (dividend as u16) << 8; /* move dividend byte into MSB of 16Bit CRC */

        let mut bit = 0;
        while bit < 8
        {
            if (cur_byte & 0x8000) != 0
            {
                cur_byte <<= 1;
                cur_byte ^= GENERATOR;
            }
            else
            {
                cur_byte <<= 1;
            }

            bit += 1;
        }
        crctable16[dividend] = [(cur_byte << 8) as u8, cur_byte as u8];
        dividend += 1;
    }

    crctable16
}

pub fn crc16(data: &[u8]) -> u16 {
    const CRC_TABLE: [[u8; 2]; 256] = generate_crc_table();

    let mut uch_crchi = 0xFF;
    let mut uch_crclo = 0xFF;

    for &byte in data {
        let u_index = uch_crchi ^ byte;
        let [hi, lo] = CRC_TABLE[u_index as usize];

        uch_crchi = uch_crclo ^ hi;
        uch_crclo = lo;
    }

    (u16::from(uch_crchi) << 8) | u16::from(uch_crclo)
}