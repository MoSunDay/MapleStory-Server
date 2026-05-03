pub mod codec;
pub mod handler;
pub mod packet;

pub use codec::MapleCodec;
pub use handler::{create_login_registry, ClientConnection, HandlerRegistry, LoginServer, Session};

use crate::packet::reader::PacketReader;

pub fn dispatch_packet(data: &[u8], session: &Session, registry: &HandlerRegistry) -> Option<Vec<u8>> {
    if data.len() < 2 {
        return None;
    }
    let opcode = u16::from_le_bytes([data[0], data[1]]);
    if let Some(handler) = registry.get_handler(opcode) {
        handler(session, data)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::builder;
    use crate::packet::opcodes::RecvOpcode;

    #[test]
    fn test_dispatch_unknown_opcode() {
        let registry = HandlerRegistry::new();
        let session = Session::new();
        let data = vec![0xFF, 0xFF, 0x00];
        assert!(dispatch_packet(&data, &session, &registry).is_none());
    }

    #[test]
    fn test_dispatch_pong() {
        let mut registry = HandlerRegistry::new();
        registry.register(RecvOpcode::Pong, |_, _| None);
        let session = Session::new();
        let data = vec![0x18, 0x00];
        assert!(dispatch_packet(&data, &session, &registry).is_none());
    }

    #[test]
    fn test_dispatch_login_password_builds_auth() {
        let registry = create_login_registry();
        let session = Session::new();

        let mut w = crate::packet::writer::PacketWriter::new();
        w.write_short(RecvOpcode::LoginPassword.value() as i16);
        w.write_maple_ascii_string("testuser");
        w.write_maple_ascii_string("testpass");
        let data = w.into_bytes();

        let response = dispatch_packet(&data, &session, &registry);
        assert!(response.is_some());
        let resp = response.unwrap();
        assert_eq!(resp[0], 0x00);
        assert_eq!(resp[1], 0x00); // LOGIN_STATUS
    }

    #[test]
    fn test_dispatch_serverlist_request() {
        let registry = create_login_registry();
        let session = Session::new();
        let data = vec![0x0B, 0x00];
        let response = dispatch_packet(&data, &session, &registry);
        assert!(response.is_some());
        let resp = response.unwrap();
        assert_eq!(resp[0], 0x0A); // SERVERLIST
        assert_eq!(resp[1], 0x00);
    }
}
