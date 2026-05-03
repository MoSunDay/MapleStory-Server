Commit: ea0bee5e598775e27e5f0e2bd842a4cbc0ce7264

# client 模块 — 玩家域模型

## 职责

玩家/角色完整状态模型：连接会话、角色属性、背包、技能、任务、GM 命令、自动封禁检测。

## 边界

- 包含角色的数据模型和持久化逻辑
- 不包含地图实体交互逻辑（在 `server.maps`）
- 不包含网络包解析（在 `net.server.channel.handlers`）

## 关键抽象

| 抽象 | 位置 | 职责 |
|---|---|---|
| `MapleCharacter` | `client.MapleCharacter` | 角色完整实体：属性/背包/技能/Buff/任务/移动/升级/DB 持久化 |
| `MapleClient` | `client.MapleClient` | 连接会话：AES 状态/登录状态机/账号信息/脚本引擎缓存 |
| `Skill` / `SkillFactory` | `client.Skill` | 技能数据模型和 WZ 加载 |
| `MapleJob` | `client.MapleJob` | 职业枚举 |
| `MapleInventory` | `client.inventory.MapleInventory` | 背包（装备/消耗/设置/其他四栏） |

## 主流程

### 角色加载

`MapleCharacter.loadCharFromDB(id, client, channel)` → 从 DB 读取角色全部数据 → 构建完整角色对象

### 角色保存

`MapleCharacter.saveCharToDB()` → 将全部状态写回 DB（属性/背包/技能/任务/Buff 等）

### GM 命令

`CommandsExecutor.execute()` → 按权限级别分发到 `command/commands/gm0..gm6/` 下的命令类

## 子包

| 子包 | 文件数 | 职责 |
|---|---|---|
| `client.autoban/` | 2 | 自动作弊检测（攻击频率/移动速度异常） |
| `client.command/` | ~3 + commands/ | GM 命令框架 + 7 级命令实现 |
| `client.creator/` | ~10 | 角色创建工厂（6 种初始职业） |
| `client.inventory/` | ~15 | 物品/背包/宠物/操纵器 |
| `client.processor/` | 8 | 客户端操作处理器（AP/SP/Duey/Maker/Storage/Buyback/Pet/Fredrick） |
| `client.status/` | 2 | 怪物状态效果（来自玩家技能） |

## 并发模型

- `MapleCharacter`：5 把锁（chrLock/evtLock/petLock/prtLock/cpnLock）+ 3 个 Atomic 字段
- `MapleClient`：Semaphore(7) 限制操作频率 + encoderLock + 分片 loginLocks[200]
- 角色 DB 保存通过 `ThreadManager` 异步执行

## 依赖

- → `server`：MapleStatEffect、MapleItemInformationProvider、MapleMap
- → `net.server`：PlayerStorage、World 访问
- → `scripting`：脚本引擎缓存
- → `provider`：Skill WZ 数据
- → `tools`：DatabaseConnection、MaplePacketCreator

## 关键文件

| 文件 | 行数 | 说明 |
|---|---|---|
| `MapleCharacter.java` | 10,626 | 最大文件，角色完整状态 |
| `MapleClient.java` | 1,644 | 客户端会话 |
| `Skill.java` | ~700 | 技能数据模型 |
