Commit: ea0bee5e598775e27e5f0e2bd842a4cbc0ce7264

# tools 模块 — 基础设施

## 职责

底层工具：包构建、加密、数据库连接池、日志、随机数、字节流 I/O。

## 边界

- 不包含游戏逻辑
- 不包含网络协议处理
- 纯工具层，被所有模块依赖

## 关键抽象

| 抽象 | 位置 | 职责 |
|---|---|---|
| `MaplePacketCreator` | `tools.MaplePacketCreator` | 构建所有出站包（8,790 行，纯静态方法） |
| `DatabaseConnection` | `tools.DatabaseConnection` | HikariCP MySQL 连接池 |
| `MapleAESOFB` | `tools.MapleAESOFB` | AES-OFB 流密码 |
| `MaplePacketLittleEndianWriter` | `tools.data.output` | 小端字节流写入器 |
| `SeekableLittleEndianAccessor` | `tools.data.input` | 小端字节流读取器 |

## 子包

| 子包 | 职责 |
|---|---|
| `tools.data/input/` | 小端字节流读取器（包解析用） |
| `tools.data/output/` | 小端字节流写入器（包构建用） |
| `tools/packets/` | 功能包辅助（钓鱼/结婚） |
| `tools/exceptions/` | 自定义异常 |

## 其他工具类

| 类 | 职责 |
|---|---|
| `BCrypt` | 密码哈希 |
| `FilePrinter` | 错误/日志文件输出 |
| `MapleLogger` | 封包日志 |
| `HexTool` | 字节→十六进制 |
| `StringUtil` | 字符串工具 |
| `Pair` | 二元组 |
| `Randomizer` | 线程安全随机数 |
| `AutoJCE` | 移除 JCE 加密限制 |

## 依赖

- 无出向依赖（基础模块）
- ← 被所有其他模块依赖
