# rust-migration-init → rust-migration-phase-0-1

## 上下文

MapleStory-Server 启动 Java → Rust 增量旁路迁移。在保持 Java 代码零变化前提下，用 Rust 构建可选后端，共享 MySQL + WZ + JS 脚本。

## 变更摘要 (最终状态: commit bd10e6ef)

- 新建 `rust_workspace/` Cargo workspace，含 8 个 crate
- **Phase 0a** (`maple-core`): 类型枚举 (MapleJob/MapleStat/InventoryType/EquipSlot)、常量 (经验表/游戏常量/物品判定)、54 职业技能 ID 模块，**32 测试**
- **Phase 0b** (`maple-crypto`): AES-256 OFB + 自定义字节滚动加密 (自含 AES-256 实现，NIST 已知向量验证)，**14 测试**
- **Phase 0c** (`maple-wz` + `maple-db`): WZ 数据模型/Provider 工厂，11 表 sqlx 模型 + 基础查询，**6 测试**
- **Phase 1** (`maple-net`): 400+ opcode 枚举、LE writer/reader、包构建器 (Hello/Auth/ServerList/Charlist/ServerIP)、tokio codec、handler registry、登录分发，**22 测试**
- **Phase 1** (`maple-server`): TCP 登录服务器监听 0.0.0.0:8484，完整闭环 (握手→解码→路由→处理→编码→响应)
- 新增 `agents/rust-workspace/index.md` 模块文档，更新 `AGENTS.md` 模块索引

## Review 修复

| 严重度 | 问题 | 修复 |
|--------|------|------|
| CRITICAL | `write_char_entry` 缺失 GM 判定 | 添加 `is_gm` 字段，GM 角色跳过排名数据 |
| MEDIUM | 解码路径缺少 `check_packet_header` | codec.rs decode + login.rs recv_packet |
| LOW | IV 随机字节从同一时间戳派生 | 使用 LCG 独立派生两个 IV 字节 |
| - | AES-128 误用 (Java 为 AES-256) | 重写为完整 AES-256 实现 |

## 影响面

- **新增**: `rust_workspace/` 独立目录，不影响现有 Java/JS/SQL 资源
- **共享**: `scripts/`、`sql/`、`wz/` 为 Java 和 Rust 共同使用 (不修改)
- **测试**: **74 个 Rust 单元测试**，`cargo test --all` 全通过
- **运行**: `cargo run -p maple-server` 启动 v83 登录服务器

## 相关文档

- [agents/rust-workspace/index.md](../../agents/rust-workspace/index.md)
- [rust_workspace/PLAN.md](../../../rust_workspace/PLAN.md)
