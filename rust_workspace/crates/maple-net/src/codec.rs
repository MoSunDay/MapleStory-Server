// Packet codec: encode/decode with AES-OFB + custom encryption
// Ported from net/mina/MaplePacketEncoder.java + MaplePacketDecoder.java

use bytes::{Buf, BufMut, BytesMut};
use maple_crypto::aes_ofb::MapleAesOfb;
use maple_crypto::custom_enc;
use tokio_util::codec::{Decoder, Encoder};

pub struct MapleCodec {
    send_crypto: MapleAesOfb,
    recv_crypto: MapleAesOfb,
}

impl MapleCodec {
    pub fn new(send_iv: [u8; 4], recv_iv: [u8; 4], version: u16) -> Self {
        MapleCodec {
            send_crypto: MapleAesOfb::new(send_iv, version ^ 0xFFFF),
            recv_crypto: MapleAesOfb::new(recv_iv, version),
        }
    }

    pub fn recv_crypto_mut(&mut self) -> &mut MapleAesOfb {
        &mut self.recv_crypto
    }
}

impl Decoder for MapleCodec {
    type Item = Vec<u8>;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            return Ok(None);
        }

        // Parse 4-byte header to get packet length
        let header = [src[0], src[1], src[2], src[3]];
        let header_i32 = i32::from_be_bytes(header);

        if !self.recv_crypto.check_packet_header(header_i32) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Packet header validation failed",
            ));
        }

        let packet_length = MapleAesOfb::get_packet_length(header_i32);

        if packet_length < 0 || packet_length > 65536 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid packet length: {}", packet_length),
            ));
        }

        let total_needed = 4 + packet_length as usize;
        if src.len() < total_needed {
            src.reserve(total_needed - src.len());
            return Ok(None);
        }

        src.advance(4);
        let mut body = src.split_to(packet_length as usize).to_vec();

        // Decrypt: reverse order of encoder
        self.recv_crypto.crypt(&mut body);
        custom_enc::decrypt_data(&mut body);

        Ok(Some(body))
    }
}

impl Encoder<Vec<u8>> for MapleCodec {
    type Error = std::io::Error;

    fn encode(&mut self, data: Vec<u8>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let len = data.len();
        let header = self.send_crypto.get_packet_header(len);

        let mut body = data;
        custom_enc::encrypt_data(&mut body);
        self.send_crypto.crypt(&mut body);

        dst.reserve(4 + body.len());
        dst.put_slice(&header);
        dst.put_slice(&body);

        Ok(())
    }
}
