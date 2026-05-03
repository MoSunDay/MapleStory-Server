// Packet reader - ported from tools/data/input/GenericLittleEndianAccessor.java

pub struct PacketReader {
    buf: Vec<u8>,
    pos: usize,
}

impl PacketReader {
    pub fn new(data: Vec<u8>) -> Self {
        PacketReader { buf: data, pos: 0 }
    }

    pub fn read_byte(&mut self) -> u8 {
        let b = self.buf[self.pos];
        self.pos += 1;
        b
    }

    pub fn read_short(&mut self) -> i16 {
        let lo = self.read_byte() as i16;
        let hi = self.read_byte() as i16;
        (lo & 0xFF) | ((hi & 0xFF) << 8)
    }

    pub fn read_int(&mut self) -> i32 {
        let b0 = self.read_byte() as i32;
        let b1 = self.read_byte() as i32;
        let b2 = self.read_byte() as i32;
        let b3 = self.read_byte() as i32;
        (b0 & 0xFF) | ((b1 & 0xFF) << 8) | ((b2 & 0xFF) << 16) | ((b3 & 0xFF) << 24)
    }

    pub fn read_long(&mut self) -> i64 {
        let b0 = self.read_byte() as i64;
        let b1 = self.read_byte() as i64;
        let b2 = self.read_byte() as i64;
        let b3 = self.read_byte() as i64;
        let b4 = self.read_byte() as i64;
        let b5 = self.read_byte() as i64;
        let b6 = self.read_byte() as i64;
        let b7 = self.read_byte() as i64;
        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24) | (b4 << 32) | (b5 << 40) | (b6 << 48) | (b7 << 56)
    }

    pub fn read_maple_ascii_string(&mut self) -> String {
        let len = self.read_short() as usize;
        let bytes: Vec<u8> = (0..len).map(|_| self.read_byte()).collect();
        String::from_utf8_lossy(&bytes).to_string()
    }

    pub fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let bytes: Vec<u8> = (0..n).map(|_| self.read_byte()).collect();
        bytes
    }

    pub fn skip(&mut self, n: usize) {
        self.pos += n;
    }

    pub fn available(&self) -> usize {
        self.buf.len() - self.pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_short() {
        let mut r = PacketReader::new(vec![0x02, 0x01]);
        assert_eq!(r.read_short(), 0x0102);
    }

    #[test]
    fn test_read_int() {
        let mut r = PacketReader::new(vec![0x04, 0x03, 0x02, 0x01]);
        assert_eq!(r.read_int(), 0x01020304);
    }

    #[test]
    fn test_read_maple_string() {
        let mut r = PacketReader::new(vec![0x02, 0x00, b'A', b'B']);
        assert_eq!(r.read_maple_ascii_string(), "AB");
    }

    #[test]
    fn test_packet_roundtrip() {
        // Write
        let mut w = crate::packet::writer::PacketWriter::new();
        w.write_short(0x42);
        w.write_maple_ascii_string("Hello");
        w.write_int(12345);
        let data = w.into_bytes();

        // Read
        let mut r = PacketReader::new(data);
        assert_eq!(r.read_short(), 0x42);
        assert_eq!(r.read_maple_ascii_string(), "Hello");
        assert_eq!(r.read_int(), 12345);
    }
}
