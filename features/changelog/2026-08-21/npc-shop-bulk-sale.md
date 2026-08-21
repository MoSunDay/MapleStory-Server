Commit: 327d5c87ecb0d83ee8f9fc360d8c3df656d1a4f7

# NPC 商店现金安全批量售卖

## Context

客户端需要一次出售当前背包分类的所有普通物品，同时现金物品必须在显示层之外继续由服务端强制拒绝，避免伪造请求造成不可逆损失。

## Change Summary

- `NPC_SHOP_ACTION` mode 4 接收背包分类，并在对应背包锁内基于实时快照执行一次批量结算。
- 单件和批量路径共用数量、分类、可售价格和现金身份策略。
- 现金身份同时检查持久化现金标志与 WZ 属性；商城取得物品在写入背包前补记来源标志。
- 批量操作只发送一次最终交易结果，失败状态区分无可售物品与现金物品拒绝。

## Validation

- 全量 Java 7 source/target 编译通过。
- `NpcShopSalePolicyVerifier` 覆盖分类、数量边界、可充值物完整堆叠和现金物品拒绝。

## Notes / Compatibility

- 不新增数据库表、字段或环境变量，不修改数据库数据。
- 原购买、单件售卖、充值和退出模式保持兼容。

## Related Docs

- [server 模块](../../../agents/server/index.md)
