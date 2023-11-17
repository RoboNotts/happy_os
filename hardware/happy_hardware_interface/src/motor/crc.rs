const fn generate_crc_table(is_high: bool) -> [u8; 256] {
    let polynomial = if is_high { 0x1021 } else { 0x8408 };
    let mut crc_table = [0u8; 256];

    for i in 0..256 {
        let mut value = i as u16;
        let mut crc = 0;

        for _ in 0..8 {
            if (value ^ crc) & 1 != 0 {
                crc = (crc >> 1) ^ polynomial;
            } else {
                crc >>= 1;
            }
            value >>= 1;
        }

        crc_table[i] = crc as u8;
    }

    crc_table
}

pub fn crc16(data: &[u8]) -> u16 {
    const AUCH_CRCHI: [u8; 256] = generate_crc_table(true);
    const AUCH_CRCLO: [u8; 256] = generate_crc_table(false);


    let mut uch_crchi = 0xFF;
    let mut uch_crclo = 0xFF;

    for &byte in data {
        let u_index = uch_crchi ^ byte;
        uch_crchi = uch_crclo ^ AUCH_CRCHI[u_index as usize];
        uch_crclo = AUCH_CRCLO[u_index as usize];
    }

    (u16::from(uch_crchi) << 8) | u16::from(uch_crclo)
}