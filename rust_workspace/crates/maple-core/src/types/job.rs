#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MapleJob {
    Beginner,

    Warrior,
    Fighter,
    Crusader,
    Hero,
    Page,
    WhiteKnight,
    Paladin,
    Spearman,
    DragonKnight,
    DarkKnight,

    Magician,
    FpWizard,
    FpMage,
    FpArchMage,
    IlWizard,
    IlMage,
    IlArchMage,
    Cleric,
    Priest,
    Bishop,

    Bowman,
    Hunter,
    Ranger,
    Bowmaster,
    Crossbowman,
    Sniper,
    Marksman,

    Thief,
    Assassin,
    Hermit,
    NightLord,
    Bandit,
    ChiefBandit,
    Shadower,

    Pirate,
    Brawler,
    Marauder,
    Buccaneer,
    Gunslinger,
    Outlaw,
    Corsair,

    MapleleafBrigadier,
    Gm,
    SuperGm,

    Noblesse,
    DawnWarrior1,
    DawnWarrior2,
    DawnWarrior3,
    DawnWarrior4,
    BlazeWizard1,
    BlazeWizard2,
    BlazeWizard3,
    BlazeWizard4,
    WindArcher1,
    WindArcher2,
    WindArcher3,
    WindArcher4,
    NightWalker1,
    NightWalker2,
    NightWalker3,
    NightWalker4,
    ThunderBreaker1,
    ThunderBreaker2,
    ThunderBreaker3,
    ThunderBreaker4,

    Legend,
    Evan,
    Aran1,
    Aran2,
    Aran3,
    Aran4,
    Evan1,
    Evan2,
    Evan3,
    Evan4,
    Evan5,
    Evan6,
    Evan7,
    Evan8,
    Evan9,
    Evan10,
}

impl MapleJob {
    pub const MAX_ID: i32 = 22;

    pub fn id(self) -> i32 {
        match self {
            MapleJob::Beginner => 0,
            MapleJob::Warrior => 100,
            MapleJob::Fighter => 110,
            MapleJob::Crusader => 111,
            MapleJob::Hero => 112,
            MapleJob::Page => 120,
            MapleJob::WhiteKnight => 121,
            MapleJob::Paladin => 122,
            MapleJob::Spearman => 130,
            MapleJob::DragonKnight => 131,
            MapleJob::DarkKnight => 132,
            MapleJob::Magician => 200,
            MapleJob::FpWizard => 210,
            MapleJob::FpMage => 211,
            MapleJob::FpArchMage => 212,
            MapleJob::IlWizard => 220,
            MapleJob::IlMage => 221,
            MapleJob::IlArchMage => 222,
            MapleJob::Cleric => 230,
            MapleJob::Priest => 231,
            MapleJob::Bishop => 232,
            MapleJob::Bowman => 300,
            MapleJob::Hunter => 310,
            MapleJob::Ranger => 311,
            MapleJob::Bowmaster => 312,
            MapleJob::Crossbowman => 320,
            MapleJob::Sniper => 321,
            MapleJob::Marksman => 322,
            MapleJob::Thief => 400,
            MapleJob::Assassin => 410,
            MapleJob::Hermit => 411,
            MapleJob::NightLord => 412,
            MapleJob::Bandit => 420,
            MapleJob::ChiefBandit => 421,
            MapleJob::Shadower => 422,
            MapleJob::Pirate => 500,
            MapleJob::Brawler => 510,
            MapleJob::Marauder => 511,
            MapleJob::Buccaneer => 512,
            MapleJob::Gunslinger => 520,
            MapleJob::Outlaw => 521,
            MapleJob::Corsair => 522,
            MapleJob::MapleleafBrigadier => 800,
            MapleJob::Gm => 900,
            MapleJob::SuperGm => 910,
            MapleJob::Noblesse => 1000,
            MapleJob::DawnWarrior1 => 1100,
            MapleJob::DawnWarrior2 => 1110,
            MapleJob::DawnWarrior3 => 1111,
            MapleJob::DawnWarrior4 => 1112,
            MapleJob::BlazeWizard1 => 1200,
            MapleJob::BlazeWizard2 => 1210,
            MapleJob::BlazeWizard3 => 1211,
            MapleJob::BlazeWizard4 => 1212,
            MapleJob::WindArcher1 => 1300,
            MapleJob::WindArcher2 => 1310,
            MapleJob::WindArcher3 => 1311,
            MapleJob::WindArcher4 => 1312,
            MapleJob::NightWalker1 => 1400,
            MapleJob::NightWalker2 => 1410,
            MapleJob::NightWalker3 => 1411,
            MapleJob::NightWalker4 => 1412,
            MapleJob::ThunderBreaker1 => 1500,
            MapleJob::ThunderBreaker2 => 1510,
            MapleJob::ThunderBreaker3 => 1511,
            MapleJob::ThunderBreaker4 => 1512,
            MapleJob::Legend => 2000,
            MapleJob::Evan => 2001,
            MapleJob::Aran1 => 2100,
            MapleJob::Aran2 => 2110,
            MapleJob::Aran3 => 2111,
            MapleJob::Aran4 => 2112,
            MapleJob::Evan1 => 2200,
            MapleJob::Evan2 => 2210,
            MapleJob::Evan3 => 2211,
            MapleJob::Evan4 => 2212,
            MapleJob::Evan5 => 2213,
            MapleJob::Evan6 => 2214,
            MapleJob::Evan7 => 2215,
            MapleJob::Evan8 => 2216,
            MapleJob::Evan9 => 2217,
            MapleJob::Evan10 => 2218,
        }
    }

    pub fn from_id(id: i32) -> Option<MapleJob> {
        Some(match id {
            0 => MapleJob::Beginner,
            100 => MapleJob::Warrior,
            110 => MapleJob::Fighter,
            111 => MapleJob::Crusader,
            112 => MapleJob::Hero,
            120 => MapleJob::Page,
            121 => MapleJob::WhiteKnight,
            122 => MapleJob::Paladin,
            130 => MapleJob::Spearman,
            131 => MapleJob::DragonKnight,
            132 => MapleJob::DarkKnight,
            200 => MapleJob::Magician,
            210 => MapleJob::FpWizard,
            211 => MapleJob::FpMage,
            212 => MapleJob::FpArchMage,
            220 => MapleJob::IlWizard,
            221 => MapleJob::IlMage,
            222 => MapleJob::IlArchMage,
            230 => MapleJob::Cleric,
            231 => MapleJob::Priest,
            232 => MapleJob::Bishop,
            300 => MapleJob::Bowman,
            310 => MapleJob::Hunter,
            311 => MapleJob::Ranger,
            312 => MapleJob::Bowmaster,
            320 => MapleJob::Crossbowman,
            321 => MapleJob::Sniper,
            322 => MapleJob::Marksman,
            400 => MapleJob::Thief,
            410 => MapleJob::Assassin,
            411 => MapleJob::Hermit,
            412 => MapleJob::NightLord,
            420 => MapleJob::Bandit,
            421 => MapleJob::ChiefBandit,
            422 => MapleJob::Shadower,
            500 => MapleJob::Pirate,
            510 => MapleJob::Brawler,
            511 => MapleJob::Marauder,
            512 => MapleJob::Buccaneer,
            520 => MapleJob::Gunslinger,
            521 => MapleJob::Outlaw,
            522 => MapleJob::Corsair,
            800 => MapleJob::MapleleafBrigadier,
            900 => MapleJob::Gm,
            910 => MapleJob::SuperGm,
            1000 => MapleJob::Noblesse,
            1100 => MapleJob::DawnWarrior1,
            1110 => MapleJob::DawnWarrior2,
            1111 => MapleJob::DawnWarrior3,
            1112 => MapleJob::DawnWarrior4,
            1200 => MapleJob::BlazeWizard1,
            1210 => MapleJob::BlazeWizard2,
            1211 => MapleJob::BlazeWizard3,
            1212 => MapleJob::BlazeWizard4,
            1300 => MapleJob::WindArcher1,
            1310 => MapleJob::WindArcher2,
            1311 => MapleJob::WindArcher3,
            1312 => MapleJob::WindArcher4,
            1400 => MapleJob::NightWalker1,
            1410 => MapleJob::NightWalker2,
            1411 => MapleJob::NightWalker3,
            1412 => MapleJob::NightWalker4,
            1500 => MapleJob::ThunderBreaker1,
            1510 => MapleJob::ThunderBreaker2,
            1511 => MapleJob::ThunderBreaker3,
            1512 => MapleJob::ThunderBreaker4,
            2000 => MapleJob::Legend,
            2001 => MapleJob::Evan,
            2100 => MapleJob::Aran1,
            2110 => MapleJob::Aran2,
            2111 => MapleJob::Aran3,
            2112 => MapleJob::Aran4,
            2200 => MapleJob::Evan1,
            2210 => MapleJob::Evan2,
            2211 => MapleJob::Evan3,
            2212 => MapleJob::Evan4,
            2213 => MapleJob::Evan5,
            2214 => MapleJob::Evan6,
            2215 => MapleJob::Evan7,
            2216 => MapleJob::Evan8,
            2217 => MapleJob::Evan9,
            2218 => MapleJob::Evan10,
            _ => return None,
        })
    }

    pub fn from_5_byte_encoding(encoded: i32) -> MapleJob {
        match encoded {
            2 => MapleJob::Warrior,
            4 => MapleJob::Magician,
            8 => MapleJob::Bowman,
            16 => MapleJob::Thief,
            32 => MapleJob::Pirate,
            1024 => MapleJob::Noblesse,
            2048 => MapleJob::DawnWarrior1,
            4096 => MapleJob::BlazeWizard1,
            8192 => MapleJob::WindArcher1,
            16384 => MapleJob::NightWalker1,
            32768 => MapleJob::ThunderBreaker1,
            _ => MapleJob::Beginner,
        }
    }

    pub fn is_beginner_job(beginners: MapleJob) -> bool {
        matches!(
            beginners,
            MapleJob::Warrior
                | MapleJob::Magician
                | MapleJob::Bowman
                | MapleJob::Thief
                | MapleJob::Pirate
                | MapleJob::Aran1
                | MapleJob::ThunderBreaker1
                | MapleJob::DawnWarrior1
                | MapleJob::NightWalker1
                | MapleJob::BlazeWizard1
        )
    }

    pub fn is_a(self, basejob: MapleJob) -> bool {
        let basebranch = basejob.id() / 10;
        (self.id() / 10 == basebranch && self.id() >= basejob.id())
            || (basebranch % 10 == 0 && self.id() / 100 == basejob.id() / 100)
    }

    pub fn job_niche(self) -> i32 {
        (self.id() / 100) % 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_roundtrip() {
        let cases = [
            MapleJob::Beginner,
            MapleJob::Hero,
            MapleJob::Bishop,
            MapleJob::NightLord,
            MapleJob::Corsair,
            MapleJob::Noblesse,
            MapleJob::DawnWarrior4,
            MapleJob::Aran4,
            MapleJob::Evan10,
        ];
        for job in cases {
            assert_eq!(MapleJob::from_id(job.id()), Some(job));
        }
    }

    #[test]
    fn test_invalid_id() {
        assert_eq!(MapleJob::from_id(999), None);
        assert_eq!(MapleJob::from_id(-1), None);
    }

    #[test]
    fn test_5byte_encoding() {
        assert_eq!(MapleJob::from_5_byte_encoding(2), MapleJob::Warrior);
        assert_eq!(MapleJob::from_5_byte_encoding(32), MapleJob::Pirate);
        assert_eq!(MapleJob::from_5_byte_encoding(999), MapleJob::Beginner);
    }

    #[test]
    fn test_is_a() {
        assert!(MapleJob::Hero.is_a(MapleJob::Fighter));
        assert!(MapleJob::Crusader.is_a(MapleJob::Fighter));
        assert!(MapleJob::Fighter.is_a(MapleJob::Fighter));
        assert!(!MapleJob::Hero.is_a(MapleJob::Page));
    }

    #[test]
    fn test_job_niche() {
        assert_eq!(MapleJob::Beginner.job_niche(), 0);
        assert_eq!(MapleJob::Warrior.job_niche(), 1);
        assert_eq!(MapleJob::Magician.job_niche(), 2);
        assert_eq!(MapleJob::Bowman.job_niche(), 3);
        assert_eq!(MapleJob::Thief.job_niche(), 4);
        assert_eq!(MapleJob::Pirate.job_niche(), 5);
    }
}
