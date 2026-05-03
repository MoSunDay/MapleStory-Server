Commit: ea0bee5e598775e27e5f0e2bd842a4cbc0ce7264

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
| `AbstractPlayerInteraction` | `scripting.AbstractPlayerInteraction` | 基础 API：传送/物品/任务/组队/经验/金币/怪物生成/效果 |
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

## 依赖

- → `client`：操作 MapleCharacter/MapleClient
- → `server`：操作 MapleMap/MapleMonster/MapleItemInformationProvider
- → `net.server`：World/Channel 访问、事件注册
- ← `net`：Handler 触发脚本执行（NPCTalkHandler 等）
