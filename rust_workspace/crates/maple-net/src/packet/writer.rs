// Ported from tools/data/output/GenericLittleEndianWriter.java + MaplePacketLittleEndianWriter.java

pub struct PacketWriter {
    buf: Vec<u8>,
}

impl PacketWriter {
    pub fn new() -> Self {
        Self::with_capacity(32)
    }

    pub fn with_capacity(cap: usize) -> Self {
        PacketWriter {
            buf: Vec::with_capacity(cap),
        }
    }

    pub fn write_byte(&mut self, b: u8) {
        self.buf.push(b);
    }

    pub fn write_bytes(&mut self, b: &[u8]) {
        self.buf.extend_from_slice(b);
    }

    pub fn write_short(&mut self, s: i16) {
        let s = s as u16;
        self.buf.push((s & 0xFF) as u8);
        self.buf.push(((s >> 8) & 0xFF) as u8);
    }

    pub fn write_int(&mut self, i: i32) {
        let i = i as u32;
        self.buf.push((i & 0xFF) as u8);
        self.buf.push(((i >> 8) & 0xFF) as u8);
        self.buf.push(((i >> 16) & 0xFF) as u8);
        self.buf.push(((i >> 24) & 0xFF) as u8);
    }

    pub fn write_long(&mut self, l: i64) {
        let l = l as u64;
        self.buf.push((l & 0xFF) as u8);
        self.buf.push(((l >> 8) & 0xFF) as u8);
        self.buf.push(((l >> 16) & 0xFF) as u8);
        self.buf.push(((l >> 24) & 0xFF) as u8);
        self.buf.push(((l >> 32) & 0xFF) as u8);
        self.buf.push(((l >> 40) & 0xFF) as u8);
        self.buf.push(((l >> 48) & 0xFF) as u8);
        self.buf.push(((l >> 56) & 0xFF) as u8);
    }

    pub fn write_bool(&mut self, b: bool) {
        self.write_byte(if b { 1 } else { 0 });
    }

    pub fn write_maple_ascii_string(&mut self, s: &str) {
        self.write_short(s.len() as i16);
        self.write_bytes(s.as_bytes());
    }

    pub fn write_pos(&mut self, x: i16, y: i16) {
        self.write_short(x);
        self.write_short(y);
    }

    pub fn write_fixed_string(&mut self, s: &str, len: usize) {
        let bytes = s.as_bytes();
        let copy_len = bytes.len().min(len);
        self.write_bytes(&bytes[..copy_len]);
        for _ in copy_len..len {
            self.write_byte(0);
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_short() {
        let mut w = PacketWriter::new();
        w.write_short(0x0102);
        assert_eq!(w.into_bytes(), vec![0x02, 0x01]);
    }

    #[test]
    fn test_write_int() {
        let mut w = PacketWriter::new();
        w.write_int(0x01020304);
        assert_eq!(w.into_bytes(), vec![0x04, 0x03, 0x02, 0x01]);
    }

    #[test]
    fn test_write_maple_string() {
        let mut w = PacketWriter::new();
        w.write_maple_ascii_string("AB");
        assert_eq!(w.into_bytes(), vec![0x02, 0x00, b'A', b'B']);
    }

    #[test]
    fn test_hello_packet() {
        let mut w = PacketWriter::new();
        w.write_short(0x0E);
        w.write_short(83);
        w.write_short(1);
        w.write_byte(49);
        let recv_iv = [0x01u8, 0x02, 0x03, 0x04];
        let send_iv = [0x05u8, 0x06, 0x07, 0x08];
        w.write_bytes(&recv_iv);
        w.write_bytes(&send_iv);
        w.write_byte(8);
        assert_eq!(w.len(), 16);
    }
}
