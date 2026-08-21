Commit: 327d5c87ecb0d83ee8f9fc360d8c3df656d1a4f7

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

`MapleCharacter.saveCharToDB()` → 捕获发起保存的 `MapleClient` 会话代次；只有账号最新会话能取得保存 permit，并在 permit 生命周期内将属性、背包、技能、任务等完整状态写回 DB。异步保存和断线保存都保留原始发起会话，已被新登录替代的旧角色快照会被拒绝。

### 角色创建

`CreateCharHandler` 按客户端选择分发到 `BeginnerCreator`、`NoblesseCreator` 或 `LegendCreator`，三者通过 `CharacterFactoryRecipe` 生成初始角色并由 `MapleCharacter.insertNewChar()` 落库。普通新角色的 STR/DEX/INT/LUK 默认均为 20；角色创建协议不接收客户端自报四维，服务端配方是最终数据源。

### 组队金币

组队成员拾取金币时，当前地图中在线的每位成员各自获得完整掉落金额，不再按在场人数平分。共享金额由 `PartyRewardPolicy` 纯函数定义，拾取与背包锁语义保持不变。

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
- `MapleClient`：Semaphore(7) 限制操作频率 + encoderLock + 分片 loginLocks[200]；login() 支持顶号：密码正确且账号在线时先踢活跃会话（forceDisconnectAccount），残留 loggedin 脏旗标（重启遗留）直接清零放行
- 角色 DB 保存可通过 `ThreadManager` 异步执行；`SessionSaveFence` permit 覆盖完整数据库事务，新会话建立会等待已经开始的合法保存提交，已开始建立的新会话则会阻止旧保存进入事务

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
