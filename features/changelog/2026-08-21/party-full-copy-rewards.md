Commit: 327d5c87ecb0d83ee8f9fc360d8c3df656d1a4f7

# 组队经验与金币完整复制

## Context

组队玩家希望保留共同作战资格边界，但不再因为队伍人数对怪物经验份额或金币掉落进行二次分摊。

## Change Summary

- 每个符合最低等级和反吸经验等级差限制的同地图成员，获得该队伍本次击杀经验份额的完整副本。
- 移除人数加成、最高等级平方根衰减、最高伤害成员倍率和额外全局组队倍率。
- 符合原有拾取范围的每位成员各自获得完整金币掉落金额。
- 奖励规则提取到无状态 `PartyRewardPolicy`，地图、资格和在线边界沿用现有逻辑。

## Validation

- 全量 Java 7 source/target 编译通过。
- `PartyRewardPolicyVerifier` 覆盖完整经验、完整金币、最低等级和最大等级差边界。

## Notes / Compatibility

- 不新增数据库表、字段或环境变量，不修改数据库数据。
- 仅调整组队奖励数值分配；单人奖励和掉落实体生命周期不变。

## Related Docs

- [client 模块](../../../agents/client/index.md)
- [server 模块](../../../agents/server/index.md)
