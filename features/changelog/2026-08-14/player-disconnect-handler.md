Commit: 13254d0db7000b8952b8bc1a0e1532a63ea2a46b

# 标准 PLAYER_DC 主动登出

## Context

`RecvOpcode` 已声明 `PLAYER_DC(0x0C)`，但登录服和频道服均未注册处理器。客户端关闭传输后，账号在线状态可能不能及时释放。

## Change Summary

- 登录服与频道服共同注册 `PlayerDisconnectHandler`。
- 收到标准 `PLAYER_DC` 后由服务端主动关闭 MINA 会话，复用既有断连清理和角色保存流程。

## Impact Surface

- 账号主动登出、在线状态释放和角色持久化。

## Notes / Compatibility

- 未新增私有 opcode；未修改数据库结构或删除数据。

## Related Docs

- [net 模块](../../../agents/net/index.md)
