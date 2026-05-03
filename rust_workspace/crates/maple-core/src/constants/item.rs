// Ported from constants/ItemConstants.java

use crate::types::item::InventoryType;

pub const EXPIRING_ITEMS: bool = true;

pub const PERMANENT_ITEM_IDS: &[i32] = &[5000060, 5000100, 5000101, 5000102];

pub fn is_permanent_item(item_id: i32) -> bool {
    PERMANENT_ITEM_IDS.contains(&item_id)
}

pub fn is_throwing_star(item_id: i32) -> bool {
    item_id / 10000 == 207
}

pub fn is_bullet(item_id: i32) -> bool {
    item_id / 10000 == 233
}

pub fn is_potion(item_id: i32) -> bool {
    item_id / 1000 == 2000
}

pub fn is_food(item_id: i32) -> bool {
    let use_type = item_id / 1000;
    use_type == 2022 || use_type == 2010 || use_type == 2020
}

pub fn is_consumable(item_id: i32) -> bool {
    is_potion(item_id) || is_food(item_id)
}

pub fn is_rechargeable(item_id: i32) -> bool {
    is_throwing_star(item_id) || is_bullet(item_id)
}

pub fn is_arrow_for_crossbow(item_id: i32) -> bool {
    item_id / 1000 == 2061
}

pub fn is_arrow_for_bow(item_id: i32) -> bool {
    item_id / 1000 == 2060
}

pub fn is_arrow(item_id: i32) -> bool {
    is_arrow_for_bow(item_id) || is_arrow_for_crossbow(item_id)
}

pub fn is_pet(item_id: i32) -> bool {
    item_id / 1000 == 5000
}

pub fn is_new_year_card_etc(item_id: i32) -> bool {
    item_id / 10000 == 430
}

pub fn is_new_year_card_use(item_id: i32) -> bool {
    item_id / 10000 == 216
}

pub fn is_accessory(item_id: i32) -> bool {
    item_id >= 1110000 && item_id < 1140000
}

pub fn is_taming(item_id: i32) -> bool {
    let item_type = item_id / 1000;
    item_type == 1902 || item_type == 1912
}

pub fn is_town_scroll(item_id: i32) -> bool {
    item_id >= 2030000 && item_id < 2030100
}

pub fn is_antibanish_scroll(item_id: i32) -> bool {
    item_id == 2030100
}

pub fn is_clean_slate(scroll_id: i32) -> bool {
    scroll_id > 2048999 && scroll_id < 2049004
}

pub fn is_modifier_scroll(scroll_id: i32) -> bool {
    scroll_id == 2040727 || scroll_id == 2041058
}

pub fn is_flag_modifier(scroll_id: i32, flag: u8) -> bool {
    if scroll_id == 2041058
        && (flag & crate::types::item::flags::COLD as u8) == crate::types::item::flags::COLD as u8
    {
        return true;
    }
    if scroll_id == 2040727
        && (flag & crate::types::item::flags::SPIKES as u8)
            == crate::types::item::flags::SPIKES as u8
    {
        return true;
    }
    false
}

pub fn is_chaos_scroll(scroll_id: i32) -> bool {
    scroll_id >= 2049100 && scroll_id <= 2049103
}

pub fn is_rate_coupon(item_id: i32) -> bool {
    let item_type = item_id / 1000;
    item_type == 5211 || item_type == 5360
}

pub fn is_exp_coupon(coupon_id: i32) -> bool {
    coupon_id / 1000 == 5211
}

pub fn is_party_item(item_id: i32) -> bool {
    item_id >= 2022430 && item_id <= 2022433
}

pub fn is_party_allcure(item_id: i32) -> bool {
    item_id == 2022433
}

pub fn is_hired_merchant(item_id: i32) -> bool {
    item_id / 10000 == 503
}

pub fn is_player_shop(item_id: i32) -> bool {
    item_id / 10000 == 514
}

pub fn is_maker_reagent(item_id: i32) -> bool {
    item_id / 10000 == 425
}

pub fn is_overall(item_id: i32) -> bool {
    item_id / 10000 == 105
}

pub fn is_cash_store(item_id: i32) -> bool {
    let item_type = item_id / 10000;
    item_type == 503 || item_type == 514
}

pub fn is_maple_life(item_id: i32) -> bool {
    let item_type = item_id / 10000;
    item_type == 543 && item_id != 5430000
}

pub fn is_weapon(item_id: i32) -> bool {
    item_id >= 1302000 && item_id < 1493000
}

pub fn is_equipment(item_id: i32) -> bool {
    item_id < 2000000 && item_id != 0
}

pub fn is_fishing_chair(item_id: i32) -> bool {
    item_id == 3011000
}

pub fn is_medal(item_id: i32) -> bool {
    item_id >= 1140000 && item_id < 1143000
}

pub fn is_wedding_ring(item_id: i32) -> bool {
    item_id >= 1112803 && item_id <= 1112809
}

pub fn is_wedding_token(item_id: i32) -> bool {
    item_id >= 4031357 && item_id <= 4031364
}

pub fn is_face(item_id: i32) -> bool {
    item_id >= 20000 && item_id < 22000
}

pub fn is_hair(item_id: i32) -> bool {
    item_id >= 30000 && item_id < 35000
}

pub fn get_inventory_type(item_id: i32) -> InventoryType {
    InventoryType::from_item_id(item_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_classifications() {
        assert!(is_throwing_star(2070000));
        assert!(!is_throwing_star(2000000));
        assert!(is_bullet(2330000));
        assert!(is_potion(2000000));
        assert!(!is_potion(1000000));
        assert!(is_weapon(1302000));
        assert!(!is_weapon(2000000));
        assert!(is_equipment(1000000));
        assert!(!is_equipment(2000000));
        assert!(is_medal(1142000));
        assert!(is_medal(1140000));
        assert!(!is_medal(1143000));
    }

    #[test]
    fn test_is_arrow() {
        assert!(is_arrow_for_bow(2060000));
        assert!(is_arrow_for_crossbow(2061000));
        assert!(is_arrow(2060000));
        assert!(is_arrow(2061000));
        assert!(!is_arrow(1000000));
    }

    #[test]
    fn test_is_pet() {
        assert!(is_pet(5000000));
        assert!(!is_pet(1000000));
    }

    #[test]
    fn test_get_inventory_type() {
        assert_eq!(get_inventory_type(1302000), InventoryType::Equip);
        assert_eq!(get_inventory_type(2000000), InventoryType::Use);
        assert_eq!(get_inventory_type(0), InventoryType::Undefined);
    }
}
