Commit: (working-tree, pre-commit)

# 登录顶号接线：forceDisconnectAccount 接入 login()，并清理 loggedin 脏旗标

## 背景

- 0b34d5da 新增了 `MapleSessionCoordinator.forceDisconnectAccount(int)`，但**全仓库零调用**（当时 changelog 明确「供后续登录顶替流程接线」）。登录流程碰到账号在线仍走拒绝路径，客户端继续提示「帐号已登录」。
- 拒绝点在 `MapleClient.login()` 两处重载（按 name / 按 email）：`getLoginState() > LOGIN_NOTLOGGEDIN → loginok = 7`，且该校验在**密码校验之前**执行。
- 另一独立病根：服务器重启/shutdown 会为在线账号遗留 `accounts.loggedin=2` 脏旗标（本次部署实测：重启后 admin 账号 loggedin=2 但无任何活跃会话），即使接了 forceDisconnectAccount 也救不了——它只处理 onlineClients 里的**活跃**会话。

## 变更

### MapleClient.login()（两处重载）：先验密码，再谈接管
- 原逻辑先查在线旗标（返回 7）再验密码；改为**先验密码**（错误密码一律返回 4，不再泄露账号在线状态，也杜绝无密码顶号）。
- 密码正确且 `getLoginState() > LOGIN_NOTLOGGEDIN` 时进入顶号路径：
  1. `forceDisconnectAccount(accId)` —— 踢掉活跃在游会话。其 `disconnectInternal` 在调用线程**同步**执行：保存角色 → `closeSession`（同步清 `onlineRemoteHwids`，同机顶号不会被 multiclient 检查挡成 REMOTE_LOGGEDIN/17）→ `updateLoginState(LOGIN_NOTLOGGEDIN)` 重置 DB 旗标。
  2. 重读旗标，若仍 >0（重启遗留脏旗标 / 角色选择期会话等不在 onlineClients 的形态）→ `UPDATE accounts SET loggedin=0` 清零放行。
- 接管后走正常登录流程；进入游戏时 `finishLogin()` 的 loginLock + 状态复检、`updateOnlineSession` 的唯一性语义均保持不变，作为最后防线。

## 测试覆盖

| 功能 | 验证方式 | 结果 |
|------|--------|------|
| 全量编译（757 文件） | `javac --release 11 -cp "cores/*" -d /tmp/jb @/tmp/srcs.txt` | EXIT=0，仅 deprecation/unchecked 提示 |
| 运行时启动 | `systemctl restart` 后 `journalctl _PID=` | Scania × 1 频道正常，8484+7575 监听，0 异常 |
| Rust 回归（本次未触及 Rust 代码） | `cargo test --workspace` | 74 passed / 0 failed |
| e2e 登录顶号 | 无可用客户端测试 harness（mock_login_server 属客户端仓库） | 待用户客户端复测：①脏旗标场景（admin loggedin=2 重启遗留）→ 首次登录应直接放行；②双客户端场景 → 后登录者顶掉先登录者 |

- 行数：MapleClient.java 1956 行为**既有**超限大文件，本次净增约 30 行（两处同构块），不做拆分（超范围）。

## Impact Surface

- 行为变化：密码错误时不再因「账号在线」返回 7，统一返回 4（密码错误）——顺带修复了无密码探测账号在线状态的问题。
- 同机顶号依赖 `closeSession` 同步清理 `onlineRemoteHwids`（forceDisconnect → disconnectInternal → closeSession 在同一线程完成），无竞态窗口。
- 单 JVM 部署假设：清零脏旗标依赖「本 DB 只有这一个服务端进程」；多 JVM 共享 DB 场景需改用带会话探测的清理（当前部署为 1×1，不适用）。
- 不影响：角色数据结构、频道/world 拓扑、协议格式。

## 已知缺口

- 服务器 shutdown 路径本身仍不重置在线玩家 `accounts.loggedin`（顶号路径可自愈其后果，未修根因，超本次范围）。
- 角色选择期（loggedin=1）的会话不在 onlineClients 中、无法被踢，只能靠清零旗标放行；该残留客户端此后进入游戏会在 finishLogin/updateOnlineSession 被唯一性机制处理。

## Related Docs

- [features/index.md](../index.md)
- [2026-08-11 forceDisconnectAccount API](../2026-08-11/session-takeover-api-skillbook-fix-jre11-launcher.md)
- [single-world-single-channel](./single-world-single-channel.md)
