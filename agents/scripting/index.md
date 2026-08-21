Commit: 327d5c87ecb0d83ee8f9fc360d8c3df656d1a4f7

# scripting 模块 — JS 脚本引擎桥接

## 职责

通过 javax.script（Nashorn/Rhino）执行 JavaScript 脚本，为 NPC 对话、事件实例、任务、传送门、反应堆、道具、地图入口提供可扩展的内容层。

## 边界

- 包含脚本引擎管理和 Java→JS API 桥接
- 不包含游戏逻辑（调用 server/client 层 API）
- 不包含脚本文件内容（`scripts/` 目录）

## 关键抽象

| 抽象 | 位置 | 职责 |
|---|---|---|
| `AbstractScriptManager` | `scripting.AbstractScriptManager` | 基类：创建 ScriptEngine、加载/缓存 .js 文件 |
| `AbstractPlayerInteraction` | `scripting.AbstractPlayerInteraction` | 基础 API：传送/物品/任务/组队/经验/金币/怪物生成/效果，并承接旧脚本兼容方法 |
| `NPCConversationManager` | `scripting.npc.NPCConversationManager` | NPC 对话 API：sendNext/sendYesNo/sendSimple 等 + 职业转职/外观 |
| `EventInstanceManager` | `scripting.event.EventInstanceManager` | 事件实例 API：玩家注册/地图操作/定时器/奖励/队伍管理 |
| `EventManager` | `scripting.event.EventManager` | 事件管理：创建实例/定时回调/属性 |

## 脚本绑定

| 脚本类型 | JS 全局变量 | 绑定类 | Manager | 脚本数量 |
|---|---|---|---|---|
| NPC | `cm` | `NPCConversationManager` | `NPCScriptManager` | 691 |
| 事件 | `em` | `EventManager` | `EventScriptManager` | 86 |
| 任务 | `qm` | `QuestActionManager` | `QuestScriptManager` | 257 |
| 传送门 | `pi` | `PortalPlayerInteraction` | `PortalScriptManager` | 450 |
| 反应堆 | `rm` | `ReactorActionManager` | `ReactorScriptManager` | 287 |
| 地图 | `ms` | `MapScriptMethods` | `MapScriptManager` | 88 |

## 脚本模式

### NPC/任务 — 状态机模式

```javascript
var status = -1;
function start() { action(1, 0, 0); }
function action(mode, type, selection) {
    if (mode == -1) cm.dispose();    // ESC
    else if (mode == 1) status++;    // Next/Yes
    else status--;                    // Prev/No
    // if (status == N) cm.sendXxx(...)
}
```

### 事件 — 回调模式

```javascript
function init() {}                    // 服务器启动
function setup(level, lobbyid) {}     // 创建实例
function playerEntry(eim, chr) {}     // 玩家进入
function monsterKilled(mob, eim) {}   // 怪物击杀
function scheduledTimeout(eim) {}     // 定时器到期
function dispose(eim) {}              // 实例销毁
```

### 传送门 — 单函数

```javascript
function enter(pi) { pi.warp(map, portal); return true; }
```

### 反应堆 — 回调

```javascript
function act() {}    // 状态变更
function hit() {}    // 被击中
```

## 状态管理

- **NPC/任务脚本**：每个客户端会话缓存独立 ScriptEngine，`dispose()` 时释放
- **事件脚本**：启动时加载一次，多个 EIM 实例共享引擎，通过 EIM 参数区分状态
- **传送门/地图脚本**：全局编译缓存，所有玩家共享
- **EIM 状态**：Properties（键值对）、chars Map、killCount Map、定时器、标志位

## 兼容接口

- `AbstractPlayerInteraction` 是 `cm`、`qm`、`pi`、`ms` 的共同兜底接口；旧脚本常用的提示、教程图、临时 NPC、反应堆触发、怪物计数、事件入场、地铁平台入口与队伍传送方法应在这里保持可调用。
- 对缺少可靠 v83 封包实现的旧 direction/camera 控制方法，当前策略是安全 no-op 或复用已有展示封包，优先保证脚本不异常、玩家交互不中断。
- `NPCConversationManager` 继续承接 NPC/Quest 共享的职业与任务辅助接口，包含 `forceStartQuest(int, String)` 这类旧 quest 脚本进度写入重载。
- 缺失脚本桥接方法或重载不匹配会表现为点击 NPC/触发任务后无对话框，排查时优先查看 `logs/YYYY-MM-DD/error/game/npcs/*.txt` 与 `game/mapscript/*.txt`，并复跑 `cm/qm/pi/ms` 方法名与参数数量扫描。

## 依赖

- → `client`：操作 MapleCharacter/MapleClient
- → `server`：操作 MapleMap/MapleMonster/MapleItemInformationProvider
- → `net.server`：World/Channel 访问、事件注册
- ← `net`：Handler 触发脚本执行（NPCTalkHandler 等）
