Commit: (working-tree, pre-commit)

# 账号顶号 API、技能书 imgdir 解析修复与 JRE 11 启动脚本

## 背景

- 账号已在游戏中时无法顶替登录：需要一个公开 API 供新登录强制踢掉旧会话。
- `MapleSkillbookInformationProvider` 逐行解析 WZ XML 时，自闭合 `<imgdir ... />` 标签被误判为开标签，导致嵌套深度计数错误、技能书数据解析错位。
- 服务端需要在 JRE 11（保留 Nashorn JS 引擎）下启动：缺少启动脚本，且 `JAVA8` 开关与运行时不匹配。

## 变更

### 会话协调器：账号顶号 API
- **`src/net/server/coordinator/MapleSessionCoordinator.java`**：新增 `forceDisconnectAccount(int)` (src/net/server/coordinator/MapleSessionCoordinator.java:250)。从 `onlineClients` 移除并 `forceDisconnect()` 该账号的在线客户端，使新登录可接管账号；返回是否踢掉了旧会话。

### 技能书 XML 解析修复
- **`src/server/MapleSkillbookInformationProvider.java`**：三处 `imgdir` 深度计数 (src/server/MapleSkillbookInformationProvider.java:144,185,206) 增加 `!token.trim().endsWith("/>")` 条件，自闭合 `<imgdir ... />` 不再使嵌套深度 +1。

### JRE 11 启动与运行配置
- **`run.sh`**（新增，10 行）：用 `java-11-openjdk-amd64` 启动 `net.server.Server`（`-Xmx2048m -Dwzpath=wz -cp ".:dist:cores/*"`），并过滤 Nashorn 弃用提示。
- **`configuration.ini`**：`JAVA8=FALSE → TRUE`，与 Java 8+ 脚本调用路径（`ServerConstants.JAVA_8` 控制的 scripting 分支）匹配 JRE 11 运行时。
- **`.gitignore`**：新增 `logging.properties`；修复 acbfe377 引入的断行错误——`scripts*/rust_workspace/target/` 还原为 `scripts*/` 与 `rust_workspace/target/` 两条独立规则（原断行使 `scripts*/` 规则失效）。

## 测试覆盖

| 功能 | 验证方式 | 结果 |
|------|--------|------|
| 全量编译（757 文件） | `javac -d /tmp/jb @srcs.txt`（JDK 17，产物运行于 JRE 11） | 通过，仅 deprecation/unchecked 提示 |
| Rust 回归（本次未触及 Rust 代码） | `cargo test --workspace` | 74 passed / 0 failed |

- 行数：MapleSessionCoordinator.java 646 ≤ 800；MapleSkillbookInformationProvider.java 358 ≤ 400；run.sh 10 ≤ 400

## Impact Surface

- 新增公开 API `forceDisconnectAccount`，当前无调用方，供后续登录顶替流程接线；不改变现有登录/会话行为。
- 技能书数据解析行为修正：自闭合 imgdir 不再破坏解析深度（只读数据提供器，不影响写库）。
- `run.sh` 为可选启动入口；`JAVA8=TRUE` 切换 scripting 走 Java 8+ 分支，与 JRE 11 运行时配套。
- 不影响：rust_workspace 源码、数据库 schema、网络协议。

## 已知缺口（不属本次变更面）

- `cargo clippy --workspace --all-targets -- -D warnings` 在 `maple-crypto` 有 10 个**既有** lint 错误（identity_op / needless_range_loop / manual_rotate 等），为 bd10e6ef 已提交代码在新版 clippy (1.97.0) 下暴露，与本次 Java 变更无关，留待 Rust 侧单独清理。

## Related Docs

- [features/index.md](../index.md)
- [rust-migration-init](../2026-05-03/rust-migration-init.md)
