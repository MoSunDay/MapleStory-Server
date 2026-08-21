Commit: 76ab209138d501b78a165bdbc696836ab0bfa65b

# provider 模块 — WZ 数据提供层

## 职责

读取 MapleStory WZ（Wizet）游戏资源文件，提供统一的树形数据访问接口。支持二进制 `.wz` 格式和 XML 预提取格式。

## 边界

- 纯数据读取层，无状态变更
- 不包含游戏逻辑
- 不包含网络/数据库交互

## 关键抽象

| 抽象 | 位置 | 职责 |
|---|---|---|
| `MapleDataProvider` | `provider.MapleDataProvider` | 接口：`getData(path)` / `getRoot()` |
| `MapleDataProviderFactory` | `provider.MapleDataProviderFactory` | 工厂：按文件类型创建 WZFile 或 XMLWZFile |
| `MapleData` | `provider.MapleData` | 接口：树形数据节点（子节点/类型/名称） |
| `MapleDataTool` | `provider.MapleDataTool` | 工具：从 MapleData 提取类型化值（int/string/image 等） |

## WZ 格式实现（provider/wz/）

| 类 | 职责 |
|---|---|
| `WZFile` | 二进制 .wz 文件读取（解密+解析） |
| `XMLWZFile` | XML 格式 WZ 数据读取 |
| `XMLDomMapleData` | DOM 方式实现 MapleData |
| `PNGMapleCanvas` | PNG 图片画布 |
| `WZTool` | WZ 解密/密钥工具 |

## 数据类型

`MapleDataType` 枚举：int / float / string / img / vector / null / uol

## 依赖

- → `tools`：字节流 I/O
- ← `server`：加载地图/怪物/物品/任务/反应堆数据
- ← `client`：加载技能数据

## 数据加载入口

| 数据 | 加载者 | WZ 源 |
|---|---|---|
| 技能 | `SkillFactory.loadAllSkills()` | Skill.wz |
| 物品 | `MapleItemInformationProvider` | Item.wz / Character.wz |
| 任务 | `MapleQuest.loadAllQuest()` | Quest.wz |
| 地图 | `MapleMapFactory`（懒加载） | Map.wz |
| 怪物/NPC | `MapleLifeFactory` | Mob.wz / NPC.wz / String.wz |
| 现金道具 | `CashItemFactory` | Cash.wz（估算） |
| 反应堆 | `MapleReactorFactory` | Reactor.wz |
