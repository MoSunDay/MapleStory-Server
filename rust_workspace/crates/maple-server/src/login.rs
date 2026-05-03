// MapleStory-Server Rust - Login Server
// Ported from net/server/Server.java login path

use maple_crypto::aes_ofb::MapleAesOfb;
use maple_crypto::custom_enc;
use maple_net::handler::{create_login_registry, HandlerRegistry, Session};
use maple_net::packet::builder;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::{info, warn};

const SERVER_VERSION: u16 = 83;

struct ClientState {
    send_crypto: Mutex<MapleAesOfb>,
    recv_crypto: Mutex<MapleAesOfb>,
    session: Mutex<Session>,
    registry: Arc<HandlerRegistry>,
}

impl ClientState {
    fn new(send_iv: [u8; 4], recv_iv: [u8; 4], registry: Arc<HandlerRegistry>) -> Self {
        ClientState {
            send_crypto: Mutex::new(MapleAesOfb::new(send_iv, SERVER_VERSION ^ 0xFFFF)),
            recv_crypto: Mutex::new(MapleAesOfb::new(recv_iv, SERVER_VERSION)),
            session: Mutex::new(Session::new()),
            registry,
        }
    }

    async fn send_packet(&self, data: &[u8], stream: &mut (impl AsyncWriteExt + Unpin)) -> Result<(), std::io::Error> {
        let mut body = data.to_vec();
        let mut send = self.send_crypto.lock().await;
        let header = send.get_packet_header(body.len());
        custom_enc::encrypt_data(&mut body);
        send.crypt(&mut body);

        let mut framed = Vec::with_capacity(4 + body.len());
        framed.extend_from_slice(&header);
        framed.extend_from_slice(&body);

        stream.write_all(&framed).await
    }

    async fn recv_packet(&self, stream: &mut (impl AsyncReadExt + Unpin)) -> Result<Vec<u8>, std::io::Error> {
        let mut header = [0u8; 4];
        stream.read_exact(&mut header).await?;

        let header_i32 = i32::from_be_bytes(header);

        {
            let recv = self.recv_crypto.lock().await;
            if !recv.check_packet_header(header_i32) {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Packet header validation failed"));
            }
        }

        let packet_length = MapleAesOfb::get_packet_length(header_i32);

        if packet_length < 0 || packet_length > 65536 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad packet length"));
        }

        let mut body = vec![0u8; packet_length as usize];
        stream.read_exact(&mut body).await?;

        let mut recv = self.recv_crypto.lock().await;
        recv.crypt(&mut body);
        custom_enc::decrypt_data(&mut body);

        Ok(body)
    }

    async fn dispatch(&self, data: &[u8]) -> Option<Vec<u8>> {
        if data.len() < 2 {
            return None;
        }
        let opcode = u16::from_le_bytes([data[0], data[1]]);
        let session = self.session.lock().await;
        if let Some(handler) = self.registry.get_handler(opcode) {
            handler(&session, data)
        } else {
            None
        }
    }
}

pub async fn run_login_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8484").await?;
    info!("Login server listening on 0.0.0.0:8484");

    let registry = Arc::new(create_login_registry());

    loop {
        let (mut stream, addr) = listener.accept().await?;
        info!("New connection from {}", addr);

        let registry = registry.clone();

        tokio::spawn(async move {
            // Generate IVs with independent random last bytes
            let mut send_iv = [82u8, 48, 120, 115];
            let mut recv_iv = [70u8, 114, 122, 82];
            {
                let dur = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap();
                let nanos = dur.subsec_nanos();
                // Use different bit ranges for statistical independence
                send_iv[3] = (nanos % 256) as u8;
                recv_iv[3] = ((nanos.wrapping_mul(1103515245).wrapping_add(12345)) % 256) as u8;
            }

            let client = ClientState::new(send_iv, recv_iv, registry);

            // Send Hello
            let hello = builder::build_hello(SERVER_VERSION, &send_iv, &recv_iv);
            if let Err(e) = client.send_packet(&hello, &mut stream).await {
                warn!("Failed to send hello to {}: {}", addr, e);
                return;
            }

            // Read/process loop
            loop {
                match client.recv_packet(&mut stream).await {
                    Ok(data) => {
                        if let Some(response) = client.dispatch(&data).await {
                            if let Err(e) = client.send_packet(&response, &mut stream).await {
                                warn!("Failed to send response to {}: {}", addr, e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        if e.kind() != std::io::ErrorKind::UnexpectedEof {
                            warn!("Read error from {}: {}", addr, e);
                        }
                        break;
                    }
                }
            }

            info!("Connection from {} closed", addr);
        });
    }
}
