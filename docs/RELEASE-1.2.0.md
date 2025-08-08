### v1.2.0 发行公告（中文）

#### 亮点
- **国际化**：支持中英双语，运行时切换并持久化到后端。
- **AI 搜索优化**：接入 LLM 对结果进行分析、评分和排序；对标题去广告并规范化，自动打标签（如 `2160p`、`Chinese`、`Chinese Sub`、`WEB-DL`）。
- **批量分析与回退**：并行批处理，失败自动降级为单项分析，实时展示进度。
- **关键词优先**：含指定关键词（如 4K、中文字幕）的结果优先展示。
- **一键下载**：新增下载按钮，可跳转到常用下载器或 115 离线下载页，并可配置自动关闭。
- **可扩展搜索**：优化自定义搜索引擎的网页抓取逻辑，更稳更快。
- **UI 与可用性**：设置页新增“调试区域”并提供开关；中文界面下左侧导航与“设置”标题字号更大。
- **跨平台**：提供 Windows、Linux、macOS 构建（Linux/macOS 暂未在真机充分测试，欢迎反馈）。

#### 变更与修复
- 设置持久化：新增 `show_debug_area` 字段并前后端打通，默认关闭。
- 后端 i18n：新增多语言支持模块与命令，错误信息可本地化；前端 i18n 预加载与语言初始化完善。
- 安装包命名统一：Windows 产物统一为 `AI Magnet Assistant_1.2.0_x64-setup.{msi,exe}`。
- 安全：从 Git 历史中彻底移除 `memory_bank/` 并加入 `.gitignore`。
- 文档：更新 README/RELEASES/ARCHITECTURE/DEVELOPER_MANUAL，版本同步至 1.2.0。

#### 开发者与 CI
- CI 测试打包：GitHub Actions 三平台矩阵构建并上传产物（不自动发布）；按标签生成 Draft/Pre-release，并附 SHA256 校验。
- 脚本：新增 `run/package-win.sh` 统一 Windows 产物命名；`run/setup.sh` 更健壮。

#### 升级提示
- 仓库历史已重写以清除敏感内容。协作者请执行：`git fetch origin && git reset --hard origin/master`（或重新克隆）。
- 使用自定义搜索引擎的用户，建议在 1.2 下重新测试抓取效果。
- Linux/macOS 构建为首次提供，欢迎反馈兼容性问题。

---

### v1.2.0 Release Notes (English)

#### Highlights
- **Internationalization**: Full English/Chinese with runtime switching and backend persistence.
- **AI Optimization**: LLM analyzes, scores, and ranks results; cleans/normalizes titles; auto‑tags (e.g., `2160p`, `Chinese`, `Chinese Sub`, `WEB‑DL`).
- **Batch Analysis + Fallback**: Parallel batches with graceful fallback to single‑item analysis and live progress.
- **Keyword Boosting**: Results containing preferred keywords (e.g., 4K, Chinese subtitles) are prioritized.
- **One‑Click Download**: Button to open your downloader or 115 offline page; optional auto‑close.
- **Extensible Engines**: Improved scraping logic for custom engines (more robust and faster).
- **UI/UX**: “Debug Area” with a toggle on Settings; larger fonts for Chinese locale in side nav and Settings title.
- **Cross‑platform**: Ship Windows, Linux, macOS builds (Linux/macOS not fully tested on real devices yet).

#### Changes & Fixes
- Settings persistence: added `show_debug_area` with frontend/backend wiring; default off.
- Backend i18n: locale storage and translation commands; frontend i18n preload/init.
- Artifact naming: unified Windows artifacts to `AI Magnet Assistant_1.2.0_x64-setup.{msi,exe}`.
- Security: purged `memory_bank/` from git history and added to `.gitignore`.
- Docs: updated README/RELEASES/ARCHITECTURE/DEVELOPER_MANUAL; version set to 1.2.0.

#### Dev & CI
- CI builds: GitHub Actions matrix (Win/Linux/macOS) uploads artifacts (no auto publish); tag-triggered workflow drafts a pre‑release with SHA256 sums.
- Scripts: `run/package-win.sh` normalizes Windows artifact names; `run/setup.sh` hardened.

#### Upgrade Notes
- History was rewritten to remove sensitive data. Please resync: `git fetch origin && git reset --hard origin/master` (or re‑clone).
- If you use custom engines, re‑validate scraping under v1.2.
- Linux/macOS builds are provided for convenience; feedback on compatibility is welcome.


