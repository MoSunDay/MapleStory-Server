# 仓库级 Go-live Gate 覆盖（.opencode/golive.md）

> 依据：默认 go-live gate 清单由 `task-plan` skill 拥有；`review` skill 声明「仓库可在
> `AGENTS.md` 或 `.opencode/golive.md` 覆盖此清单；存在时优先遵循仓库规则」。
> 本文件即本仓库的正式覆盖。未在此声明者，其余 gate 一律按默认清单执行。

## 行数 gate 覆盖：既有超限文件豁免

### 背景
全局规则要求「新增文件 ≤ 400 行；迭代中文件 ≤ 800 行」。本仓库 fork 自 HeavenMS，
基线提交时已有 17 个 Java 源文件超过 800 行（最大 `src/client/MapleCharacter.java`
10626 行），属纯上游既有债。拆分它们是独立重构任务，不应阻塞与之无关的小改动
（先例：single-world 1×1 变更仅触 Server.java 1 行文案、净零增长，却因 1927 > 800
被行数 gate 阻塞）。

### 条款
1. **豁免名单**（基线 `ced7f06f`，`wc -l > 800`，共 17 个，见下表）：触及这些文件时
   「迭代中 ≤ 800 行」检查予以豁免——行数不作为阻塞项，review 仅需以一行注明
   「当前行数 / 本次净变化量」（信息性）。
2. **仍生效的硬约束**（不豁免）：
   - 新增文件 ≤ 400 行；
   - 不在豁免名单内的文件，迭代后不得超过 800 行（超限即 ❌，须拆分）；
   - 任何文件不得因增长越过 800 行而自动进入豁免名单；名单修订须用户显式批准
     并修改本文件。
3. **债务跟踪**：豁免名单文件的拆分记为非阻塞债（见下表）；触及它们时鼓励但不强制
   做低成本的局部提取（repair-on-touch 可选）。

### 豁免名单（基线 ced7f06f 实测 wc -l）
| 行数 | 文件 |
|---|---|
| 10626 | src/client/MapleCharacter.java |
| 8790 | src/tools/MaplePacketCreator.java |
| 4143 | src/server/maps/MapleMap.java |
| 2379 | src/server/MapleItemInformationProvider.java |
| 2096 | src/server/life/MapleMonster.java |
| 2058 | src/net/server/world/World.java |
| 1927 | src/net/server/Server.java |
| 1716 | src/server/MapleStatEffect.java |
| 1678 | src/client/MapleClient.java |
| 1510 | src/scripting/event/EventInstanceManager.java |
| 1194 | src/net/server/channel/Channel.java |
| 1096 | src/scripting/AbstractPlayerInteraction.java |
| 955 | src/scripting/event/EventManager.java |
| 898 | src/net/server/channel/handlers/AbstractDealDamageHandler.java |
| 872 | src/net/server/channel/handlers/MTSHandler.java |
| 846 | src/client/processor/AssignAPProcessor.java |
| 821 | src/net/server/guild/MapleGuild.java |

## 登记在册的非阻塞债（不阻塞 go-live verdict）
| 债务 | 状态 | 备注 |
|---|---|---|
| Server.java（1927 行）按职责拆分：world-init / channel 管理 / 排名等 | 待办 · 独立任务 | 由行数债评审 gap 1 移交 |
| rust_workspace clippy 既有错误 35 处（`-D warnings` exit 101） | 待办 · 独立任务 | 先于 single-world 任务存在，与业务改动零关联 |
