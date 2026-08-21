Commit: 5391bb19e837afeb65f332262d0d80b882b0c952

# 数据库配置支持运行时覆盖

## Context

linked server 的 `configuration.ini` 包含数据库配置，但生产凭据不能写入 Git；生产进程需要直接从该仓库构建和运行。

## Change Summary

- `ServerConstants` 继续以 `configuration.ini` 为默认值。
- `MAPLE_DB_URL`、`MAPLE_DB_USER`、`MAPLE_DB_PASS` 的非空值可分别覆盖数据库连接参数。

## Impact Surface

- 服务启动时的数据库连接配置解析。

## Notes / Compatibility

- 未设置环境变量时行为不变。
- 未修改数据库结构或数据。

## Related Docs

- [constants 模块](../../../agents/constants/index.md)
