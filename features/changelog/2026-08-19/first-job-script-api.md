Commit: 327d5c87ecb0d83ee8f9fc360d8c3df656d1a4f7

# 脚本桥接与转职 NPC 交互

日期: 2026-08-19

## 变更

- `AbstractPlayerInteraction#getFirstJobStatRequirement(int)` 作为实例 API 暴露给 JS 脚本。
- 使用 `cm`/`qm` 的一转 NPC 与任务脚本可以读取职业属性门槛文案，不再因方法不可见中断对话。
- 一转条件本身不变：战士 STR 35，魔法师 INT 20，弓手/飞侠 DEX 25，海盗 DEX 20。
- 补齐旧 NPC、任务、传送门和地图脚本仍在调用的常用桥接方法，包括提示消息、教程图片、临时 NPC、反应堆触发、怪物计数、事件入场与队伍传送等。
- 2/3/4 转职相关 NPC 和任务脚本经过语法与脚本桥接扫描，不再存在 `cm`/`qm` 缺失方法导致的点击无框风险。
- 服务端启动脚本优先使用 `JAVA_HOME`，并回退到当前环境存在的 JRE 11 路径，避免后续重启使用不存在的 OpenJDK 路径。
- `9000011.js` 改用事件地图 ID 与事件入场 helper，不再把 `Channel#getEvent()` 对象当作地图 ID 比较或传送。
- Portal/Map 脚本桥接补齐教程演出相关兼容方法，避免进入教程地图或传送门时因 `pi`/`ms` 方法缺失中断。
- Mr. Lim 地铁脚本入口现在会选择空闲 Dusty Platform / Train 999 地图并传送同图队伍或单人；如果无空闲房间则沿用脚本原有提示。
- `forceStartQuest(int, String)`、NPC 特效/音效重载、宠物亲密度指定槽位重载已补齐，避免旧任务和活动 NPC 在后续步骤触发重载不匹配。
- 事件入口 NPC 先确认当前频道存在可进入事件，再调用共享入场 helper；只有成功入场才写入入场冷却时间。
- 传送门和地图教程脚本所需的 `lockUI2`、`setDirectionStatus`、`startDirection`、`setNPCValue`、`showWZEffect` 等兼容入口已补齐，避免相邻交互入口因旧脚本方法缺失中断。

## 影响面

- 冒险家一转导师 NPC，例如 Dances with Balrog。
- 骑士团一转任务脚本中复用的属性门槛提示。
- 二转教官、三转导师、四转导师以及会在转职链路前后触发的通用 NPC/任务/地图入口脚本。

## 验证

- 服务端全量 Java 编译通过。
- 实际加载的 `scripts/npc`、`scripts/quest`、`scripts/portal`、`scripts/map` 经 `cm`/`qm`/`pi`/`ms` 方法名和参数数量扫描，结果无缺口。
- 2/3/4 转职关键 NPC 与任务脚本 `node --check` 通过。
- 重启 Java 游戏服务后，进程从仓库服务端目录启动并监听登录/频道端口。
- 重启日志不再出现 JDK 路径错误，当天未生成新的脚本错误日志。
- 最终运行进程启动时间晚于最新 `dist/scripting/*.class` 编译时间，确认运行态已加载本次编译产物。
