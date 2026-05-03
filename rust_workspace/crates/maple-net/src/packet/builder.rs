// Packet builder - ported from tools/MaplePacketCreator.java (login packets)

use crate::packet::opcodes::SendOpcode;
use crate::packet::writer::PacketWriter;

pub fn build_hello(maple_version: u16, send_iv: &[u8; 4], recv_iv: &[u8; 4]) -> Vec<u8> {
    let mut w = PacketWriter::with_capacity(16);
    w.write_short(0x0E);
    w.write_short(maple_version as i16);
    w.write_short(1);
    w.write_byte(49);
    w.write_bytes(recv_iv);
    w.write_bytes(send_iv);
    w.write_byte(8);
    w.into_bytes()
}

pub fn build_ping() -> Vec<u8> {
    let mut w = PacketWriter::with_capacity(2);
    w.write_short(SendOpcode::Ping.value() as i16);
    w.into_bytes()
}

pub fn build_pong() -> Vec<u8> {
    // PONG is just a short opcode with no body - sent by client handler
    build_ping() // PING and PONG share the same opcode from server side
}

pub fn build_auth_success(
    account_id: i32,
    gender: u8,
    is_admin: bool,
    account_name: &str,
    enable_pin: bool,
    enable_pic: bool,
    has_pic: bool,
) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.write_short(SendOpcode::LoginStatus.value() as i16);
    w.write_int(0);
    w.write_short(0);
    w.write_int(account_id);
    w.write_byte(gender);
    w.write_bool(is_admin);
    w.write_byte(if is_admin { 0x80 } else { 0 });
    w.write_byte(0); // country code
    w.write_maple_ascii_string(account_name);
    w.write_byte(0);
    w.write_byte(0); // IsQuietBan
    w.write_long(0); // IsQuietBanTimeStamp
    w.write_long(0); // CreationTimeStamp
    w.write_int(1); // Remove "Select the world you want to play in"
    let pin_byte: u8 = if enable_pin { 0 } else { 1 };
    w.write_byte(pin_byte);
    let pic_byte: u8 = if !enable_pic {
        2
    } else if has_pic {
        1
    } else {
        0
    };
    w.write_byte(pic_byte);
    w.into_bytes()
}

pub fn build_login_failed(reason: i32) -> Vec<u8> {
    let mut w = PacketWriter::with_capacity(8);
    w.write_short(SendOpcode::LoginStatus.value() as i16);
    w.write_byte(reason as u8);
    w.write_byte(0);
    w.write_int(0);
    w.into_bytes()
}

pub fn build_server_list(
    server_id: i32,
    server_name: &str,
    flag: i32,
    event_msg: &str,
    channel_count: i32,
) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.write_short(SendOpcode::ServerList.value() as i16);
    w.write_byte(server_id as u8);
    w.write_maple_ascii_string(server_name);
    w.write_byte(flag as u8);
    w.write_maple_ascii_string(event_msg);
    w.write_byte(100); // rate modifier
    w.write_byte(0); // event xp * 2.6
    w.write_byte(100); // rate modifier
    w.write_byte(0); // drop rate * 2.6
    w.write_byte(0);
    w.write_byte(channel_count as u8);
    for ch in 1..=channel_count {
        let ch_name = format!("{}-{}", server_name, ch);
        w.write_maple_ascii_string(&ch_name);
        w.write_int(1000); // channel capacity
        w.write_byte(1); // nWorldID
        w.write_byte((ch - 1) as u8); // nChannelID
        w.write_bool(false); // bAdultChannel
    }
    w.write_short(0);
    w.into_bytes()
}

pub fn build_end_of_server_list() -> Vec<u8> {
    let mut w = PacketWriter::with_capacity(3);
    w.write_short(SendOpcode::ServerList.value() as i16);
    w.write_byte(0xFF);
    w.into_bytes()
}

pub fn build_select_world(world_id: i32) -> Vec<u8> {
    let mut w = PacketWriter::with_capacity(3);
    w.write_short(SendOpcode::LastConnectedWorld.value() as i16);
    w.write_int(world_id);
    w.into_bytes()
}

pub fn build_recommended(world_id: i32, msg: &str) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.write_short(SendOpcode::RecommendedWorldMessage.value() as i16);
    w.write_int(if world_id < 0 { 0 } else { 1 });
    if world_id >= 0 {
        w.write_int(world_id);
        w.write_maple_ascii_string(msg);
    }
    w.into_bytes()
}

pub fn build_char_list(
    chars: &[CharEntry],
    enable_pic: bool,
    has_pic: bool,
    char_slots: i32,
) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.write_short(SendOpcode::CharList.value() as i16);
    w.write_byte(0); // status
    w.write_byte(chars.len() as u8);
    for chr in chars {
        write_char_entry(&mut w, chr, false);
    }
    let pic_byte: u8 = if !enable_pic {
        2
    } else if has_pic {
        1
    } else {
        0
    };
    w.write_byte(pic_byte);
    w.write_int(char_slots);
    w.into_bytes()
}

pub fn build_server_ip(addr: [u8; 4], port: u16, client_id: i32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.write_short(SendOpcode::ServerIp.value() as i16);
    w.write_short(0);
    w.write_bytes(&addr);
    w.write_short(port as i16);
    w.write_int(client_id);
    w.write_bytes(&[0, 0, 0, 0, 0]);
    w.into_bytes()
}

pub fn build_server_status(status: i32) -> Vec<u8> {
    let mut w = PacketWriter::with_capacity(3);
    w.write_short(SendOpcode::ServerStatus.value() as i16);
    w.write_short(status as i16);
    w.into_bytes()
}

pub fn build_after_login_error(reason: i32) -> Vec<u8> {
    let mut w = PacketWriter::with_capacity(3);
    w.write_short(SendOpcode::ServerStatus.value() as i16);
    w.write_short(reason as i16);
    w.into_bytes()
}

#[derive(Debug, Clone)]
pub struct CharEntry {
    pub id: i32,
    pub name: String,
    pub gender: u8,
    pub skin_color: u8,
    pub face: i32,
    pub hair: i32,
    pub level: u8,
    pub job: i16,
    pub str: i16,
    pub dex: i16,
    pub int: i16,
    pub luk: i16,
    pub hp: i16,
    pub max_hp: i16,
    pub mp: i16,
    pub max_mp: i16,
    pub ap: i16,
    pub sp: i16,
    pub exp: i32,
    pub fame: i16,
    pub gacha_exp: i32,
    pub map_id: i32,
    pub spawn_point: u8,
    pub rank: i32,
    pub rank_move: i32,
    pub job_rank: i32,
    pub job_rank_move: i32,
    pub is_gm: bool,
}

fn write_char_entry(w: &mut PacketWriter, chr: &CharEntry, _viewall: bool) {
    write_char_stats(w, chr);
    write_char_look(w, chr);
    if !_viewall {
        w.write_byte(0);
    }
    if chr.is_gm {
        w.write_byte(0);
        return;
    }
    w.write_byte(1); // world rank enabled
    w.write_int(chr.rank);
    w.write_int(chr.rank_move);
    w.write_int(chr.job_rank);
    w.write_int(chr.job_rank_move);
}

fn write_char_stats(w: &mut PacketWriter, chr: &CharEntry) {
    w.write_int(chr.id);
    w.write_fixed_string(&chr.name, 13);
    w.write_byte(chr.gender);
    w.write_byte(chr.skin_color);
    w.write_int(chr.face);
    w.write_int(chr.hair);
    for _ in 0..3 {
        w.write_long(0); // pet IDs
    }
    w.write_byte(chr.level);
    w.write_short(chr.job);
    w.write_short(chr.str);
    w.write_short(chr.dex);
    w.write_short(chr.int);
    w.write_short(chr.luk);
    w.write_short(chr.hp);
    w.write_short(chr.max_hp);
    w.write_short(chr.mp);
    w.write_short(chr.max_mp);
    w.write_short(chr.ap);
    w.write_short(chr.sp);
    w.write_int(chr.exp);
    w.write_short(chr.fame);
    w.write_int(chr.gacha_exp);
    w.write_int(chr.map_id);
    w.write_byte(chr.spawn_point);
    w.write_int(0);
}

fn write_char_look(w: &mut PacketWriter, chr: &CharEntry) {
    w.write_byte(chr.gender);
    w.write_byte(chr.skin_color);
    w.write_int(chr.face);
    w.write_byte(1);
    w.write_int(chr.hair);
    w.write_byte(0xFF); // end of equips
    w.write_byte(0xFF); // end of masked equips
    w.write_int(0); // cash weapon
    for _ in 0..3 {
        w.write_int(0); // pet item IDs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_packet_length() {
        let send_iv = [0x01, 0x02, 0x03, 0x04];
        let recv_iv = [0x05, 0x06, 0x07, 0x08];
        let pkt = build_hello(83, &send_iv, &recv_iv);
        assert_eq!(pkt.len(), 16);
    }

    #[test]
    fn test_ping_packet() {
        let pkt = build_ping();
        assert_eq!(pkt, vec![0x11, 0x00]);
    }

    #[test]
    fn test_login_failed() {
        let pkt = build_login_failed(4);
        assert_eq!(pkt.len(), 8);
        assert_eq!(pkt[0], 0x00); // LOGIN_STATUS low byte
        assert_eq!(pkt[1], 0x00); // LOGIN_STATUS high byte
        assert_eq!(pkt[2], 4); // reason
    }

    #[test]
    fn test_server_list() {
        let pkt = build_server_list(0, "Scania", 0, "", 2);
        assert!(pkt.len() > 20);
        // Opcode check
        assert_eq!(pkt[0], 0x0A); // SERVERLIST low byte
        assert_eq!(pkt[1], 0x00);
    }

    #[test]
    fn test_server_ip() {
        let pkt = build_server_ip([127, 0, 0, 1], 7575, 1);
        assert_eq!(pkt[0], 0x0C); // SERVER_IP low byte
        assert_eq!(pkt[1], 0x00);
    }

    #[test]
    fn test_char_list_empty() {
        let pkt = build_char_list(&[], false, false, 3);
        assert_eq!(pkt[0], 0x0B); // CHARLIST opcode
        assert_eq!(pkt[1], 0x00);
        assert_eq!(pkt[2], 0x00); // status
        assert_eq!(pkt[3], 0x00); // 0 chars
    }

    #[test]
    fn test_char_list_with_char() {
        let chr = CharEntry {
            id: 1,
            name: "Test".to_string(),
            gender: 0,
            skin_color: 0,
            face: 20000,
            hair: 30000,
            level: 1,
            job: 0,
            str: 12,
            dex: 5,
            int: 4,
            luk: 4,
            hp: 50,
            max_hp: 50,
            mp: 5,
            max_mp: 5,
            ap: 0,
            sp: 0,
            exp: 0,
            fame: 0,
            gacha_exp: 0,
            map_id: 0,
            spawn_point: 0,
            rank: 1,
            rank_move: 0,
            job_rank: 1,
            job_rank_move: 0,
            is_gm: false,
        };
        let pkt = build_char_list(&[chr], false, false, 3);
        assert_eq!(pkt[0], 0x0B); // CHARLIST
        assert_eq!(pkt[3], 0x01); // 1 char
    }

    #[test]
    fn test_char_list_gm_has_no_rank_data() {
        let gm = CharEntry {
            id: 2,
            name: "GM".to_string(),
            gender: 0,
            skin_color: 0,
            face: 20000,
            hair: 30000,
            level: 200,
            job: 900,
            str: 32767,
            dex: 32767,
            int: 32767,
            luk: 32767,
            hp: 30000,
            max_hp: 30000,
            mp: 30000,
            max_mp: 30000,
            ap: 0,
            sp: 0,
            exp: 0,
            fame: 0,
            gacha_exp: 0,
            map_id: 0,
            spawn_point: 0,
            rank: 1,
            rank_move: 0,
            job_rank: 1,
            job_rank_move: 0,
            is_gm: true,
        };
        let pkt = build_char_list(&[gm], false, false, 3);
        assert_eq!(pkt[0], 0x0B); // CHARLIST
        assert_eq!(pkt[3], 0x01); // 1 char
                                  // GM char should be shorter (no 20 bytes of rank data)
        assert!(pkt.len() > 30);
    }
}
