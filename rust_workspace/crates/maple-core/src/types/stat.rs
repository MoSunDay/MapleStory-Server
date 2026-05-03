#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MapleStat {
    Skin,
    Face,
    Hair,
    Level,
    Job,
    Str,
    Dex,
    Int,
    Luk,
    Hp,
    MaxHp,
    Mp,
    MaxMp,
    AvailableAp,
    AvailableSp,
    Exp,
    Fame,
    Meso,
    Pet,
    GachaExp,
}

impl MapleStat {
    pub fn value(self) -> i32 {
        match self {
            MapleStat::Skin => 0x1,
            MapleStat::Face => 0x2,
            MapleStat::Hair => 0x4,
            MapleStat::Level => 0x10,
            MapleStat::Job => 0x20,
            MapleStat::Str => 0x40,
            MapleStat::Dex => 0x80,
            MapleStat::Int => 0x100,
            MapleStat::Luk => 0x200,
            MapleStat::Hp => 0x400,
            MapleStat::MaxHp => 0x800,
            MapleStat::Mp => 0x1000,
            MapleStat::MaxMp => 0x2000,
            MapleStat::AvailableAp => 0x4000,
            MapleStat::AvailableSp => 0x8000,
            MapleStat::Exp => 0x10000,
            MapleStat::Fame => 0x20000,
            MapleStat::Meso => 0x40000,
            MapleStat::Pet => 0x180008,
            MapleStat::GachaExp => 0x200000,
        }
    }

    pub fn from_value(value: i32) -> Option<MapleStat> {
        Some(match value {
            0x1 => MapleStat::Skin,
            0x2 => MapleStat::Face,
            0x4 => MapleStat::Hair,
            0x10 => MapleStat::Level,
            0x20 => MapleStat::Job,
            0x40 => MapleStat::Str,
            0x80 => MapleStat::Dex,
            0x100 => MapleStat::Int,
            0x200 => MapleStat::Luk,
            0x400 => MapleStat::Hp,
            0x800 => MapleStat::MaxHp,
            0x1000 => MapleStat::Mp,
            0x2000 => MapleStat::MaxMp,
            0x4000 => MapleStat::AvailableAp,
            0x8000 => MapleStat::AvailableSp,
            0x10000 => MapleStat::Exp,
            0x20000 => MapleStat::Fame,
            0x40000 => MapleStat::Meso,
            0x180008 => MapleStat::Pet,
            0x200000 => MapleStat::GachaExp,
            _ => return None,
        })
    }

    pub fn from_5_byte_encoding(encoded: i32) -> Option<MapleStat> {
        match encoded {
            64 => Some(MapleStat::Str),
            128 => Some(MapleStat::Dex),
            256 => Some(MapleStat::Int),
            512 => Some(MapleStat::Luk),
            _ => None,
        }
    }

    pub fn from_str(type_name: &str) -> Option<MapleStat> {
        match type_name {
            "SKIN" => Some(MapleStat::Skin),
            "FACE" => Some(MapleStat::Face),
            "HAIR" => Some(MapleStat::Hair),
            "LEVEL" => Some(MapleStat::Level),
            "JOB" => Some(MapleStat::Job),
            "STR" => Some(MapleStat::Str),
            "DEX" => Some(MapleStat::Dex),
            "INT" => Some(MapleStat::Int),
            "LUK" => Some(MapleStat::Luk),
            "HP" => Some(MapleStat::Hp),
            "MAXHP" => Some(MapleStat::MaxHp),
            "MP" => Some(MapleStat::Mp),
            "MAXMP" => Some(MapleStat::MaxMp),
            "AVAILABLEAP" => Some(MapleStat::AvailableAp),
            "AVAILABLESP" => Some(MapleStat::AvailableSp),
            "EXP" => Some(MapleStat::Exp),
            "FAME" => Some(MapleStat::Fame),
            "MESO" => Some(MapleStat::Meso),
            "PET" => Some(MapleStat::Pet),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_roundtrip() {
        let cases = [
            MapleStat::Skin,
            MapleStat::Level,
            MapleStat::Str,
            MapleStat::Hp,
            MapleStat::Exp,
            MapleStat::Pet,
        ];
        for stat in cases {
            assert_eq!(MapleStat::from_value(stat.value()), Some(stat));
        }
    }

    #[test]
    fn test_invalid_value() {
        assert_eq!(MapleStat::from_value(0xDEAD), None);
    }

    #[test]
    fn test_5byte_encoding() {
        assert_eq!(MapleStat::from_5_byte_encoding(64), Some(MapleStat::Str));
        assert_eq!(MapleStat::from_5_byte_encoding(128), Some(MapleStat::Dex));
        assert_eq!(MapleStat::from_5_byte_encoding(999), None);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(MapleStat::from_str("SKIN"), Some(MapleStat::Skin));
        assert_eq!(MapleStat::from_str("MESO"), Some(MapleStat::Meso));
        assert_eq!(MapleStat::from_str("UNKNOWN"), None);
    }
}
