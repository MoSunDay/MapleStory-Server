Commit: 2f1cb14f364b511f06e6471d03f99fb98a069500

# 账号会话代次与角色保存 fencing

## Context

MINA 会话关闭先从协调器移除在线客户端，再异步执行 `MapleClient.disconnect`。当同一账号的新登录已加载并修改角色后，旧断线任务仍可能把旧内存角色完整写回数据库，造成角色精确回退到旧会话登录时的快照。

## Change Summary

- 新增 `SessionSaveFence`，以进程内单调 `sessionId` 作为账号保存代次。
- 新会话注册、`onlineClients` 替换和 `accounts.loggedin` 更新持有账号分片 permit；延迟到达的较旧会话不能重新取得所有权。
- `MapleCharacter.saveCharToDB` 捕获保存发起方会话，只有最新代次能取得 permit；permit 覆盖完整数据库事务，而非只保护事务前检查。
- 正常断线后仍保留最后代次，使当前会话可以完成保存；新会话一旦建立，旧断线保存、排队中的旧异步保存都会被拒绝并记录。
- 关闭旧会话时使用 `onlineClients.remove(accountId, client)`，且只有当前代次可以把 `accounts.loggedin` 清零，防止旧连接清理掉新会话映射或在线状态。

## Impact Surface

- 登录、顶号、频道进入和断线保存的账号会话协调。
- 角色属性、背包、技能、任务等整角色持久化事务。

## Validation

- 全量 Java 源码通过 `javac -source 7 -target 7` 编译。
- `SessionSaveFenceVerifier` 覆盖新代次替换、旧代次不可回收、账号隔离，以及新会话等待已开始保存事务完成。

## Notes / Compatibility

- 未新增数据库字段、表或环境变量，未修改或删除数据库数据。
- fencing 作用域为单 JVM；当前部署是单实例服务端。

## Related Docs

- [net 模块](../../../agents/net/index.md)
- [client 模块](../../../agents/client/index.md)
