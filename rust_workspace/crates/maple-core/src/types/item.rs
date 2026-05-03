#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InventoryType {
    Undefined,
    Equip,
    Use,
    Setup,
    Etc,
    Cash,
    CanHold,
    Equipped,
}

impl InventoryType {
    pub fn type_id(self) -> i8 {
        match self {
            InventoryType::Undefined => 0,
            InventoryType::Equip => 1,
            InventoryType::Use => 2,
            InventoryType::Setup => 3,
            InventoryType::Etc => 4,
            InventoryType::Cash => 5,
            InventoryType::CanHold => 6,
            InventoryType::Equipped => -1,
        }
    }

    pub fn bitfield_encoding(self) -> i16 {
        (2i16 << self.type_id()) as i16
    }

    pub fn from_type(type_id: i8) -> Option<InventoryType> {
        Some(match type_id {
            0 => InventoryType::Undefined,
            1 => InventoryType::Equip,
            2 => InventoryType::Use,
            3 => InventoryType::Setup,
            4 => InventoryType::Etc,
            5 => InventoryType::Cash,
            6 => InventoryType::CanHold,
            -1 => InventoryType::Equipped,
            _ => return None,
        })
    }

    pub fn from_wz_name(name: &str) -> InventoryType {
        match name {
            "Install" => InventoryType::Setup,
            "Consume" => InventoryType::Use,
            "Etc" => InventoryType::Etc,
            "Cash" => InventoryType::Cash,
            "Pet" => InventoryType::Cash,
            _ => InventoryType::Undefined,
        }
    }

    pub fn from_item_id(item_id: i32) -> InventoryType {
        let type_id = (item_id / 1000000) as i8;
        if (1..=5).contains(&type_id) {
            InventoryType::from_type(type_id).unwrap_or(InventoryType::Undefined)
        } else {
            InventoryType::Undefined
        }
    }
}

pub mod flags {
    pub const LOCK: i32 = 0x01;
    pub const SPIKES: i32 = 0x02;
    pub const KARMA_USE: i32 = 0x02;
    pub const COLD: i32 = 0x04;
    pub const UNTRADEABLE: i32 = 0x08;
    pub const KARMA_EQP: i32 = 0x10;
    pub const SANDBOX: i32 = 0x40;
    pub const PET_COME: i32 = 0x80;
    pub const ACCOUNT_SHARING: i32 = 0x100;
    pub const MERGE_UNTRADEABLE: i32 = 0x200;
}

pub struct ItemFlags;

impl ItemFlags {
    pub fn flag_by_int(flag_type: i32) -> i32 {
        match flag_type {
            128 => flags::PET_COME,
            256 => flags::ACCOUNT_SHARING,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_type_from_type() {
        assert_eq!(InventoryType::from_type(0), Some(InventoryType::Undefined));
        assert_eq!(InventoryType::from_type(1), Some(InventoryType::Equip));
        assert_eq!(InventoryType::from_type(5), Some(InventoryType::Cash));
        assert_eq!(InventoryType::from_type(-1), Some(InventoryType::Equipped));
        assert_eq!(InventoryType::from_type(99), None);
    }

    #[test]
    fn test_from_wz_name() {
        assert_eq!(InventoryType::from_wz_name("Install"), InventoryType::Setup);
        assert_eq!(InventoryType::from_wz_name("Consume"), InventoryType::Use);
        assert_eq!(InventoryType::from_wz_name("Pet"), InventoryType::Cash);
        assert_eq!(
            InventoryType::from_wz_name("Unknown"),
            InventoryType::Undefined
        );
    }

    #[test]
    fn test_from_item_id() {
        assert_eq!(InventoryType::from_item_id(1302000), InventoryType::Equip);
        assert_eq!(InventoryType::from_item_id(2000000), InventoryType::Use);
        assert_eq!(InventoryType::from_item_id(5000000), InventoryType::Cash);
    }
}
