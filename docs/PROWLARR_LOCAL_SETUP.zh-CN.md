# 本地 Prowlarr 配置说明

这份文档说明为什么 AI Magnet Assistant 推荐接入本地 Prowlarr，以及如何配置索引源、代理和 FlareSolverr。它不是必须照抄的唯一方案，而是一套已经跑通的本地配置思路。

## 为什么需要本地 Prowlarr

AI Magnet Assistant 本身负责搜索入口、结果展示、磁力链接整理、AI 分析和收藏管理。真正和各类 BT 索引站打交道的部分，推荐交给 Prowlarr。

这么做有几个好处：

- Prowlarr 已经维护了大量索引站适配器，不需要本应用为每个站点手写解析规则。
- API Key、Cookie、Passkey、账号密码等站点凭据留在 Prowlarr，本应用只保存 Prowlarr 的 API Key。
- Prowlarr 能测试每个索引源是否能连接，能看到失败原因，比如 DNS、SSL、Cloudflare、站点关闭或需要登录。
- 本应用只负责读取 Prowlarr 中已经配置好的索引源，并在界面里启用或禁用这些源。
- 搜索结果里如果 Prowlarr 返回的是本地代理下载地址，本应用会尽量根据 `infoHash` 转成标准 `magnet:?xt=urn:btih:...` 链接。

一句话：Prowlarr 管“源能不能用”，AI Magnet Assistant 管“搜索结果怎么用”。

## 推荐拓扑

```text
AI Magnet Assistant
  -> http://127.0.0.1:9696/api/v1/search
  -> Prowlarr
       -> 普通索引源
       -> Clash / Mihomo HTTP 代理
       -> FlareSolverr 处理 Cloudflare 挑战
       -> 各类公开或私有索引站
```

常见本地端口示例：

| 服务 | 示例地址 | 说明 |
| --- | --- | --- |
| Prowlarr | `http://127.0.0.1:9696` | 本应用连接的搜索聚合 API |
| FlareSolverr | `http://127.0.0.1:8191` | 处理部分 Cloudflare 挑战 |
| Clash/Mihomo HTTP 或 mixed 代理 | `127.0.0.1:7897` | 给可通过代理访问的索引站使用 |

端口不是固定要求，按你的本机代理软件实际端口填写即可。

## 安装 Prowlarr

可以从 Prowlarr 官方 Release 下载：

<https://github.com/Prowlarr/Prowlarr/releases>

Windows 用户可以使用安装版，也可以使用便携版。启动后默认访问：

```text
http://127.0.0.1:9696
```

首次打开 Prowlarr 可能会要求设置认证方式。建议至少设置一个本地管理账号，避免局域网或误暴露端口时被别人访问。

## 获取 Prowlarr API Key

在 Prowlarr 页面中进入：

```text
Settings -> General -> Security -> API Key
```

复制这个 API Key，然后在 AI Magnet Assistant 中添加 Prowlarr 引擎：

```text
搜索引擎 -> 添加新引擎 -> Prowlarr
```

填写示例：

```text
Prowlarr URL: http://127.0.0.1:9696
API Key: 从 Prowlarr 设置页复制
```

注意：

- Prowlarr 登录账号密码和 API Key 不是一回事。
- 本应用只需要 API Key，不需要 Prowlarr 登录账号密码。
- 不要把 API Key 写进 README、截图或提交到 GitHub。

## 添加索引源

现在推荐在 Prowlarr 中添加和测试索引源，而不是让 AI Magnet Assistant 直接批量添加。

原因是 Prowlarr 能清楚显示每个源的失败原因，例如：

- 站点暂时不可用。
- 站点需要账号或 Cookie。
- DNS/SSL 失败。
- 被 Cloudflare 阻挡。
- 当前代理不通。
- 查询成功但没有返回结果。

添加入口：

```text
Prowlarr -> Indexers -> Add Indexer
```

公开中文/亚洲资源可以先尝试：

- ACG.RIP
- Bangumi Moe
- dmhy
- Mikan
- Nyaa.si
- 52BT
- 0Magnet

这些源是否可用取决于地区、DNS、代理和站点状态。能添加不代表长期稳定，不能添加也不一定是配置错了。

## 在 AI Magnet Assistant 中管理索引源

本应用的“搜索引擎 -> 管理 Prowlarr 索引源”只显示 Prowlarr 中已经配置好的索引源。

这里可以做的事情：

- 查看 Prowlarr 已配置索引源。
- 查看每个源当前是否启用。
- 启用或禁用某个索引源。

这里不会做的事情：

- 不显示 Prowlarr 尚未添加的源。
- 不在本应用里批量添加 Prowlarr 源。
- 不显示完整连接失败日志。

连接是否成功、失败原因和站点字段配置，请在 Prowlarr 里查看。这能避免本应用界面变成第二个 Prowlarr，也能减少误导。

## Cloudflare 是什么

Cloudflare 是很多网站使用的防护服务。它可能会在访问页面时加一道浏览器验证，例如“检查浏览器”“JS Challenge”或验证码。

对普通浏览器来说，这通常只是等待几秒；但对 Prowlarr 这种后端请求来说，可能会表现为：

```text
Unable to access site, blocked by Cloudflare Protection
Unexpected response status Forbidden
HTTP 403
HTTP 503
```

这不是本应用本地生成的错误，也不是 Prowlarr 收费问题，而是目标站点启用了反爬或访问保护。

## 使用 FlareSolverr 处理 Cloudflare

FlareSolverr 是一个辅助服务，用浏览器环境帮 Prowlarr 解决部分 Cloudflare 挑战。

下载地址：

<https://github.com/FlareSolverr/FlareSolverr/releases>

启动后默认地址通常是：

```text
http://127.0.0.1:8191
```

可以用浏览器打开或用命令测试：

```powershell
Invoke-RestMethod http://127.0.0.1:8191
```

正常时会看到类似：

```json
{
  "msg": "FlareSolverr is ready!",
  "version": "3.x.x"
}
```

在 Prowlarr 中添加 FlareSolverr：

```text
Settings -> Indexer Proxies -> Add -> FlareSolverr
```

示例：

```text
Name: FlareSolverr 8191
Host: http://127.0.0.1:8191/
Request Timeout: 60 或 120
Tags: flaresolverr
```

然后把需要走 FlareSolverr 的索引源也打上同一个标签。

重要限制：

- FlareSolverr 不能保证所有 Cloudflare 站点都能通过。
- 如果目标站点要求验证码、登录、私有 Cookie，仍然可能失败。
- FlareSolverr 会增加搜索耗时，不建议所有源都无脑走它。

## 使用 Clash 或 Mihomo 代理

如果某些索引站在本地网络不可访问，但浏览器通过 Clash/Mihomo 可以访问，可以在 Prowlarr 中配置 HTTP 或 SOCKS5 代理。

示例端口：

```text
HTTP/mixed: 127.0.0.1:7897
SOCKS5:     127.0.0.1:7898
HTTP:       127.0.0.1:7899
```

以 HTTP 代理为例：

```text
Prowlarr -> Settings -> Indexer Proxies -> Add -> Http
```

示例：

```text
Name: Clash 7897
Host: 127.0.0.1
Port: 7897
Tags: clash7897
```

然后把需要走 Clash 的索引源打上 `clash7897` 标签。

建议：

- 已确认浏览器和 Clash 能访问的源，可以走 Clash。
- 明确被 Cloudflare 阻挡的源，优先走 FlareSolverr。
- 不要把所有源都塞进同一个代理规则里，否则慢源会拖慢整体搜索。

## 为什么不要启用太多索引源

Prowlarr 支持添加很多源，但“全部启用”不一定是好事。

常见问题：

- 某些源响应很慢，搜索会被拖长。
- 某些源长期 403、502、SSL 错误。
- 某些源只适合特定语种或资源类型，搜索无关关键词时没有帮助。
- 成人源、游戏源、音乐源、影视源混在一起，会让结果噪声变多。

AI Magnet Assistant 已经对 Prowlarr 搜索做了分批和超时处理，慢源会被跳过，但仍建议只启用真正有用的源。

## 成人分类开关

本应用的 Prowlarr 引擎有“允许成人分类”开关。

- 关闭：过滤 Prowlarr 分类编号在成人范围内的结果。
- 开启：允许显示成人分类结果。

这个开关只影响本应用展示结果，不会修改 Prowlarr 里索引源本身。

如果你在 Prowlarr 中启用了成人索引源，但本应用没有显示相关结果，请检查：

1. 本应用 Prowlarr 引擎是否开启“允许成人分类”。
2. Prowlarr 对应索引源是否启用。
3. Prowlarr 中直接搜索是否能返回结果。
4. 搜索关键词是否被“标题必须包含关键词”等前端过滤项过滤掉。

## 常见问题

### 为什么 Prowlarr 里能看到源，本应用里看不到未添加源？

这是设计如此。本应用只显示 Prowlarr 已配置的索引源，不再显示尚未添加的源。添加、测试和失败诊断请在 Prowlarr 里完成。

### 为什么 Prowlarr 搜索结果里有 `127.0.0.1:9696/download?...`？

这是 Prowlarr 的本地代理下载地址，不是标准磁力链接。本应用会在有 `infoHash` 时生成标准磁力链接：

```text
magnet:?xt=urn:btih:<infoHash>
```

### 为什么中文资源搜索差？

中文资源覆盖取决于你添加了哪些索引源。很多中文影视或综合站需要账号、Cookie 或私有站权限。公开源能用但覆盖有限，这是索引源生态问题，不是单个搜索应用能完全解决的。

### Jackett 是否必须？

不是。Prowlarr 和 Jackett 职责高度重叠，当前推荐只使用 Prowlarr。后续如果需要 Torznab/Jackett 兼容，可以作为可选功能支持，不建议把两者都内置进应用。

### Prowlarr 是否收费？

Prowlarr 本身是免费开源软件。某些索引站可能要求账号、邀请、捐赠或会员，但那是站点自己的规则，不是 Prowlarr 收费。

## 一套跑通示例

下面是一套本地跑通过的配置思路，端口可按你的机器调整：

```text
Prowlarr:
  URL: http://127.0.0.1:9696

Clash/Mihomo:
  HTTP/mixed: 127.0.0.1:7897

FlareSolverr:
  URL: http://127.0.0.1:8191

Prowlarr Tags:
  clash7897
  flaresolverr

Indexer Proxy:
  Clash 7897 -> tag: clash7897
  FlareSolverr 8191 -> tag: flaresolverr

AI Magnet Assistant:
  Prowlarr URL: http://127.0.0.1:9696
  API Key: 使用 Prowlarr Settings -> General -> Security 中的 API Key
```

添加索引源时：

- 能直接访问或通过 Clash 访问的源，打 `clash7897` 标签。
- 被 Cloudflare 拦截的源，打 `flaresolverr` 标签。
- 连续失败、无结果或与你需求无关的源，在 Prowlarr 中禁用。

最后在 AI Magnet Assistant 中进入：

```text
搜索引擎 -> Local Prowlarr -> 管理 Prowlarr 索引源
```

确认需要参与搜索的源处于“已启用”状态即可。
