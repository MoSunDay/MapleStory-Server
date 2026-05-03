// Database models - ported from db_database.sql schema

use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub pin: Option<String>,
    pub pic: Option<String>,
    pub loggedin: i8,
    pub lastlogin: Option<NaiveDateTime>,
    pub createdat: NaiveDateTime,
    pub birthday: NaiveDate,
    pub banned: bool,
    pub banreason: Option<String>,
    pub macs: Option<String>,
    pub nx_credit: i32,
    pub maple_point: i32,
    pub nx_prepaid: i32,
    pub characterslots: i8,
    pub gender: i8,
    pub tempban: NaiveDateTime,
    pub greason: i8,
    pub tos: bool,
    pub webadmin: i8,
    pub nick: Option<String>,
    pub mute: i8,
    pub email: Option<String>,
    pub rewardpoints: i32,
    pub votepoints: i32,
    pub hwid: String,
}

#[derive(Debug, Clone)]
pub struct Character {
    pub id: i32,
    pub accountid: i32,
    pub world: i32,
    pub name: String,
    pub level: i32,
    pub exp: i32,
    pub gachaexp: i32,
    pub str: i32,
    pub dex: i32,
    pub luk: i32,
    pub int: i32,
    pub hp: i32,
    pub mp: i32,
    pub maxhp: i32,
    pub maxmp: i32,
    pub meso: i32,
    pub hp_mp_used: i32,
    pub job: i32,
    pub skincolor: i32,
    pub gender: i32,
    pub fame: i32,
    pub fquest: i32,
    pub hair: i32,
    pub face: i32,
    pub ap: i32,
    pub sp: String,
    pub map: i32,
    pub spawnpoint: i32,
    pub gm: i8,
    pub party: i32,
    pub buddy_capacity: i32,
    pub createdate: NaiveDateTime,
    pub guildid: i32,
    pub guildrank: i32,
    pub messengerid: i32,
    pub messengerposition: i32,
    pub mountlevel: i32,
    pub mountexp: i32,
    pub mounttiredness: i32,
    pub equip_slots: i32,
    pub use_slots: i32,
    pub setup_slots: i32,
    pub etc_slots: i32,
    pub family_id: i32,
    pub monsterbookcover: i32,
    pub alliance_rank: i32,
    pub dojo_points: i32,
    pub last_dojo_stage: i32,
    pub partner_id: i32,
    pub marriage_item_id: i32,
    pub reborns: i32,
    pub pq_points: i32,
    pub pendant_exp: i8,
}

#[derive(Debug, Clone)]
pub struct Guild {
    pub id: i32,
    pub name: String,
    pub leader: i32,
    pub gp: i32,
    pub logo: i32,
    pub logo_color: i32,
    pub logo_bg: i32,
    pub logo_bg_color: i32,
    pub capacity: i32,
    pub notice: String,
    pub signature: i32,
    pub alliance_id: i32,
}

#[derive(Debug, Clone)]
pub struct Alliance {
    pub id: i32,
    pub name: String,
    pub capacity: i32,
    pub notice: String,
}

#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub id: i32,
    pub characterid: i32,
    pub itemid: i32,
    pub inventorytype: i8,
    pub position: i32,
    pub quantity: i32,
    pub owner: String,
    pub petid: i32,
    pub flag: i8,
    pub expiration: i64,
    pub gifts_from: String,
}

#[derive(Debug, Clone)]
pub struct InventoryEquipment {
    pub inventoryitemid: i64,
    pub upgradeslots: i32,
    pub level: i32,
    pub str: i32,
    pub dex: i32,
    pub int: i32,
    pub luk: i32,
    pub hp: i32,
    pub mp: i32,
    pub watk: i32,
    pub matk: i32,
    pub wdef: i32,
    pub mdef: i32,
    pub acc: i32,
    pub avoid: i32,
    pub hands: i32,
    pub speed: i32,
    pub jump: i32,
    pub locked: i32,
    pub vicious: i32,
    pub itemlevel: i32,
    pub itemexp: i32,
    pub ringid: i32,
}

#[derive(Debug, Clone)]
pub struct Skill {
    pub id: i32,
    pub characterid: i32,
    pub skillid: i32,
    pub skilllevel: i32,
    pub masterlevel: i32,
    pub expiration: i64,
}

#[derive(Debug, Clone)]
pub struct QuestStatus {
    pub questid: i32,
    pub characterid: i32,
    pub status: i32,
    pub time: i32,
    pub expires: i64,
    pub forfeited: i32,
    pub completed: i32,
}

#[derive(Debug, Clone)]
pub struct KeyMap {
    pub id: i32,
    pub characterid: i32,
    pub key: i32,
    pub key_type: i32,
    pub action: i32,
}

#[derive(Debug, Clone)]
pub struct Buddy {
    pub id: i32,
    pub characterid: i32,
    pub buddyid: i32,
    pub pending: i8,
    pub group: String,
}

#[derive(Debug, Clone)]
pub struct ShopItem {
    pub id: i32,
    pub shopid: i32,
    pub itemid: i32,
    pub price: i32,
    pub pitch: i32,
    pub position: i32,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            id: 0,
            accountid: 0,
            world: 0,
            name: String::new(),
            level: 1,
            exp: 0,
            gachaexp: 0,
            str: 12,
            dex: 5,
            luk: 4,
            int: 4,
            hp: 50,
            mp: 5,
            maxhp: 50,
            maxmp: 5,
            meso: 0,
            hp_mp_used: 0,
            job: 0,
            skincolor: 0,
            gender: 0,
            fame: 0,
            fquest: 0,
            hair: 0,
            face: 0,
            ap: 0,
            sp: String::from("0,0,0,0,0,0,0,0,0,0"),
            map: 0,
            spawnpoint: 0,
            gm: 0,
            party: 0,
            buddy_capacity: 25,
            createdate: NaiveDateTime::from_timestamp(0, 0),
            guildid: 0,
            guildrank: 5,
            messengerid: 0,
            messengerposition: 4,
            mountlevel: 1,
            mountexp: 0,
            mounttiredness: 0,
            equip_slots: 24,
            use_slots: 24,
            setup_slots: 24,
            etc_slots: 24,
            family_id: -1,
            monsterbookcover: 0,
            alliance_rank: 5,
            dojo_points: 0,
            last_dojo_stage: 0,
            partner_id: 0,
            marriage_item_id: 0,
            reborns: 0,
            pq_points: 0,
            pendant_exp: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_defaults() {
        let c = Character::default();
        assert_eq!(c.level, 1);
        assert_eq!(c.str, 12);
        assert_eq!(c.dex, 5);
        assert_eq!(c.maxhp, 50);
        assert_eq!(c.job, 0);
        assert_eq!(c.buddy_capacity, 25);
        assert_eq!(c.family_id, -1);
    }
}
