// Session management and handler dispatch
// Ported from net/MapleServerHandler.java + net/PacketProcessor.java

use crate::codec::MapleCodec;
use crate::packet::builder;
use crate::packet::opcodes::RecvOpcode;
use crate::packet::reader::PacketReader;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type LoginHandlerFn = Arc<dyn Fn(&Session, &[u8]) -> Option<Vec<u8>> + Send + Sync>;

pub struct HandlerRegistry {
    login_handlers: HashMap<u16, LoginHandlerFn>,
}

impl HandlerRegistry {
    pub fn new() -> Self {
        HandlerRegistry {
            login_handlers: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, opcode: RecvOpcode, handler: F)
    where
        F: Fn(&Session, &[u8]) -> Option<Vec<u8>> + Send + Sync + 'static,
    {
        self.login_handlers
            .insert(opcode.value(), Arc::new(handler));
    }

    pub fn get_handler(&self, opcode: u16) -> Option<&LoginHandlerFn> {
        self.login_handlers.get(&opcode)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionState {
    HelloSent,
    LoggedIn,
    ServerTransition,
}

pub struct Session {
    pub account_id: i32,
    pub account_name: String,
    pub gender: u8,
    pub gm_level: i32,
    pub state: SessionState,
    pub world: i32,
    pub channel: i32,
    pub pin: Option<String>,
    pub pic: Option<String>,
}

impl Session {
    pub fn new() -> Self {
        Session {
            account_id: -1,
            account_name: String::new(),
            gender: 0,
            gm_level: 0,
            state: SessionState::HelloSent,
            world: -1,
            channel: -1,
            pin: None,
            pic: None,
        }
    }

    pub fn is_logged_in(&self) -> bool {
        self.state == SessionState::LoggedIn
    }
}

pub struct ClientConnection {
    pub session: Session,
    pub codec: MapleCodec,
}

impl ClientConnection {
    pub fn new(send_iv: [u8; 4], recv_iv: [u8; 4], version: u16) -> Self {
        ClientConnection {
            session: Session::new(),
            codec: MapleCodec::new(send_iv, recv_iv, version),
        }
    }

    pub fn build_hello(&self, version: u16) -> Vec<u8> {
        // IVs are embedded in the codec; the hello packet just sends them as-is
        let send_iv = [82u8, 48, 120, 115];
        let recv_iv = [70u8, 114, 122, 82];
        let mut iv_send = send_iv;
        let mut iv_recv = recv_iv;
        iv_send[3] = (rand_u8()) as u8;
        iv_recv[3] = (rand_u8()) as u8;
        builder::build_hello(version, &iv_send, &iv_recv)
    }
}

fn rand_u8() -> u8 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos % 256) as u8
}

pub type ConnectionMap = Arc<Mutex<HashMap<u64, ClientConnection>>>;

pub struct LoginServer {
    pub connections: ConnectionMap,
    pub registry: HandlerRegistry,
    pub next_id: u64,
}

impl LoginServer {
    pub fn new() -> Self {
        LoginServer {
            connections: Arc::new(Mutex::new(HashMap::new())),
            registry: HandlerRegistry::new(),
            next_id: 0,
        }
    }
}

pub fn create_login_registry() -> HandlerRegistry {
    let mut registry = HandlerRegistry::new();

    // PONG handler
    registry.register(RecvOpcode::Pong, |session, _data| {
        // Client heartbeat received; no response needed
        let _ = session;
        None
    });

    // LoginPassword handler (stub - will be implemented in maple-game)
    registry.register(RecvOpcode::LoginPassword, |session, data| {
        let mut reader = PacketReader::new(data.to_vec());
        reader.skip(2); // skip opcode (already consumed)
        let login = reader.read_maple_ascii_string();
        let _pwd = reader.read_maple_ascii_string();

        let state = &session;
        if state.is_logged_in() {
            return Some(builder::build_login_failed(7));
        }

        // Stub: accept any login in development mode
        // TODO: Implement real auth via maple-db
        let response = builder::build_auth_success(1, 0, false, &login, false, false, false);
        Some(response)
    });

    // Serverlist request handler
    registry.register(RecvOpcode::ServerlistRequest, |session, _data| {
        let _ = session;
        let response = builder::build_server_list(0, "Scania", 0, "", 2);
        Some(response)
    });

    // Charlist request handler
    registry.register(RecvOpcode::CharlistRequest, |session, data| {
        let mut reader = PacketReader::new(data.to_vec());
        reader.skip(2); // skip opcode
        reader.read_byte(); // mode
        let _world = reader.read_byte();
        let _channel = reader.read_byte();

        let _ = session;
        let chars: Vec<builder::CharEntry> = vec![];
        Some(builder::build_char_list(&chars, false, false, 3))
    });

    registry
}
