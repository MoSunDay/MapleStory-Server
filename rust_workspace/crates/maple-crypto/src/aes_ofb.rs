// Ported from tools/MapleAESOFB.java
// AES-256 OFB mode with custom IV evolution

use crate::aes256::Aes256;

static AES_KEY: [u8; 32] = [
    0x13, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0xB4, 0x00, 0x00, 0x00,
    0x1B, 0x00, 0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x33, 0x00, 0x00, 0x00, 0x52, 0x00, 0x00, 0x00,
];

pub static FUNNY_BYTES: [u8; 256] = [
    0xEC, 0x3F, 0x77, 0xA4, 0x45, 0xD0, 0x71, 0xBF, 0xB7, 0x98, 0x20, 0xFC, 0x4B, 0xE9, 0xB3, 0xE1,
    0x5C, 0x22, 0xF7, 0x0C, 0x44, 0x1B, 0x81, 0xBD, 0x63, 0x8D, 0xD4, 0xC3, 0xF2, 0x10, 0x19, 0xE0,
    0xFB, 0xA1, 0x6E, 0x66, 0xEA, 0xAE, 0xD6, 0xCE, 0x06, 0x18, 0x4E, 0xEB, 0x78, 0x95, 0xDB, 0xBA,
    0xB6, 0x42, 0x7A, 0x2A, 0x83, 0x0B, 0x54, 0x67, 0x6D, 0xE8, 0x65, 0xE7, 0x2F, 0x07, 0xF3, 0xAA,
    0x27, 0x7B, 0x85, 0xB0, 0x26, 0xFD, 0x8B, 0xA9, 0xFA, 0xBE, 0xA8, 0xD7, 0xCB, 0xCC, 0x92, 0xDA,
    0xF9, 0x93, 0x60, 0x2D, 0xDD, 0xD2, 0xA2, 0x9B, 0x39, 0x5F, 0x82, 0x21, 0x4C, 0x69, 0xF8, 0x31,
    0x87, 0xEE, 0x8E, 0xAD, 0x8C, 0x6A, 0xBC, 0xB5, 0x6B, 0x59, 0x13, 0xF1, 0x04, 0x00, 0xF6, 0x5A,
    0x35, 0x79, 0x48, 0x8F, 0x15, 0xCD, 0x97, 0x57, 0x12, 0x3E, 0x37, 0xFF, 0x9D, 0x4F, 0x51, 0xF5,
    0xA3, 0x70, 0xBB, 0x14, 0x75, 0xC2, 0xB8, 0x72, 0xC0, 0xED, 0x7D, 0x68, 0xC9, 0x2E, 0x0D, 0x62,
    0x46, 0x17, 0x11, 0x4D, 0x6C, 0xC4, 0x7E, 0x53, 0xC1, 0x25, 0xC7, 0x9A, 0x1C, 0x88, 0x58, 0x2C,
    0x89, 0xDC, 0x02, 0x64, 0x40, 0x01, 0x5D, 0x38, 0xA5, 0xE2, 0xAF, 0x55, 0xD5, 0xEF, 0x1A, 0x7C,
    0xA7, 0x5B, 0xA6, 0x6F, 0x86, 0x9F, 0x73, 0xE6, 0x0A, 0xDE, 0x2B, 0x99, 0x4A, 0x47, 0x9C, 0xDF,
    0x09, 0x76, 0x9E, 0x30, 0x0E, 0xE4, 0xB2, 0x94, 0xA0, 0x3B, 0x34, 0x1D, 0x28, 0x0F, 0x36, 0xE3,
    0x23, 0xB4, 0x03, 0xD8, 0x90, 0xC8, 0x3C, 0xFE, 0x5E, 0x32, 0x24, 0x50, 0x1F, 0x3A, 0x43, 0x8A,
    0x96, 0x41, 0x74, 0xAC, 0x52, 0x33, 0xF0, 0xD9, 0x29, 0x80, 0xB1, 0x16, 0xD3, 0xAB, 0x91, 0xB9,
    0x84, 0x7F, 0x61, 0x1E, 0xCF, 0xC5, 0xD1, 0x56, 0x3D, 0xCA, 0xF4, 0x05, 0xC6, 0xE5, 0x08, 0x49,
];

fn multiply_bytes(input: &[u8], count: usize, mul: usize) -> Vec<u8> {
    let mut ret = vec![0u8; count * mul];
    for x in 0..count * mul {
        ret[x] = input[x % count];
    }
    ret
}

fn new_cipher() -> Aes256 {
    Aes256::new(AES_KEY)
}

pub struct MapleAesOfb {
    iv: [u8; 4],
    maple_version: u16,
    cipher: Aes256,
}

impl MapleAesOfb {
    pub fn new(iv: [u8; 4], maple_version: u16) -> Self {
        let swapped_version = ((maple_version >> 8) & 0xFF) | ((maple_version << 8) & 0xFF00);
        MapleAesOfb {
            iv,
            maple_version: swapped_version,
            cipher: new_cipher(),
        }
    }

    pub fn crypt(&mut self, data: &mut [u8]) {
        let mut remaining = data.len() as i32;
        let mut llength: i32 = 0x5B0;
        let mut start: usize = 0;
        while remaining > 0 {
            let mut my_iv = multiply_bytes(&self.iv, 4, 4);
            if remaining < llength {
                llength = remaining;
            }
            let end = start + llength as usize;
            for x in start..end {
                let offset = x - start;
                if offset % my_iv.len() == 0 {
                    let mut block: [u8; 16] = [
                        my_iv[0], my_iv[1], my_iv[2], my_iv[3], my_iv[4], my_iv[5], my_iv[6],
                        my_iv[7], my_iv[8], my_iv[9], my_iv[10], my_iv[11], my_iv[12], my_iv[13],
                        my_iv[14], my_iv[15],
                    ];
                    self.cipher.encrypt_block(&mut block);
                    my_iv[0..16].copy_from_slice(&block);
                }
                data[x] ^= my_iv[offset % my_iv.len()];
            }
            start = end;
            remaining -= llength;
            llength = 0x5B4;
        }
        self.update_iv();
    }

    pub fn get_packet_header(&self, length: usize) -> [u8; 4] {
        let length = length as u32;
        let iiv: u32 = (self.iv[3] as u32) | ((self.iv[2] as u32) << 8);
        let iiv = iiv ^ (self.maple_version as u32);
        let mlength: u32 = ((length << 8) & 0xFF00) | ((length >> 8) & 0xFF);
        let xored_iv: u32 = iiv ^ mlength;
        [
            ((iiv >> 8) & 0xFF) as u8,
            (iiv & 0xFF) as u8,
            ((xored_iv >> 8) & 0xFF) as u8,
            (xored_iv & 0xFF) as u8,
        ]
    }

    pub fn get_packet_length(packet_header: i32) -> i32 {
        let mut packet_length =
            ((packet_header as u32 >> 16) ^ (packet_header as u32 & 0xFFFF)) as i32;
        packet_length =
            ((packet_length << 8) & 0xFF00) | ((packet_length as u32 >> 8) & 0xFF) as i32;
        packet_length
    }

    pub fn check_packet_bytes(&self, packet: &[u8]) -> bool {
        ((packet[0] ^ self.iv[2]) as u16) == ((self.maple_version >> 8) & 0xFF)
            && ((packet[1] ^ self.iv[3]) as u16) == (self.maple_version & 0xFF)
    }

    pub fn check_packet_header(&self, packet_header: i32) -> bool {
        let buf: [u8; 2] = [
            ((packet_header as u32 >> 24) & 0xFF) as u8,
            ((packet_header as u32 >> 16) & 0xFF) as u8,
        ];
        self.check_packet_bytes(&buf)
    }

    fn update_iv(&mut self) {
        self.iv = get_new_iv(&self.iv);
    }
}

pub fn get_new_iv(old_iv: &[u8; 4]) -> [u8; 4] {
    let mut result: [u8; 4] = [0xf2, 0x53, 0x50, 0xc6];
    for x in 0..4 {
        funny_shit(old_iv[x], &mut result);
    }
    result
}

fn funny_shit(input_byte: u8, state: &mut [u8; 4]) {
    let mut elina = state[1];
    let anna = input_byte;
    let mut moritz = FUNNY_BYTES[elina as usize];
    moritz = moritz.wrapping_sub(input_byte);
    state[0] = state[0].wrapping_add(moritz);
    moritz = state[2];
    moritz ^= FUNNY_BYTES[anna as usize];
    elina = elina.wrapping_sub(moritz);
    state[1] = elina;
    elina = state[3];
    moritz = elina;
    elina = elina.wrapping_sub(state[0]);
    moritz = FUNNY_BYTES[moritz as usize];
    moritz = moritz.wrapping_add(input_byte);
    moritz ^= state[2];
    state[2] = moritz;
    elina = elina.wrapping_add(FUNNY_BYTES[anna as usize]);
    state[3] = elina;
    let mut merry: u32 = (state[0] as u32)
        | ((state[1] as u32) << 8)
        | ((state[2] as u32) << 16)
        | ((state[3] as u32) << 24);
    let ret_value: u32 = merry >> 0x1d;
    merry = merry << 3;
    let merry = ret_value | merry;
    state[0] = (merry & 0xFF) as u8;
    state[1] = ((merry >> 8) & 0xFF) as u8;
    state[2] = ((merry >> 16) & 0xFF) as u8;
    state[3] = ((merry >> 24) & 0xFF) as u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes256_encrypt_block_via_crypt() {
        // Verify AES-256 encrypts correctly through the OFB path
        let iv = [0x00, 0x00, 0x00, 0x00];
        let mut cipher = MapleAesOfb::new(iv, 83);

        let plaintext: Vec<u8> = vec![0; 16];
        let mut data = plaintext.clone();
        cipher.crypt(&mut data);

        // XOR of plaintext (zeros) with encrypted all-zero IV block
        // The result should equal the first AES-256-ECB encryption of the padded IV
        assert_ne!(
            data, plaintext,
            "Encrypted data should differ from plaintext"
        );
        assert_eq!(data.len(), 16);
    }

    #[test]
    fn test_packet_header_roundtrip() {
        let iv = [0x01, 0x02, 0x03, 0x04];
        let version: u16 = 83;
        let cipher = MapleAesOfb::new(iv, version);
        let header = cipher.get_packet_header(100);
        let header_i32 = i32::from_be_bytes([header[0], header[1], header[2], header[3]]);
        let length = MapleAesOfb::get_packet_length(header_i32);
        assert_eq!(length, 100);
    }

    #[test]
    fn test_packet_header_roundtrip_varied() {
        let iv = [0xAB, 0xCD, 0xEF, 0x12];
        let version: u16 = 83;
        let cipher = MapleAesOfb::new(iv, version);
        for len in [1, 16, 100, 255, 1024, 0xFFFF] {
            let header = cipher.get_packet_header(len);
            let header_i32 = i32::from_be_bytes([header[0], header[1], header[2], header[3]]);
            let decoded = MapleAesOfb::get_packet_length(header_i32);
            assert_eq!(decoded, len as i32, "failed for length {len}");
        }
    }

    #[test]
    fn test_check_packet() {
        let iv = [0x01, 0x02, 0x03, 0x04];
        let version: u16 = 83;
        let cipher = MapleAesOfb::new(iv, version);
        let header = cipher.get_packet_header(100);
        assert!(cipher.check_packet_bytes(&header));
    }

    #[test]
    fn test_check_packet_header() {
        let iv = [0x01, 0x02, 0x03, 0x04];
        let version: u16 = 83;
        let cipher = MapleAesOfb::new(iv, version);
        let header_bytes = cipher.get_packet_header(100);
        let header_i32 = i32::from_be_bytes([
            header_bytes[0],
            header_bytes[1],
            header_bytes[2],
            header_bytes[3],
        ]);
        assert!(cipher.check_packet_header(header_i32));
    }

    #[test]
    fn test_crypt_idempotency() {
        let iv = [0xAA, 0xBB, 0xCC, 0xDD];
        let version: u16 = 83;
        let original: Vec<u8> = (0..64u8).collect();

        let mut enc = MapleAesOfb::new(iv, version);
        let mut enc_data = original.clone();
        enc.crypt(&mut enc_data);

        let mut dec = MapleAesOfb::new(iv, version);
        let mut dec_data = enc_data.clone();
        dec.crypt(&mut dec_data);

        assert_eq!(original, dec_data);
    }

    #[test]
    fn test_get_new_iv() {
        let old_iv = [0x01, 0x02, 0x03, 0x04];
        let new_iv = get_new_iv(&old_iv);
        assert_ne!(old_iv, new_iv);
        let new_iv2 = get_new_iv(&new_iv);
        assert_ne!(new_iv, new_iv2);
    }

    #[test]
    fn test_funny_bytes_table() {
        assert_eq!(FUNNY_BYTES[0], 0xEC);
        assert_eq!(FUNNY_BYTES[255], 0x49);
        assert_eq!(FUNNY_BYTES.len(), 256);
    }
}
