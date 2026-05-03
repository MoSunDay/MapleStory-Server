Commit: bd10e6ef

# rust-workspace — Rust 旁路迁移

## 职责

Java 服务端的 Rust 旁路实现，作为可选后端并行运行，共享 MySQL 数据库和 WZ 游戏资源。在保持 Java 代码零变化的前提下，用 Rust 重写核心游戏服务器逻辑。

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
    ├── maple-core/         # ✅ 类型、常量、54 职业技能模块
    ├── maple-crypto/       # ✅ AES-256 OFB + 自定义加密
    ├── maple-wz/           # ✅ WZ 数据模型 + Provider 工厂
    ├── maple-db/           # ✅ 11 表 sqlx 模型 + 基础查询
    ├── maple-net/          # ✅ 网络层 (400+ opcodes, writer/reader, builder, codec, handler)
    ├── maple-game/         # 📋 游戏世界逻辑
    ├── maple-script/       # 📋 JS 脚本桥接
    └── maple-server/       # ✅ TCP 登录服务器 (8484)
```

## 已完成模块

### maple-core (32 tests)

**54 个职业技能常量模块**，与 Java `constants/skills/` 逐文件对应。

| 导出 | 说明 |
|------|------|
| `types::MapleJob` | 92 个变体，含 `id()`/`from_id()`/`is_a()`/`job_niche()` |
| `types::MapleStat` | 19 个属性枚举，含 `value()`/`from_str()`/`from_5_byte_encoding()` |
| `types::InventoryType` | 8 种背包类型（Equip/Use/Setup/Etc/Cash/CanHold/Equipped） |
| `types::EquipSlot` | 23 种装备槽位 |
| `constants::exp_table` | 经验/宠物亲密度/坐骑/装备经验表 |
| `constants::game` | 城镇坐标、倍率增益、键位预设、怪物 HP 表、职业判定函数 |
| `constants::item` | 物品类型判定（40+ 个 `is_*` 函数） |

### maple-crypto (14 tests)

与 Java `MapleAESOFB` + `MapleCustomEncryption` 逐行对应。

| 组件 | 功能 |
|------|------|
| `aes256::Aes256` | 自含 AES-256 软件实现，NIST 已知向量验证 |
| `aes_ofb::MapleAesOfb` | OFB 模式 + funnyBytes 256 字节 IV 演进，包头部生成/校验 |
| `custom_enc` | 6 轮交替正向/逆向字节滚动混淆（encryptData/decryptData） |

### maple-wz (4 tests)

WZ 数据模型：`DataType`(16 种)、`DataValue`、`DataEntry`、`WzBinaryProvider`/`XmlWzProvider` 工厂。

### maple-db (2 tests)

11 张核心表 sqlx 模型：Account、Character、Guild、Alliance、InventoryItem、InventoryEquipment、Skill、QuestStatus、KeyMap、Buddy、ShopItem。基础查询：连接池、角色/账号加载、登录状态重置。

### maple-net (22 tests)

| 模块 | 功能 |
|------|------|
| `packet::opcodes` | 400+ SendOpcode/RecvOpcode 枚举，含 `from_value()` 查找 |
| `packet::writer` | LE 字节写出器：writeByte/Short/Int/Long/Bool/String/Pos |
| `packet::reader` | LE 字节读取器：readByte/Short/Int/Long/MapleAsciiString/skip |
| `packet::builder` | 包构建器：build_hello、build_auth_success、build_login_failed、build_server_list、build_char_list、build_server_ip 等 |
| `codec::MapleCodec` | tokio Encoder/Decoder：custom_enc → AES → header 包帧化 |
| `handler` | HandlerRegistry + login dispatch（PONG/LoginPassword/Serverlist/Charlist） |

### maple-server (启动入口)

`cargo run -p maple-server` → TCP 监听 `0.0.0.0:8484`，完整闭环：
握手(Hello) → 解码(AES-decrypt→custom-decrypt) → 路由(opcode lookup) → 处理(handler) → 编码(custom-encrypt→AES-encrypt→prepend header) → 响应

## 技术选型

| 维度 | 选择 | 状态 |
|------|------|------|
| 异步运行时 | tokio | ✅ 已集成 |
| 加密 | 自含 AES-256 (无外部依赖) | ✅ NIST 验证 |
| 数据库 | sqlx 0.8 | ✅ 模型就绪 |
| 环境 | Rust 1.95 | ✅ 全部 crate 可编译 |

## 关键设计约束

1. **纯函数式**: 无 `class`，全部使用 enum + const + 纯函数
2. **文件行数**: 新增 ≤400 行，迭代 ≤800 行
3. **逻辑不变**: 与 Java 源码逐行对应，不优化、不重构
4. **JS 脚本共享**: `scripts/` 目录为 Java 和 Rust 共同使用

## 迁移规划

详见 [rust_workspace/PLAN.md](../../rust_workspace/PLAN.md)

## 依赖关系

```
maple-core        ← (所有 crate 依赖)
maple-crypto      ← (无外部依赖)
maple-wz          ← maple-core, nom
maple-db          ← maple-core, sqlx, chrono
maple-net         ← maple-core, maple-crypto, tokio, tokio-util
maple-game        ← maple-core, maple-db, maple-net, maple-script
maple-script      ← maple-core
maple-server      ← 所有 crate, tracing, toml
```
