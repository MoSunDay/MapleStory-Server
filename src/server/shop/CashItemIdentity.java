package server.shop;

import client.inventory.Item;
import constants.ItemConstants;
import server.MapleItemInformationProvider;

public final class CashItemIdentity {
    private CashItemIdentity() {
    }

    public static boolean hasCashOrigin(Item item) {
        return (item.getFlag() & ItemConstants.CASH_ORIGIN) != 0;
    }

    public static boolean isCashItem(Item item, MapleItemInformationProvider itemInfo) {
        return hasCashOrigin(item) || itemInfo.isCash(item.getItemId());
    }

    public static void markCashOrigin(Item item) {
        item.setFlag((byte) (item.getFlag() | ItemConstants.CASH_ORIGIN));
    }
}
