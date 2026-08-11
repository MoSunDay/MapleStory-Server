Commit: (working-tree, pre-commit)

# 单世界（Scania）× 单频道部署

## 背景

- 服务器默认发行 5 世界 × 2 频道，本部署只需 1 世界 1 频道；多余世界/频道空耗内存并占用 7576-79xx 端口。
- 世界/频道数由仓库根 `world.ini` 驱动；启动循环 `Server.init()` (src/net/server/Server.java:1061) 按 `worlds` 逐个 `initWorld()`，失败即 `System.exit(0)`。
- 硬阻塞：`ServerConstants.CHANNEL_MIN_SIZE = 2` 是每世界频道下限，`channels0=1` 会命中 Server.java:638 检查直接退出，故常量必须同步改为 1。
- 数据面安全：DB 中 4 个角色全在 world=0（2 个账号），世界 1-4 无孤儿角色。

## 变更

### world.ini：单世界单频道
- `worlds=5 → 1`、`channels0=2 → 1`、头注释 `#Number of channels: 2-30 → 1-30`。
- world 1-4 配置块**保留不删**（加载器按 worldCount 截断），小 diff 便于回滚。

### ServerConstants.java：放宽频道下限
- `CHANNEL_MIN_SIZE = 2 → 1` (src/constants/ServerConstants.java:38)。另一处引用为运行时 addChannel (src/net/server/Server.java:552)，MIN=1 语义仍自洽。

### Server.java：报错文案同步
- "Number of channels must be at least two" → "at least one" (src/net/server/Server.java:639)。

### dist/ 全量重建
- `javac --release 11 -cp "cores/*"` 全树重编 757 源文件 → 1023 classes，替换 dist/。`CHANNEL_MIN_SIZE` 是编译期常量、会内联进引用类（Server.class 等），只编改动文件不够，必须整树重编（已用 javap 验证 Server.class 内联值与文案为新值）。

## 测试覆盖

| 功能 | 验证方式 | 结果 |
|------|--------|------|
| 全量编译（757 文件） | `javac --release 11 -cp "cores/*" -d /tmp/jb @/tmp/srcs.txt` | EXIT=0，仅 deprecation/unchecked 提示 |
| 字节码常量/文案 | `javap -constants` / `javap -c` | CHANNEL_MIN_SIZE=1、"at least one" 已内联进 Server.class |
| 运行时启动 | `systemctl restart` 后 `journalctl _PID=` | 仅 `Loading Scania (0)` → `Channel 1: Listening on port 7575` → `Finished loading` → `Listening on port 8484` → `MapleStory is now online`，0 异常 |
| 端口面 | `ss -tlnp` | 仅 8484 + 7575，同一 java PID；7576/76xx/77xx/78xx/79xx 全部消失 |
| Rust 回归（本次未触及 Rust 代码） | `cargo test --workspace` | 74 passed / 0 failed |

- 行数：ServerConstants.java 345 ≤ 800；world.ini 40 ≤ 400；Server.java 1927 行为**既有**超限大文件，本次仅改 1 行文案，不做拆分（超范围）。

## Impact Surface

- 客户端世界/频道列表由服务端包动态生成，1 世界 × 1 频道为合法形态，无兼容问题。
- DB `characters.world` 历史值无需清理（现存角色均为 0）。
- 运行时 addChannel 下限同步放宽为 1。

## 回滚

- 恢复 `world.ini` 的 `worlds=5` / `channelsN=2` 并重启服务即可（`CHANNEL_MIN_SIZE=1` 不阻碍 2 频道，无需重编代码）。

## 已知缺口（运维注记）

- `systemctl restart` 若紧跟旧进程退出（旧服若有活跃连接且由服务端先关闭，端口会残留 ~60s TIME_WAIT），MINA acceptor bind 会报 "Address already in use"，进程照常打印 online 但实际未监听。本次部署第一次重启即命中，待 TIME_WAIT 过期后二次重启恢复；后续重启如遇此象，等 ~1 分钟再重启。

## Related Docs

- [features/index.md](../index.md)
- [2026-08-11 session-takeover-api-skillbook-fix-jre11-launcher](../2026-08-11/session-takeover-api-skillbook-fix-jre11-launcher.md)
