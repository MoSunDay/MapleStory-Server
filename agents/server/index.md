Commit: ea0bee5e598775e27e5f0e2bd842a4cbc0ce7264

# server 模块 — 游戏世界逻辑

## 职责

游戏世界数据结构与逻辑：地图、怪物、NPC、反应堆、物品、商店、任务、交易、事件、扭蛋、远征队、组队任务、移动、掉落。

## 边界

- 包含所有游戏实体类型和交互逻辑
- 不包含网络包处理（在 `net`）
- 不包含角色属性/Buff 管理（在 `client`）
- 不包含脚本引擎管理（在 `scripting`）

## 关键抽象

| 抽象 | 位置 | 职责 |
|---|---|---|
| `MapleMap` | `server.maps.MapleMap` | 地图实例：实体管理、重生点、掉落物、玩家/怪物集合 |
| `MapleMonster` | `server.life.MapleMonster` | 怪物实体：HP/状态/仇恨/AI/技能 |
| `MapleItemInformationProvider` | `server.MapleItemInformationProvider` | 物品数据库（WZ 加载），提供物品属性/脚本/需求 |
| `MapleStatEffect` | `server.MapleStatEffect` | 技能/Buff 效果计算（伤害/治疗/Buff） |
| `MapleQuest` | `server.quest.MapleQuest` | 任务系统：WZ 数据驱动 + 条件/动作 |
| `TimerManager` | `server.TimerManager` | 全局定时任务调度器（ScheduledThreadPoolExecutor 封装） |

## 子包

| 子包 | 文件数 | 职责 |
|---|---|---|
| `server.maps/` | 35 | 地图系统：MapleMap/MapFactory/Reactor/Summon/Door/PlayerShop/Merchant/Mist/Dragon |
| `server.life/` | 22 | 生命实体：Monster/NPC/PlayerNPC/LifeFactory/MobSkill |
| `server.quest/` | 5 + actions/ + requirements/ | 任务：WZ 数据 + 19 种条件 + 12 种动作 |
| `server.events/` | 3 + gm/ (7) | GM 事件：OX/Fitness/Ola/Snowball/Coconut/Gaga |
| `server.gachapon/` | 13 | 扭蛋机按城镇分表 |
| `server.expeditions/` | 2 | BOSS 远征队 |
| `server.partyquest/` | 4 | 怪物嘉年华/金字塔 |
| `server.movement/` | 9 | 移动数据解析（绝对/相对/传送/跳跃/椅子/换装） |
| `server.loot/` | 2 | 掉落分配 |

## 主流程

### 地图实体管理

`MapleMap` 维护 `mapobjects`（HashMap<Integer, MapleMapObject>），通过 `runningOid`（AtomicInteger）分配唯一对象 ID。玩家进入/离开地图、怪物重生、掉落物生成/拾取均操作此集合。

### 怪物生命周期

`MapleLifeFactory.getMonster(id)` → 创建怪物 → `MapleMap.spawnMonster()` → 怪物 AI 定时触发 → 死亡掉落 → 清理

### 任务系统

`MapleQuest.loadAllQuest()` 加载 WZ → 玩家接取/完成任务时检查 requirements（等级/职业/物品/击杀数等）→ 执行 actions（经验/物品/技能/Buff 等）

### 交易/商店

`MapleTrade`（面对面交易）和 `MaplePlayerShop`/`MapleHiredMerchant`（自由市场）管理买卖逻辑。

## 并发模型

- `MapleMap`：chrRLock/chrWLock + objectRLock/objectWLock（ReadWriteLock）+ lootLock
- `MapleMonster`：externalLock/monsterLock/statiLock/animationLock/aggroUpdateLock（5 把锁）
- 95 处 `DatabaseConnection.getConnection()` 散落调用，无事务边界

## 依赖

- → `client`：操作 MapleCharacter 状态
- → `provider`：WZ 数据加载（地图/怪物/物品/任务/反应堆）
- → `tools`：DatabaseConnection、MaplePacketCreator
- ← `net`：Handler 调用 server 层 API
- ← `scripting`：脚本调用 server 层 API

## 关键文件

| 文件 | 行数 | 说明 |
|---|---|---|
| `MapleCharacter.java` | 10,626 | 角色完整状态（位于 client 包） |
| `MaplePacketCreator.java` | 8,790 | 所有出站包构建（位于 tools 包） |
| `MapleItemInformationProvider.java` | 2,379 | 物品数据库 |
| `MapleStatEffect.java` | 1,716 | 技能/Buff 效果 |
| `MapleMap.java` | 4,143 | 地图实例 |
| `MapleMonster.java` | 2,096 | 怪物实体 |
