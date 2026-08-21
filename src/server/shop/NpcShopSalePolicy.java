package server.shop;

import client.inventory.MapleInventoryType;

public final class NpcShopSalePolicy {
    private NpcShopSalePolicy() {
    }

    public static boolean supportsBulkSale(MapleInventoryType type) {
        return type == MapleInventoryType.EQUIP || type == MapleInventoryType.USE
                || type == MapleInventoryType.SETUP || type == MapleInventoryType.ETC;
    }

    public static short normalizeRequestedQuantity(short quantity) {
        return quantity == 0 || quantity == (short) 0xFFFF ? 1 : quantity;
    }

    public static boolean canSell(boolean cash, boolean rechargeable,
                                  short inventoryQuantity, short requestedQuantity) {
        if (cash) {
            return false;
        }

        short available = inventoryQuantity == (short) 0xFFFF ? 1 : inventoryQuantity;
        short requested = normalizeRequestedQuantity(requestedQuantity);
        if (available <= 0 || requested <= 0) {
            return false;
        }

        return rechargeable || requested <= available;
    }

    public static short sellingQuantity(boolean rechargeable,
                                        short inventoryQuantity, short requestedQuantity) {
        short available = inventoryQuantity == (short) 0xFFFF ? 1 : inventoryQuantity;
        return rechargeable ? available : normalizeRequestedQuantity(requestedQuantity);
    }
}
