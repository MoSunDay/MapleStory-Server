// Ported from net/mina/MapleCustomEncryption.java

fn roll_left(input: u8, count: u8) -> u8 {
    let tmp = input as i32 & 0xFF;
    let tmp = tmp << (count % 8);
    ((tmp & 0xFF) | (tmp >> 8)) as u8
}

fn roll_right(input: u8, count: u8) -> u8 {
    let tmp = input as i32 & 0xFF;
    let tmp = (tmp << 8) >> (count % 8) as i32;
    ((tmp & 0xFF) | (tmp >> 8)) as u8
}

pub fn encrypt_data(data: &mut [u8]) {
    for j in 0..6 {
        let mut remember: u8 = 0;
        let mut data_length: u8 = (data.len() & 0xFF) as u8;
        if j % 2 == 0 {
            for i in 0..data.len() {
                let mut cur = data[i];
                cur = roll_left(cur, 3);
                cur = cur.wrapping_add(data_length);
                cur ^= remember;
                remember = cur;
                cur = roll_right(cur, data_length & 0xFF);
                cur = (!cur) & 0xFF;
                cur = cur.wrapping_add(0x48);
                data_length = data_length.wrapping_sub(1);
                data[i] = cur;
            }
        } else {
            for i in (0..data.len()).rev() {
                let mut cur = data[i];
                cur = roll_left(cur, 4);
                cur = cur.wrapping_add(data_length);
                cur ^= remember;
                remember = cur;
                cur ^= 0x13;
                cur = roll_right(cur, 3);
                data_length = data_length.wrapping_sub(1);
                data[i] = cur;
            }
        }
    }
}

pub fn decrypt_data(data: &mut [u8]) {
    for j in 1u32..=6 {
        let mut remember: u8 = 0;
        let mut data_length: u8 = (data.len() & 0xFF) as u8;
        if j % 2 == 0 {
            for i in 0..data.len() {
                let mut cur = data[i];
                cur = cur.wrapping_sub(0x48);
                cur = (!cur) & 0xFF;
                cur = roll_left(cur, data_length & 0xFF);
                let next_remember = cur;
                cur ^= remember;
                remember = next_remember;
                cur = cur.wrapping_sub(data_length);
                cur = roll_right(cur, 3);
                data[i] = cur;
                data_length = data_length.wrapping_sub(1);
            }
        } else {
            for i in (0..data.len()).rev() {
                let mut cur = data[i];
                cur = roll_left(cur, 3);
                cur ^= 0x13;
                let next_remember = cur;
                cur ^= remember;
                remember = next_remember;
                cur = cur.wrapping_sub(data_length);
                cur = roll_right(cur, 4);
                data[i] = cur;
                data_length = data_length.wrapping_sub(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let original: Vec<u8> = (0..64u8).collect();
        let mut data = original.clone();
        encrypt_data(&mut data);
        assert_ne!(data, original);
        decrypt_data(&mut data);
        assert_eq!(data, original);
    }

    #[test]
    fn test_encrypt_empty() {
        let mut data: Vec<u8> = vec![];
        encrypt_data(&mut data);
        assert_eq!(data.len(), 0);
    }

    #[test]
    fn test_encrypt_single_byte() {
        let mut data = vec![42u8];
        encrypt_data(&mut data);
        assert_ne!(data[0], 42);
        decrypt_data(&mut data);
        assert_eq!(data[0], 42);
    }

    #[test]
    fn test_roll_operations() {
        assert_eq!(roll_left(0x01, 1), 0x02);
        assert_eq!(roll_right(0x02, 1), 0x01);
        assert_eq!(roll_left(0x80, 1), 0x01);
        assert_eq!(roll_right(0x01, 1), 0x80);
    }
}
