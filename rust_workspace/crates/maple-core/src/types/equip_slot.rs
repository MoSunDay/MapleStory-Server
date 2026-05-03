#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EquipSlot {
    Hat,
    SpecialHat,
    FaceAccessory,
    EyeAccessory,
    Earrings,
    Top,
    Overall,
    Pants,
    Shoes,
    Gloves,
    CashGloves,
    Cape,
    Shield,
    Weapon,
    Weapon2,
    LowWeapon,
    Ring,
    Pendant,
    TamedMob,
    Saddle,
    Medal,
    Belt,
    PetEquip,
}

impl EquipSlot {
    pub fn wz_name(&self) -> Option<&'static str> {
        match self {
            EquipSlot::Hat => Some("Cp"),
            EquipSlot::SpecialHat => Some("HrCp"),
            EquipSlot::FaceAccessory => Some("Af"),
            EquipSlot::EyeAccessory => Some("Ay"),
            EquipSlot::Earrings => Some("Ae"),
            EquipSlot::Top => Some("Ma"),
            EquipSlot::Overall => Some("MaPn"),
            EquipSlot::Pants => Some("Pn"),
            EquipSlot::Shoes => Some("So"),
            EquipSlot::Gloves => Some("GlGw"),
            EquipSlot::CashGloves => Some("Gv"),
            EquipSlot::Cape => Some("Sr"),
            EquipSlot::Shield => Some("Si"),
            EquipSlot::Weapon => Some("Wp"),
            EquipSlot::Weapon2 => Some("WpSi"),
            EquipSlot::LowWeapon => Some("WpSp"),
            EquipSlot::Ring => Some("Ri"),
            EquipSlot::Pendant => Some("Pe"),
            EquipSlot::TamedMob => Some("Tm"),
            EquipSlot::Saddle => Some("Sd"),
            EquipSlot::Medal => Some("Me"),
            EquipSlot::Belt => Some("Be"),
            EquipSlot::PetEquip => None,
        }
    }

    pub fn allowed_slots(&self) -> &'static [i32] {
        match self {
            EquipSlot::Ring => &[-12, -13, -15, -16],
            EquipSlot::Hat => &[-1],
            EquipSlot::FaceAccessory => &[-2],
            EquipSlot::EyeAccessory => &[-3],
            EquipSlot::Earrings => &[-4],
            EquipSlot::Top => &[-5],
            EquipSlot::Overall => &[-5],
            EquipSlot::Pants => &[-6],
            EquipSlot::Shoes => &[-7],
            EquipSlot::Gloves => &[-8],
            EquipSlot::CashGloves => &[-8],
            EquipSlot::Cape => &[-9],
            EquipSlot::Shield => &[-10],
            EquipSlot::Weapon => &[-11],
            EquipSlot::Weapon2 => &[-11],
            EquipSlot::LowWeapon => &[-11],
            EquipSlot::Pendant => &[-17],
            EquipSlot::TamedMob => &[-18],
            EquipSlot::Saddle => &[-19],
            EquipSlot::Medal => &[-49],
            EquipSlot::Belt => &[-50],
            EquipSlot::SpecialHat => &[-1],
            EquipSlot::PetEquip => &[],
        }
    }

    pub fn is_allowed(&self, slot: i32, cash: bool) -> bool {
        if slot < 0 {
            let allowed = self.allowed_slots();
            if !allowed.is_empty() {
                return allowed.iter().any(|&allow| {
                    let condition = if cash { allow - 100 } else { allow };
                    slot == condition
                });
            }
        }
        cash && slot < 0
    }
}

pub fn equip_slot_from_text(slot: &str) -> EquipSlot {
    if slot.is_empty() {
        return EquipSlot::PetEquip;
    }
    match slot {
        "Cp" => EquipSlot::Hat,
        "HrCp" => EquipSlot::SpecialHat,
        "Af" => EquipSlot::FaceAccessory,
        "Ay" => EquipSlot::EyeAccessory,
        "Ae" => EquipSlot::Earrings,
        "Ma" => EquipSlot::Top,
        "MaPn" => EquipSlot::Overall,
        "Pn" => EquipSlot::Pants,
        "So" => EquipSlot::Shoes,
        "GlGw" => EquipSlot::Gloves,
        "Gv" => EquipSlot::CashGloves,
        "Sr" => EquipSlot::Cape,
        "Si" => EquipSlot::Shield,
        "Wp" => EquipSlot::Weapon,
        "WpSi" => EquipSlot::Weapon2,
        "WpSp" => EquipSlot::LowWeapon,
        "Ri" => EquipSlot::Ring,
        "Pe" => EquipSlot::Pendant,
        "Tm" => EquipSlot::TamedMob,
        "Sd" => EquipSlot::Saddle,
        "Me" => EquipSlot::Medal,
        "Be" => EquipSlot::Belt,
        _ => EquipSlot::PetEquip,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_allowed_slots() {
        let ring = EquipSlot::Ring;
        assert!(ring.is_allowed(-12, false));
        assert!(ring.is_allowed(-13, false));
        assert!(ring.is_allowed(-112, true));
        assert!(!ring.is_allowed(-1, false));
    }

    #[test]
    fn test_hat_allowed_slots() {
        let hat = EquipSlot::Hat;
        assert!(hat.is_allowed(-1, false));
        assert!(hat.is_allowed(-101, true));
        assert!(!hat.is_allowed(-2, false));
    }

    #[test]
    fn test_from_text() {
        assert_eq!(equip_slot_from_text("Cp"), EquipSlot::Hat);
        assert_eq!(equip_slot_from_text("Ri"), EquipSlot::Ring);
        assert_eq!(equip_slot_from_text(""), EquipSlot::PetEquip);
        assert_eq!(equip_slot_from_text("???"), EquipSlot::PetEquip);
    }
}
