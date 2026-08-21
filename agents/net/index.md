Commit: 13254d0db7000b8952b8bc1a0e1532a63ea2a46b

# net 模块 — 网络层与服务器生命周期

## 职责

网络 I/O、协议编解码、包分发、登录/频道服务器生命周期、世界/频道管理、会话协调、全局定时任务。

## 边界

- 不包含游戏逻辑（怪物 AI、技能效果等在 `server` 模块）
- 不包含角色域模型（在 `client` 模块）
- 不包含脚本执行（在 `scripting` 模块）

## 关键抽象

| 抽象 | 位置 | 职责 |
|---|---|---|
| `Server` | `net.server.Server` | 单例入口，管理世界/频道/公会/联盟生命周期，注册全局定时任务 |
| `World` | `net.server.world.World` | 逻辑世界：频道列表、组队/信使/家庭/公会摘要/角色视图 |
| `Channel` | `net.server.channel.Channel` | 独立 NIO acceptor，拥有 MapleMapFactory、事件脚本、雇佣商人 |
| `MapleServerHandler` | `net.MapleServerHandler` | MINA IoHandler：会话创建/销毁、包路由 |
| `PacketProcessor` | `net.PacketProcessor` | opcode 索引的处理器查找表，登录/频道两套 |
| `MapleCodecFactory` | `net.mina.MapleCodecFactory` | MINA codec：AES-OFB + 自定义字节滚动加密/解密 |

## 主流程

### 启动

1. `Server.main()` → `Server.init()`：加载 `world.ini`，重置 DB 状态，创建 MINA acceptor（端口 8484）
2. 启动 `ThreadManager`（线程池）和 `TimerManager`（定时任务）
3. 注册全局 Worker：疾病、排名、优惠券、锁释放、登录协调等
4. 加载技能/任务/现金道具 WZ 数据
5. 按 `world.ini` 初始化 World → Channel（每个 Channel 独立端口）

### 包处理

```
Client TCP → NioSocketAcceptor → MaplePacketDecoder（解密）
    → MapleServerHandler.messageReceived()
    → PacketProcessor[opcode].handlePacket()
    → 具体 Handler（如 CloseRangeDamageHandler）
    → MaplePacketCreator 构建响应 → MaplePacketEncoder（加密）→ Client
```

### 登录自动注册

`LoginPasswordHandler` 在 `AUTOMATIC_REGISTER` 开启且账号不存在时，只接受 4–12 位 ASCII 字母数字账号和 4–12 位非空白可打印 ASCII 密码；合法凭据以 BCrypt cost 12 写入账号表，再重试原登录流程。新账号先返回 TOS 状态，`AcceptToSHandler` 接受后完成认证。IP 封禁检查先于建号，MAC 封禁检查仍在认证结果处理阶段；邮箱登录处理器不会触发自动建号。

### 包处理器分类

| 类别 | 包路径 | 处理器数量 | 典型处理器 |
|---|---|---|---|
| 登录 | `net.server.handlers.login/` | ~17 | LoginPasswordHandler, CreateCharHandler, ServerlistRequestHandler |
| 频道 | `net.server.channel.handlers/` | ~130 | PlayerLoggedinHandler, CloseRangeDamageHandler, MovePlayerHandler, NPCTalkHandler |
| 通用 | `net.server.handlers/` | 3 | KeepAliveHandler, CustomPacketHandler |

`PLAYER_DC(0x0C)` 在登录服和频道服共用 `PlayerDisconnectHandler`：服务端收到标准登出请求后主动关闭 MINA 会话，使 `MapleServerHandler.sessionClosed` 统一执行角色保存、在线状态释放和会话协调器清理。

## 并发模型

- `Server`：srvLock / wldLock(ReadWriteLock) / lgnLock(ReadWriteLock) / disLock
- `World`：chnLock / partyLock / accountCharsLock / suggestLock / srvMessagesLock + 多个子系统锁
- `Channel`：merchantLock / lock / faceLock[] 分区锁（7 个调度器 × CHANNEL_LOCKS）
- `MapleClient`：actionsSemaphore(7) / encoderLock / loginLocks[200] 分片
- 全局 Worker 通过 `TimerManager`（ScheduledThreadPoolExecutor, 4 核心线程）调度

## 子包

| 子包 | 职责 |
|---|---|
| `net.mina/` | MINA codec（加密/解密） |
| `net.opcodes/` | RecvOpcode / SendOpcode 枚举 |
| `net.server.audit/` | 锁审计与死锁检测（开发调试用） |
| `net.server.channel.handlers/` | ~130 个频道包处理器 |
| `net.server.channel.worker/` | 频道级定时任务：怪物状态/动画/事件 |
| `net.server.coordinator/` | 会话协调：多客户端检测、邀请追踪、仇恨管理 |
| `net.server.guild/` | 公会/联盟管理 |
| `net.server.handlers.login/` | 登录包处理器 |
| `net.server.worker/` | 全局后台 Worker（~20 个）：自动保存、宠物饱食、排名等 |
| `net.server.world/` | World 管理：组队/信使/家庭 |

## 依赖

- → `client`：Handler 操作 MapleClient / MapleCharacter
- → `server`：Handler 操作 MapleMap / MapleMonster / MapleItemInformationProvider
- → `tools`：MaplePacketCreator / DatabaseConnection / MapleAESOFB
- → `scripting`：加载/触发 NPC/事件脚本
- ← 无入向依赖（最外层接线模块）

## 关键文件

| 文件 | 行数 | 说明 |
|---|---|---|
| `net/server/Server.java` | 1,927 | 单例入口，世界/频道生命周期 |
| `net/server/world/World.java` | 2,058 | 世界管理，10+ 子系统 |
| `net/server/channel/Channel.java` | 1,195 | 频道服务器，独立 NIO |
| `client/MapleClient.java` | 1,644 | 客户端会话（逻辑上属 net 层） |
