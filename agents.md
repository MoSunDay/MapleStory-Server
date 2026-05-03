Commit: ea0bee5e598775e27e5f0e2bd842a4cbc0ce7264

# MapleStory-Server 逻辑地图

## 仓库概述

Fork 自 HeavenMS 的 MapleStory（冒险岛）v83 私服服务端，基于 Java 7 + Apache MINA NIO + MySQL 构建的单体架构游戏服务器。入口类为 `net.server.Server`（单例），通过 Ant 构建为 `dist/MapleStory.jar`，JVM 参数 `-Xmx2048m -Dwzpath=wz/`。

**规模**: 757 Java 文件，约 115,000 行代码；1,862 JS 脚本文件，约 101,000 行脚本。

## 技术栈

| 层 | 技术 |
|---|---|
| 网络 I/O | Apache MINA 2.0.19 (NIO) |
| 数据库 | MySQL + HikariCP 连接池 |
| 脚本引擎 | javax.script (Nashorn/Rhino) |
| 构建 | Apache Ant (NetBeans 项目) |
| 加密 | AES-OFB + 自定义字节滚动密码 |
| 日志 | SLF4J 1.7.21 |

## 模块索引

| 模块 | 源码路径 | 职责 | 文档 |
|---|---|---|---|
| **net** | `src/net/` | 网络层：MINA codec、包分发、登录/频道处理器、世界/频道生命周期 | [agents/net/index.md](agents/net/index.md) |
| **client** | `src/client/` | 玩家域模型：角色、背包、技能、任务、GM 命令、自动封禁 | [agents/client/index.md](agents/client/index.md) |
| **server** | `src/server/` | 游戏世界逻辑：地图、怪物、NPC、反应堆、任务、商店、交易、事件 | [agents/server/index.md](agents/server/index.md) |
| **scripting** | `src/scripting/` | JS 脚本引擎桥接：NPC/事件/任务/传送门/反应堆/道具/地图脚本 | [agents/scripting/index.md](agents/scripting/index.md) |
| **provider** | `src/provider/` | WZ 数据提供层：读取 .wz 二进制或 XML 提取的游戏资源 | [agents/provider/index.md](agents/provider/index.md) |
| **constants** | `src/constants/` | 静态常量：经验表、物品类型、54 个职业技能 ID 枚举 | [agents/constants/index.md](agents/constants/index.md) |
| **tools** | `src/tools/` | 基础设施：包构建器、加密、数据库连接、日志、字节流 I/O | [agents/tools/index.md](agents/tools/index.md) |

## 运行时结构

```
Server (单例)
 ├── LoginServer         端口 8484，处理认证/角色选择
 ├── World[0..4]         Scania/Bera/Aurora/Elysium/Reboot
 │    ├── Channel[0..1]  各自独立 MINA acceptor，独立端口
 │    │   ├── MapleMapFactory → MapleMap[] (懒加载)
 │    │   ├── EventScriptManager
 │    │   └── HiredMerchant[]
 │    ├── Party[] / Messenger[] / Family[]
 │    └── 定时器：宠物饱食、坐骑疲劳、自动保存…
 ├── Guild[] / Alliance[] (全局)
 ├── PlayerBuffStorage (跨频道 buff 持久化)
 └── 全局定时器：疾病、排名、优惠券、锁释放…
```

## 依赖方向

```
constants ← (所有模块依赖)
tools     ← (所有模块依赖)
provider  ← server, client
server    ← client, net, scripting
client    ← net, scripting
scripting ← client, server, net.server
net       ← client, server, tools (最外层，接线层)
```

## 非 Java 资源

| 路径 | 用途 |
|---|---|
| `scripts/` | 1,862 个 JS 脚本（NPC 691 / 事件 86 / 任务 257 / 传送门 450 / 反应堆 287 / 地图 88） |
| `sql/` | 数据库 schema (`db_database.sql`) 和掉落数据 (`db_drops.sql`) |
| `wz/` (运行时) | WZ 游戏资源文件（地图/物品/怪物/技能数据） |
| `cores/` | 编译期依赖 JAR（MINA/HikariCP/MySQL/SLF4J/JavaTuples） |
| `tools/` | 30 个独立 Java 数据抓取/生成工具 |
| `handbook/` | 游戏数据参考文本（NPC/怪物/物品/地图 ID 表） |

## 业务能力索引

→ [features/index.md](features/index.md)
