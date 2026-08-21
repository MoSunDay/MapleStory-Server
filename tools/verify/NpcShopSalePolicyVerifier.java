import client.inventory.MapleInventoryType;
import client.inventory.Item;
import constants.ItemConstants;
import server.shop.CashItemIdentity;
import server.shop.NpcShopSalePolicy;

public final class NpcShopSalePolicyVerifier {
    private NpcShopSalePolicyVerifier() {
    }

    private static void require(boolean condition, String message) {
        if (!condition) {
            throw new AssertionError(message);
        }
    }

    public static void main(String[] args) {
        require(NpcShopSalePolicy.supportsBulkSale(MapleInventoryType.EQUIP),
                "equip tab supports bulk sale");
        require(NpcShopSalePolicy.supportsBulkSale(MapleInventoryType.USE),
                "use tab supports bulk sale");
        require(NpcShopSalePolicy.supportsBulkSale(MapleInventoryType.SETUP),
                "setup tab supports bulk sale");
        require(NpcShopSalePolicy.supportsBulkSale(MapleInventoryType.ETC),
                "etc tab supports bulk sale");
        require(!NpcShopSalePolicy.supportsBulkSale(MapleInventoryType.CASH),
                "cash tab rejects bulk sale");
        require(!NpcShopSalePolicy.supportsBulkSale(null),
                "invalid tab rejects bulk sale");

        require(NpcShopSalePolicy.normalizeRequestedQuantity((short) 0) == 1,
                "legacy zero quantity normalizes to one");
        require(NpcShopSalePolicy.normalizeRequestedQuantity((short) 0xFFFF) == 1,
                "legacy unsigned sentinel normalizes to one");
        require(NpcShopSalePolicy.canSell(false, false, (short) 10, (short) 10),
                "entire normal stack is sellable");
        require(!NpcShopSalePolicy.canSell(false, false, (short) 10, (short) 11),
                "normal stack cannot be oversold");
        require(!NpcShopSalePolicy.canSell(true, false, (short) 1, (short) 1),
                "cash item is never sellable");

        Item cashOrigin = new Item(4031192, (short) 1, (short) 1);
        CashItemIdentity.markCashOrigin(cashOrigin);
        require(CashItemIdentity.hasCashOrigin(cashOrigin),
                "cash-shop origin survives through the persisted item flag");
        require((cashOrigin.getFlag() & ItemConstants.CASH_ORIGIN) != 0,
                "cash origin uses the dedicated flag bit");
        require(!NpcShopSalePolicy.canSell(false, true, (short) 0, (short) 1),
                "empty rechargeable stack is invalid");
        require(NpcShopSalePolicy.sellingQuantity(true, (short) 800, (short) 1) == 800,
                "rechargeable sale consumes its full stack");
        require(NpcShopSalePolicy.sellingQuantity(false, (short) 800, (short) 5) == 5,
                "normal sale preserves requested quantity");

        System.out.println("NPC shop sale policy verification passed");
    }
}
