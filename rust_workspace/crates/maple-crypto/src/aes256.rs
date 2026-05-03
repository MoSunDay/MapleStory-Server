// AES-256 block cipher (software implementation, no external deps)
// Matches Java's Cipher.getInstance("AES") with 32-byte key in ECB mode

const SBOX: [u8; 256] = [
    0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
    0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
    0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,
    0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,
    0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,
    0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,
    0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,
    0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,
    0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,
    0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,
    0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,
    0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,
    0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,
    0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,
    0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
    0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16,
];

fn sub_byte(w: u32) -> u32 {
    let b = w.to_le_bytes();
    u32::from_le_bytes([
        SBOX[b[0] as usize],
        SBOX[b[1] as usize],
        SBOX[b[2] as usize],
        SBOX[b[3] as usize],
    ])
}

fn sub_word(word: u32) -> u32 {
    let b0 = SBOX[(word as u8) as usize] as u32;
    let b1 = SBOX[((word >> 8) as u8) as usize] as u32;
    let b2 = SBOX[((word >> 16) as u8) as usize] as u32;
    let b3 = SBOX[((word >> 24) as u8) as usize] as u32;
    b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
}

fn rot_word(word: u32) -> u32 {
    (word >> 8) | (word << 24)
}

fn xtime(a: u8) -> u8 {
    let b = (a as i32) << 1;
    if (a & 0x80) != 0 {
        (b ^ 0x1B) as u8
    } else {
        b as u8
    }
}

fn xtime_column(w: u32) -> u32 {
    let b = w.to_le_bytes();
    let x = xtime;
    let t = b[0] ^ b[1] ^ b[2] ^ b[3];
    u32::from_le_bytes([
        b[0] ^ t ^ x(b[0] ^ b[1]),
        b[1] ^ t ^ x(b[1] ^ b[2]),
        b[2] ^ t ^ x(b[2] ^ b[3]),
        b[3] ^ t ^ x(b[3] ^ b[0]),
    ])
}

pub struct Aes256 {
    round_keys: [u32; 60],
}

impl Aes256 {
    pub fn new(key: [u8; 32]) -> Self {
        let round_keys = key_expansion(&key);
        Aes256 { round_keys }
    }

    pub fn encrypt_block(&self, block: &mut [u8; 16]) {
        let mut state = [
            u32::from_le_bytes([block[0], block[1], block[2], block[3]]),
            u32::from_le_bytes([block[4], block[5], block[6], block[7]]),
            u32::from_le_bytes([block[8], block[9], block[10], block[11]]),
            u32::from_le_bytes([block[12], block[13], block[14], block[15]]),
        ];

        add_round_key(&mut state, &self.round_keys, 0);

        for round in 1..14 {
            sub_bytes(&mut state);
            shift_rows(&mut state);
            mix_columns(&mut state);
            add_round_key(&mut state, &self.round_keys, round);
        }

        sub_bytes(&mut state);
        shift_rows(&mut state);
        add_round_key(&mut state, &self.round_keys, 14);

        let b = [
            state[0].to_le_bytes(),
            state[1].to_le_bytes(),
            state[2].to_le_bytes(),
            state[3].to_le_bytes(),
        ];
        block.copy_from_slice(&b.concat());
    }
}

fn key_expansion(key: &[u8; 32]) -> [u32; 60] {
    let mut w = [0u32; 60];
    for i in 0..8 {
        w[i] = u32::from_le_bytes([key[i * 4], key[i * 4 + 1], key[i * 4 + 2], key[i * 4 + 3]]);
    }
    let rcon: [u32; 8] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80];
    for i in 8..60 {
        let mut temp = w[i - 1];
        if i % 8 == 0 {
            temp = sub_word(rot_word(temp)) ^ rcon[i / 8 - 1];
        } else if i % 8 == 4 {
            temp = sub_word(temp);
        }
        w[i] = w[i - 8] ^ temp;
    }
    w
}

fn add_round_key(state: &mut [u32; 4], round_keys: &[u32; 60], round: usize) {
    for i in 0..4 {
        state[i] ^= round_keys[round * 4 + i];
    }
}

fn sub_bytes(state: &mut [u32; 4]) {
    for s in state.iter_mut() {
        *s = sub_byte(*s);
    }
}

fn shift_rows(state: &mut [u32; 4]) {
    let s0 = state[0];
    let s1 = state[1];
    let s2 = state[2];
    let s3 = state[3];

    let b0 = s0.to_le_bytes();
    let b1 = s1.to_le_bytes();
    let b2 = s2.to_le_bytes();
    let b3 = s3.to_le_bytes();

    state[0] = u32::from_le_bytes([b0[0], b1[1], b2[2], b3[3]]);
    state[1] = u32::from_le_bytes([b1[0], b2[1], b3[2], b0[3]]);
    state[2] = u32::from_le_bytes([b2[0], b3[1], b0[2], b1[3]]);
    state[3] = u32::from_le_bytes([b3[0], b0[1], b1[2], b2[3]]);
}

fn mix_columns(state: &mut [u32; 4]) {
    state[0] = xtime_column(state[0]);
    state[1] = xtime_column(state[1]);
    state[2] = xtime_column(state[2]);
    state[3] = xtime_column(state[3]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes256_known_vector() {
        let key: [u8; 32] = [0; 32];
        let mut plaintext: [u8; 16] = [0; 16];
        let expected: [u8; 16] = [
            0xDC, 0x95, 0xC0, 0x78, 0xA2, 0x40, 0x89, 0x89, 0xAD, 0x48, 0xA2, 0x14, 0x92, 0x84,
            0x20, 0x87,
        ];
        let cipher = Aes256::new(key);
        cipher.encrypt_block(&mut plaintext);
        assert_eq!(plaintext, expected);
    }

    #[test]
    fn test_aes256_roundtrip() {
        let key: [u8; 32] = [0x42; 32];
        let original: [u8; 16] = *b"ABCDEFGHIJKLMNOP";
        let mut block = original;
        let cipher = Aes256::new(key);
        cipher.encrypt_block(&mut block);
        assert_ne!(block, original);
    }
}
