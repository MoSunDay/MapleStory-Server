# rust-migration-init

## 上下文

MapleStory-Server 启动 Java → Rust 增量旁路迁移。目标：在保持 Java 代码零变化前提下，用 Rust 构建可选后端，共享 MySQL + WZ + JS 脚本。

## 变更摘要

- 新建 `rust_workspace/` Cargo workspace，含 8 个 crate
- **Phase 0a** (`maple-core`): 移植类型枚举 (MapleJob/MapleStat/InventoryType/EquipSlot)、常量 (经验表/游戏常量/物品判定)、54 职业技能 ID 模块，**32 个单元测试**
- **Phase 0b** (`maple-crypto`): 移植 AES-OFB + 自定义字节滚动加密 (自含 AES-128 软件实现)，**9 个单元测试**
- 其他 6 个 crate 为 stub，待后续 Phase 实现
- 新增 `agents/rust-workspace/index.md` 模块文档
- 更新 `AGENTS.md` 模块索引

## 影响面

- **新增**: `rust_workspace/` 独立目录，不影响现有 Java/JS/SQL 资源
- **共享**: `scripts/`、`sql/`、`wz/` 为 Java 和 Rust 共同使用 (不修改)
- **测试**: 41 个 Rust 单元测试，`cargo test` 全通过

## 备注

- 当前 Rust 1.68 版本限制：仅 `maple-core` + `maple-crypto` 可编译 (零外部依赖)
- 后续 crate 需要 Rust ≥1.85 以支持 transitively dependent on edition 2024

## 相关文档

- [agents/rust-workspace/index.md](../../agents/rust-workspace/index.md)
- [rust_workspace/PLAN.md](../../../rust_workspace/PLAN.md)
