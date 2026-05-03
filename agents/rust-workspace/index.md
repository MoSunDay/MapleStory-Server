Commit: ee5cc0ff39cc14da1ec1a7593c7501a18f560089

# rust-workspace — Rust 旁路迁移

## 职责

Java 服务端的 Rust 旁路实现，作为可选后端并行运行，共享 MySQL 数据库和 WZ 游戏资源。目标：在保持 Java 代码零变化的前提下，用 Rust 重写核心游戏服务器逻辑。

## 边界与非目标

- **在范围内**: 类型/常量定义、网络 I/O、加密、游戏世界逻辑、JS 脚本桥接
- **不在范围内**: 替换 Java 现有功能、修改 Java 源码、WZ 编辑器工具

## 目录结构

```
rust_workspace/
├── Cargo.toml              # Cargo workspace 根
├── PLAN.md                 # 迁移规划文档
├── config/default.toml     # 服务端配置 (TOML)
├── migrations/             # sqlx 数据库迁移
├── scripts/                # 共享 JS 脚本 (软链接)
└── crates/
    ├── maple-core/         # ✅ Phase 0a: 类型、常量、54 职业技能模块
    ├── maple-crypto/       # ✅ Phase 0b: AES-OFB + 自定义加密
    ├── maple-wz/           # 📋 Phase 0c: WZ 数据解析
    ├── maple-db/           # 📋 Phase 0c: 数据库模型 (sqlx)
    ├── maple-net/          # 📋 Phase 1: 网络层 (tokio)
    ├── maple-game/         # 📋 Phase 2: 游戏世界逻辑
    ├── maple-script/       # 📋 Phase 5: JS 脚本桥接 (boa_engine)
    └── maple-server/       # 📋 Phase 7: 入口 + 配置
```

## 已完成模块

### maple-core

**54 个职业技能常量模块**，与 Java `constants/skills/` 逐文件对应。

| 导出 | 说明 |
|------|------|
| `types::MapleJob` | 92 个变体，含 `id()`/`from_id()`/`is_a()`/`job_niche()` |
| `types::MapleStat` | 19 个属性枚举，含 `value()`/`from_str()`/`from_5_byte_encoding()` |
| `types::InventoryType` | 8 种背包类型（Equip/Use/Setup/Etc/Cash/CanHold/Equipped） |
| `types::EquipSlot` | 23 种装备槽位 |
| `types::item::flags` | 10 种物品 Flags（LOCK/SPIKES/KARMA/COLD/UNTRADEABLE…） |
| `constants::exp_table` | 经验/宠物亲密度/坐骑/装备经验表 |
| `constants::game` | 城镇坐标、倍率增益、键位预设、怪物 HP 表、职业判定函数 |
| `constants::item` | 物品类型判定（40+ 个 `is_*` 函数） |

**测试**: 32 个单元测试覆盖所有枚举往返、边界值和关键判定逻辑。

### maple-crypto

与 Java `MapleAESOFB` + `MapleCustomEncryption` 逐行对应。

| 组件 | 功能 |
|------|------|
| `aes_ofb::MapleAesOfb` | 自含 AES-128 软件实现，funnyBytes 256 字节 IV 演进表，包头部生成/校验 |
| `custom_enc` | 6 轮交替正向/逆向字节滚动混淆（encryptData/decryptData） |

**测试**: 9 个测试覆盖加密/解密对称性、包头部往返、IV 演进、字节滚动操作。

### 技术选型

| 维度 | 选择 | 理由 |
|------|------|------|
| 异步运行时 | tokio (未集成) | 生态最成熟 |
| JS 引擎 | boa_engine (未集成) | 纯 Rust，无 FFI |
| 数据库 | sqlx (未集成) | 编译期 SQL 检查 |
| 加密 | 自含 (无外部依赖) | 忠实移植 v83 协议 |

## 关键设计约束

1. **纯函数式**: 无 `class`，全部使用 enum + const + 纯函数
2. **文件行数**: 的新增 ≤400 行，迭代 ≤800 行
3. **逻辑不变**: 与 Java 源码逐行对应，不优化、不重构
4. **JS 脚本共享**: `scripts/` 目录为 Java 和 Rust 共同使用

## 迁移规划

详见 [rust_workspace/PLAN.md](../../rust_workspace/PLAN.md)

## 依赖关系

```
maple-core        ← (所有 crate 依赖，无外部依赖)
maple-crypto      ← (无外部依赖，无 maple-core 依赖)
maple-wz (待定)   ← maple-core
maple-db (待定)   ← maple-core, sqlx
maple-net (待定)  ← maple-core, maple-crypto, tokio
maple-game (待定) ← maple-core, maple-db, maple-net, maple-script
maple-script (待定) ← maple-core, boa_engine
maple-server (待定) ← 所有 crate, tracing
```
