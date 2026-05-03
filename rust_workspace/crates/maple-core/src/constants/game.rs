// Ported from constants/GameConstants.java

use crate::types::job::MapleJob;

pub const WORLD_NAMES: &[&str] = &["Scania", "Bera", "Aurora", "Elysium", "Reboot"];

pub const OWL_DATA: [i32; 10] = [
    1082002, 2070005, 2070006, 1022047, 1102041, 2044705, 2340000, 2040017, 1092030, 2040804,
];

pub const CASH_DATA: [i32; 5] = [50200004, 50200069, 50200117, 50100008, 50000047];

pub fn goto_town_map(town: &str) -> Option<i32> {
    match town {
        "southperry" => Some(60000),
        "amherst" => Some(1000000),
        "henesys" => Some(100000000),
        "ellinia" => Some(101000000),
        "perion" => Some(102000000),
        "kerning" => Some(103000000),
        "lith" => Some(104000000),
        "sleepywood" => Some(105040300),
        "florina" => Some(110000000),
        "nautilus" => Some(120000000),
        "ereve" => Some(130000000),
        "rien" => Some(140000000),
        "orbis" => Some(200000000),
        "happy" => Some(209000000),
        "elnath" => Some(211000000),
        "ludi" => Some(220000000),
        "aqua" => Some(230000000),
        "leafre" => Some(240000000),
        "mulung" => Some(250000000),
        "herb" => Some(251000000),
        "omega" => Some(221000000),
        "korean" => Some(222000000),
        "ellin" => Some(300000000),
        "nlc" => Some(600000000),
        "showa" => Some(801000000),
        "shrine" => Some(800000000),
        "ariant" => Some(260000000),
        "magatia" => Some(261000000),
        "singapore" => Some(540000000),
        "quay" => Some(541000000),
        "kampung" => Some(551000000),
        "amoria" => Some(680000000),
        "temple" => Some(270000100),
        "square" => Some(103040000),
        "neo" => Some(240070000),
        "mushking" => Some(106020000),
        _ => None,
    }
}

pub fn goto_area_map(area: &str) -> Option<i32> {
    match area {
        "gmmap" => Some(180000000),
        "excavation" => Some(990000000),
        "mushmom" => Some(100000005),
        "griffey" => Some(240020101),
        "manon" => Some(240020401),
        "horseman" => Some(682000001),
        "balrog" => Some(105090900),
        "zakum" => Some(211042300),
        "papu" => Some(220080001),
        "guild" => Some(200000301),
        "skelegon" => Some(240040511),
        "hpq" => Some(100000200),
        "pianus" => Some(230040420),
        "horntail" => Some(240050400),
        "pinkbean" => Some(270050000),
        "keep" => Some(610020006),
        "dojo" => Some(925020001),
        "bosspq" => Some(970030000),
        "fm" => Some(910000000),
        other => goto_town_map(other),
    }
}

pub const DROP_RATE_GAIN: [i32; 14] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
pub const MESO_RATE_GAIN: [i32; 14] = [1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105];
pub const EXP_RATE_GAIN: [i32; 14] = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610];

pub const JOB_UPGRADE_BLOB: [i32; 5] = [1, 20, 60, 110, 190];
pub const JOB_UPGRADE_SP_UP: [i32; 5] = [0, 1, 2, 3, 6];

pub const DEFAULT_KEY: [i32; 40] = [
    18, 65, 2, 23, 3, 4, 5, 6, 16, 17, 19, 25, 26, 27, 31, 34, 35, 37, 38, 40, 43, 44, 45, 46, 50,
    56, 59, 60, 61, 62, 63, 64, 57, 48, 29, 7, 24, 33, 41, 39,
];
pub const DEFAULT_TYPE: [i32; 40] = [
    4, 6, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 4, 4, 5, 6, 6, 6, 6, 6, 6,
    5, 4, 5, 4, 4, 4, 4, 4,
];
pub const DEFAULT_ACTION: [i32; 40] = [
    0, 106, 10, 1, 12, 13, 18, 24, 8, 5, 4, 19, 14, 15, 2, 17, 11, 3, 20, 16, 9, 50, 51, 6, 7, 53,
    100, 101, 102, 103, 104, 105, 54, 22, 52, 21, 25, 26, 23, 27,
];

pub const CUSTOM_KEY: [i32; 39] = [
    18, 65, 2, 23, 3, 4, 5, 6, 16, 17, 19, 25, 26, 27, 31, 34, 35, 37, 38, 40, 43, 44, 45, 46, 50,
    56, 59, 60, 61, 62, 63, 64, 57, 48, 29, 7, 24, 41, 39,
];
pub const CUSTOM_TYPE: [i32; 39] = [
    4, 6, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 4, 4, 5, 6, 6, 6, 6, 6, 6,
    5, 4, 5, 4, 4, 4, 4,
];
pub const CUSTOM_ACTION: [i32; 39] = [
    0, 106, 10, 1, 12, 13, 18, 23, 8, 5, 4, 19, 14, 15, 2, 17, 11, 3, 20, 16, 9, 50, 51, 6, 7, 53,
    100, 101, 102, 103, 104, 105, 54, 42, 52, 1001, 27, 22, 26,
];

pub const MOB_HP_VAL: [i32; 200] = [
    // Indices 0-19
    0, 15, 20, 25, 35, 50, 65, 80, 95, 110, 125, 150, 175, 200, 225, 250, 275, 300, 325, 350,
    // Indices 20-39
    375, 405, 435, 465, 495, 525, 580, 650, 720, 790, 900, 990, 1100, 1200, 1300, 1400, 1500, 1600,
    1700, 1800, // Indices 40-59
    1900, 2000, 2100, 2200, 2300, 2400, 2520, 2640, 2760, 2880, 3000, 3200, 3400, 3600, 3800, 4000,
    4300, 4600, 4900, 5200, // Indices 60-79
    5500, 5900, 6300, 6700, 7100, 7500, 8000, 8500, 9000, 9500, 10000, 11000, 12000, 13000, 14000,
    15000, 17000, 19000, 21000, 23000, // Indices 80-99
    25000, 27000, 29000, 31000, 33000, 35000, 37000, 39000, 41000, 43000, 45000, 47000, 49000,
    51000, 53000, 55000, 57000, 59000, 61000, 63000, // Indices 100-119
    65000, 67000, 69000, 71000, 73000, 75000, 77000, 79000, 81000, 83000, 85000, 87000, 89000,
    91000, 93000, 95000, 97000, 99000, 101000, 103000, // Indices 120-139
    105000, 107000, 109000, 111000, 113000, 115000, 118000, 120000, 125000, 130000, 135000, 140000,
    145000, 150000, 155000, 160000, 165000, 170000, 175000, 180000, // Indices 140-159
    185000, 190000, 195000, 200000, 205000, 210000, 215000, 220000, 225000, 230000, 235000, 240000,
    250000, 260000, 270000, 280000, 290000, 300000, 310000, 320000, // Indices 160-179
    330000, 340000, 350000, 360000, 370000, 380000, 390000, 400000, 410000, 420000, 430000, 440000,
    450000, 460000, 470000, 480000, 490000, 500000, 510000, 520000, // Indices 180-199
    530000, 550000, 570000, 590000, 610000, 630000, 650000, 670000, 690000, 710000, 730000, 750000,
    770000, 790000, 810000, 830000, 850000, 870000, 890000, 910000,
];

pub fn player_bonus_drop_rate(slot: usize) -> i32 {
    DROP_RATE_GAIN[slot]
}

pub fn player_bonus_meso_rate(slot: usize) -> i32 {
    MESO_RATE_GAIN[slot]
}

pub fn player_bonus_exp_rate(slot: usize) -> i32 {
    EXP_RATE_GAIN[slot]
}

pub fn get_custom_key(custom_keyset: bool) -> &'static [i32] {
    if custom_keyset {
        &CUSTOM_KEY
    } else {
        &DEFAULT_KEY
    }
}

pub fn get_custom_type(custom_keyset: bool) -> &'static [i32] {
    if custom_keyset {
        &CUSTOM_TYPE
    } else {
        &DEFAULT_TYPE
    }
}

pub fn get_custom_action(custom_keyset: bool) -> &'static [i32] {
    if custom_keyset {
        &CUSTOM_ACTION
    } else {
        &DEFAULT_ACTION
    }
}

pub fn format_job_name(job_id: i32) -> String {
    if let Some(job) = MapleJob::from_id(job_id) {
        let name = format!("{:?}", job);
        name.chars()
            .take(1)
            .chain(name.chars().skip(1).map(|c| c.to_ascii_lowercase()))
            .collect::<String>()
            .replace(|c: char| c.is_ascii_digit(), "")
    } else {
        String::new()
    }
}

pub fn job_upgrade_level_range(jobbranch: usize) -> i32 {
    JOB_UPGRADE_BLOB[jobbranch]
}

pub fn change_job_sp_upgrade(jobbranch: usize) -> i32 {
    JOB_UPGRADE_SP_UP[jobbranch]
}

pub fn is_hall_of_fame_map(mapid: i32) -> bool {
    matches!(
        mapid,
        102000004
            | 101000004
            | 100000204
            | 103000008
            | 120000105
            | 130000100
            | 130000101
            | 130000110
            | 130000120
            | 140010110
    )
}

pub fn is_podium_hall_of_fame_map(mapid: i32) -> bool {
    matches!(
        mapid,
        102000004 | 101000004 | 100000204 | 103000008 | 120000105
    )
}

pub fn get_hall_of_fame_branch(job: MapleJob, mapid: i32) -> u8 {
    if !is_hall_of_fame_map(mapid) {
        return (26 + 4 * (mapid / 100000000)) as u8;
    }
    if job.is_a(MapleJob::Warrior) {
        10
    } else if job.is_a(MapleJob::Magician) {
        11
    } else if job.is_a(MapleJob::Bowman) {
        12
    } else if job.is_a(MapleJob::Thief) {
        13
    } else if job.is_a(MapleJob::Pirate) {
        14
    } else if job.is_a(MapleJob::DawnWarrior1) {
        15
    } else if job.is_a(MapleJob::BlazeWizard1) {
        16
    } else if job.is_a(MapleJob::WindArcher1) {
        17
    } else if job.is_a(MapleJob::NightWalker1) {
        18
    } else if job.is_a(MapleJob::ThunderBreaker1) {
        19
    } else if job.is_a(MapleJob::Aran1) {
        20
    } else if job.is_a(MapleJob::Evan1) {
        21
    } else if job.is_a(MapleJob::Beginner) {
        22
    } else if job.is_a(MapleJob::Noblesse) {
        23
    } else if job.is_a(MapleJob::Legend) {
        24
    } else {
        25
    }
}

pub fn overall_job_rank_by_script_id(script_id: i32) -> i32 {
    let branch = (script_id / 100) % 100;
    if branch < 26 {
        (script_id % 100) + 1
    } else {
        ((script_id - 2600) % 400) + 1
    }
}

pub fn can_pnpc_branch_use_script_id(branch: u8, script_id: i32) -> bool {
    let branch_check = (script_id / 100) % 100;
    if branch < 26 {
        branch as i32 == branch_check
    } else {
        branch_check >= branch as i32 && branch_check < branch as i32 + 4
    }
}

pub fn get_hall_of_fame_mapid(job: MapleJob) -> i32 {
    let jobid = job.id();
    if is_cygnus(jobid) {
        130000100
    } else if is_aran(jobid) {
        140010110
    } else if job.is_a(MapleJob::Warrior) {
        102000004
    } else if job.is_a(MapleJob::Magician) {
        101000004
    } else if job.is_a(MapleJob::Bowman) {
        100000204
    } else if job.is_a(MapleJob::Thief) {
        103000008
    } else if job.is_a(MapleJob::Pirate) {
        120000105
    } else {
        130000110
    }
}

pub fn get_job_branch(job: MapleJob) -> i32 {
    let jobid = job.id();
    if jobid % 1000 == 0 {
        0
    } else if jobid % 100 == 0 {
        1
    } else {
        2 + (jobid % 10)
    }
}

pub fn get_job_max_level(job: MapleJob) -> i32 {
    match get_job_branch(job) {
        0 => 10,
        1 => 30,
        2 => 70,
        3 => 120,
        _ => {
            if job.id() / 1000 == 1 {
                120
            } else {
                200
            }
        }
    }
}

pub fn is_cygnus(job: i32) -> bool {
    job / 1000 == 1
}

pub fn is_aran(job: i32) -> bool {
    job == 2000 || (job >= 2100 && job <= 2112)
}

fn is_in_branch_job_tree(skill_job_id: i32, job_id: i32, branch_type: u32) -> bool {
    let branch = 10i32.pow(branch_type);
    let skill_branch = (skill_job_id / branch) * branch;
    let job_branch = (job_id / branch) * branch;
    skill_branch == job_branch
}

fn has_diverged_branch_job_tree(skill_job_id: i32, job_id: i32, branch_type: u32) -> bool {
    let branch = 10i32.pow(branch_type);
    let skill_branch = skill_job_id / branch;
    let job_branch = job_id / branch;
    skill_branch != job_branch && skill_branch % 10 != 0
}

pub fn is_in_job_tree(skill_id: i32, job_id: i32) -> bool {
    let skill_job = skill_id / 10000;
    if !is_in_branch_job_tree(skill_job, job_id, 0) {
        for i in 1..=3 {
            if has_diverged_branch_job_tree(skill_job, job_id, i) {
                return false;
            }
            if is_in_branch_job_tree(skill_job, job_id, i) {
                return skill_job <= job_id;
            }
        }
    } else {
        return skill_job <= job_id;
    }
    false
}

pub fn is_pq_skill(skill: i32) -> bool {
    (skill >= 20000014 && skill <= 20000018)
        || skill == 10000013
        || skill == 20001013
        || (skill % 10000000 >= 1009 && skill % 10000000 <= 1011)
        || skill % 10000000 == 1020
}

pub fn is_gm_skills(skill: i32) -> bool {
    (skill >= 9001000 && skill <= 9101008) || (skill >= 8001000 && skill <= 8001001)
}

pub fn is_free_market_room(mapid: i32) -> bool {
    mapid > 910000000 && mapid < 910000023
}

pub fn is_boss_rush(mapid: i32) -> bool {
    mapid >= 970030100 && mapid <= 970042711
}

pub fn is_dojo(mapid: i32) -> bool {
    mapid >= 925020000 && mapid < 925040000
}

pub fn is_pyramid(mapid: i32) -> bool {
    mapid >= 926010010 && mapid <= 930010000
}

pub fn is_pq_skill_map(mapid: i32) -> bool {
    is_dojo(mapid) || is_pyramid(mapid)
}

pub fn is_fishing_area(mapid: i32) -> bool {
    mapid == 120010000 || mapid == 251000100 || mapid == 541010110
}

pub fn is_finisher_skill(skill_id: i32) -> bool {
    (skill_id > 1111002 && skill_id < 1111007) || skill_id == 11111002 || skill_id == 11111003
}

pub fn has_sp_table(job: MapleJob) -> bool {
    matches!(
        job,
        MapleJob::Evan
            | MapleJob::Evan1
            | MapleJob::Evan2
            | MapleJob::Evan3
            | MapleJob::Evan4
            | MapleJob::Evan5
            | MapleJob::Evan6
            | MapleJob::Evan7
            | MapleJob::Evan8
            | MapleJob::Evan9
            | MapleJob::Evan10
    )
}

pub fn get_monster_hp(level: i32) -> i32 {
    if level < 0 || level as usize >= MOB_HP_VAL.len() {
        i32::MAX
    } else {
        MOB_HP_VAL[level as usize]
    }
}

pub fn ordinal(i: i32) -> String {
    let suffixes: [&str; 10] = ["th", "st", "nd", "rd", "th", "th", "th", "th", "th", "th"];
    match i % 100 {
        11 | 12 | 13 => format!("{i}th"),
        _ => format!("{i}{}", suffixes[(i % 10) as usize]),
    }
}

pub fn get_skill_book(job: i32) -> i32 {
    if job >= 2210 && job <= 2218 {
        job - 2209
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goto_town() {
        assert_eq!(goto_town_map("henesys"), Some(100000000));
        assert_eq!(goto_town_map("nonexistent"), None);
    }

    #[test]
    fn test_goto_area_includes_towns() {
        assert_eq!(goto_area_map("henesys"), Some(100000000));
        assert_eq!(goto_area_map("zakum"), Some(211042300));
    }

    #[test]
    fn test_rate_gains() {
        assert_eq!(player_bonus_drop_rate(0), 1);
        assert_eq!(player_bonus_exp_rate(1), 2);
        assert_eq!(player_bonus_exp_rate(4), 8);
    }

    #[test]
    fn test_is_cygnus() {
        assert!(is_cygnus(1100));
        assert!(!is_cygnus(100));
    }

    #[test]
    fn test_is_hall_of_fame_map() {
        assert!(is_hall_of_fame_map(102000004));
        assert!(!is_hall_of_fame_map(100000000));
    }

    #[test]
    fn test_ordinal() {
        assert_eq!(ordinal(1), "1st");
        assert_eq!(ordinal(2), "2nd");
        assert_eq!(ordinal(3), "3rd");
        assert_eq!(ordinal(4), "4th");
        assert_eq!(ordinal(11), "11th");
        assert_eq!(ordinal(21), "21st");
    }

    #[test]
    fn test_get_monster_hp() {
        assert_eq!(get_monster_hp(0), 0);
        assert_eq!(get_monster_hp(1), 15);
        assert_eq!(get_monster_hp(100), 65000);
        assert_eq!(get_monster_hp(199), 910000);
        assert_eq!(get_monster_hp(250), i32::MAX);
    }
}
