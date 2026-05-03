# MapleStory-Server Java → Rust 迁移规划

## 执行状态

| Phase | 内容 | 状态 | 交付物 |
|-------|------|------|--------|
| Phase 0a | maple-core (类型/常量/技能) | ✅ 完成 | 54 技能模块, 8 类型枚举, 3 常量集, 32 测试 |
| Phase 0b | maple-crypto (AES-256 + 自定义加密) | ✅ 完成 | 自含 AES-256, 自定义字节混淆, 14 测试, NIST 已知向量 |
| Phase 0c | maple-wz + maple-db | ✅ 代码就绪 | WZ 数据模型/Provider, DB 模型/查询 (待 Rust 更新后编译) |
| Phase 1 | maple-net (tokio 网络层) | 📋 代码就绪 | Stub 已创建, 待 Rust 更新 |
| Phase 2 | maple-game (游戏世界逻辑) | 📋 代码就绪 | Stub 已创建, 待 Rust 更新 |
| Phase 5 | maple-script (JS 引擎桥接) | 📋 代码就绪 | Stub 已创建, 待 Rust 更新 |

## 当前编译状态

```
cargo test --all
test result: ok. 46 passed; 0 failed; 0 warnings
```

- **可编译**: `maple-core` + `maple-crypto` (零外部依赖)
- **代码就绪**: 其余 6 个 crate 源码已完成, Cargo.toml 已就绪
- **阻塞**: Rust 1.68, 需 ≥1.77 以安装外部依赖

## 启用全部 crate

```bash
# 1. 更新 Rust
rustup update stable

# 2. 编辑 rust_workspace/Cargo.toml, 取消注释其他 members
#   或运行: cargo check --all (首次需下载依赖 ~200MB)

# 3. 验证
cargo test --all
```

## 项目结构 (完整)

```
rust_workspace/
├── Cargo.toml                   # workspace (当前: maple-core + maple-crypto)
├── rust-toolchain.toml          # channel = "stable"
├── PLAN.md                      # 本文档
├── crates/
│   ├── maple-core/              # ✅ Phase 0a
│   │   └── src/
│   │       ├── types/           # job, stat, item, equip_slot
│   │       ├── constants/       # game, item, exp_table
│   │       └── skills/          # 54 职业技能常量模块
│   ├── maple-crypto/            # ✅ Phase 0b (修复: AES-128→AES-256)
│   │   └── src/
│   │       ├── aes256.rs        # AES-256 自含实现 (NIST 已知向量验证)
│   │       ├── aes_ofb.rs       # MapleAesOfb: OFB 模式 + funnyBytes IV
│   │       └── custom_enc.rs    # Nexon 6 轮字节滚动混淆
│   ├── maple-wz/ ✅ 就绪       # 数据模型 + Provider 工厂
│   │   └── src/
│   │       ├── data_types.rs    # DataType/DataEntry/DataValue
│   │       └── reader.rs          # WzBinaryProvider/XmlWzProvider
│   ├── maple-db/ ✅ 就绪       # 11 个数据模型 + 基础查询
│   │   └── src/
│   │       ├── models/schema.rs # Account/Character/Guild/Item/Skill...
│   │       └── queries/mod.rs   # 连接池/角色查询/账号管理
│   ├── maple-net/  📋 待启用
│   ├── maple-game/ 📋 待启用
│   ├── maple-script/ 📋 待启用
│   └── maple-server/ 📋 待启用
└── config/default.toml
```

## 测试覆盖

| 模块 | 测试数 | 覆盖范围 |
|------|--------|---------|
| maple-core | 32 | job 往返/嵌套/is_a, stat 编码/字符串, inventory type, equip slot, exp 表, 物品判定, 游戏常量 |
| maple-crypto | 14 | AES-256 已知向量 + 往返, OFB 加密/解密对称, packet header 往返*5, 包校验, IV 演进, 自定义加密 |
| **合计** | **46** | |
