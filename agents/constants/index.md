Commit: ea0bee5e598775e27e5f0e2bd842a4cbc0ce7264

# constants 模块 — 静态常量

## 职责

全局静态常量定义：服务器配置、游戏规则、经验表、物品类型枚举、54 个职业的技能 ID 常量。

## 边界

- 纯常量，无状态，无逻辑
- 被所有模块依赖，不依赖任何模块

## 关键文件

| 文件 | 职责 |
|---|---|
| `ServerConstants` | 服务器配置（倍率/标志/功能开关/数据库配置） |
| `GameConstants` | 游戏规则常量 |
| `ItemConstants` | 物品分类规则 |
| `ExpTable` | 各等级经验表 |
| `EquipSlot` / `EquipType` | 装备槽位/类型枚举 |
| `skills/*.java` | 54 个文件，每个职业一个技能 ID 常量类 |

## 依赖

- 无出向依赖（叶节点模块）
- ← 被所有其他模块依赖
