# NoteCli — Rust 备忘录 CLI 需求文档

## 1. 项目概述

NoteCli 是一个基于 Rust 开发的命令行备忘录管理工具。它以本地文件作为存储后端，提供快速的笔记创建、检索、编辑和整理能力。目标是成为开发者在终端中随手记录想法、待办、片段信息的首选工具——轻量、快速、零依赖外部服务。

### 1.1 设计原则

- **离线优先**：所有数据存储在本地，不依赖网络或远程服务
- **纯文本友好**：笔记内容以 Markdown 格式存储，可用任何编辑器直接查看和编辑
- **快速响应**：常用操作（新增、搜索、列表）应在 100ms 内完成
- **可组合性**：输出设计为可被其他 Unix 工具（grep、fzf、bat 等）管道处理
- **跨平台**：支持 Windows / macOS / Linux

---

## 2. 核心功能需求

### 2.1 笔记管理（CRUD）

#### 2.1.1 创建笔记

- 支持 `note add "内容"` 快速创建一条笔记
- 支持 `note add` 不带内容时自动打开系统默认编辑器（$EDITOR / $VISUAL）进行多行编辑
- 创建时自动生成以下元数据：
  - 唯一 ID（UUID v4 或基于时间戳的短 ID）
  - 创建时间（精确到秒，本地时区）
  - 修改时间（初始等于创建时间）
- 创建时可选指定：
  - 标题（`--title` / `-t`）
  - 分类（`--category` / `-c`）
  - 标签（`--tags` / `-T`，多个标签以逗号分隔）
  - 优先级（`--priority` / `-p`，可选值：low / normal / high / urgent）
- 创建成功后输出新笔记的 ID

#### 2.1.2 查看笔记

- `note show <ID>` 显示单条笔记的完整内容
- 显示内容包括：标题、正文、分类、标签、优先级、创建时间、修改时间
- 支持终端彩色输出，关键元数据（ID、优先级、标签）使用颜色高亮
- 支持 `--raw` 参数以纯文本输出（不包含 ANSI 颜色码），方便管道处理
- 当笔记内容为 Markdown 时，支持基础的终端 Markdown 渲染（标题加粗、列表缩进、代码块高亮）

#### 2.1.3 编辑笔记

- `note edit <ID>` 打开系统编辑器编辑笔记正文
- `note edit <ID> --title "新标题"` 直接修改标题
- `note edit <ID> --category "新分类"` 直接修改分类
- `note edit <ID> --tags "tag1,tag2"` 直接替换标签
- `note edit <ID> --priority high` 直接修改优先级
- `note edit <ID> --append "追加内容"` 在笔记末尾追加文本
- 编辑后自动更新修改时间
- 当指定 ID 不存在时，给出明确错误提示

#### 2.1.4 删除笔记

- `note delete <ID>` 删除指定笔记
- 删除前需确认（`--force` / `-f` 跳过确认）
- 支持批量删除：`note delete <ID1> <ID2> <ID3>`
- 删除操作为永久删除（后期可扩展回收站功能）

### 2.2 笔记列表与浏览

#### 2.2.1 列表展示

- `note list` / `note ls` 列出所有笔记
- 默认显示格式为表格，每行包含：短 ID、标题（截断到 50 字符）、分类、优先级、标签、创建时间
- 支持以下排序方式（`--sort` / `-s` 参数）：
  - `created` / `created:asc` / `created:desc` — 按创建时间（默认 desc）
  - `modified` / `modified:asc` / `modified:desc` — 按修改时间
  - `title` / `title:asc` / `title:desc` — 按标题字母序
  - `priority` / `priority:asc` / `priority:desc` — 按优先级
- 支持分页：`--limit` / `-n` 控制每页条数，`--offset` / `-o` 控制偏移
- 当没有任何笔记时，显示友好的空状态提示

#### 2.2.2 筛选过滤

- `--category <分类>` / `-c` 按分类筛选
- `--tag <标签>` / `-T` 按标签筛选（可多次指定，默认取交集）
- `--priority <优先级>` / `-p` 按优先级筛选
- `--date <日期表达式>` 按创建日期筛选，支持以下格式：
  - `2024-01-15` — 精确日期
  - `2024-01` — 某月全部
  - `today` / `yesterday` — 相对日期
  - `last-7d` / `last-30d` — 近 N 天
- `--has-tag` 仅显示有标签的笔记
- `--no-tag` 仅显示无标签的笔记
- 多个筛选条件之间为 AND 关系

### 2.3 搜索功能

- `note search <关键词>` 全文搜索笔记内容
- 搜索范围覆盖：标题、正文、分类、标签
- 匹配结果中高亮显示关键词
- 支持以下搜索模式（`--mode` 参数）：
  - `plain` — 纯文本匹配（默认）
  - `regex` — 正则表达式匹配
  - `fuzzy` — 模糊匹配
- 搜索结果按相关度排序（匹配次数、匹配位置权重）
- 支持 `--case-sensitive` 大小写敏感搜索（默认不敏感）

---

## 3. 分类与标签系统

### 3.1 分类（Category）

- 分类为层级结构，支持多级嵌套，使用 `/` 分隔（如 `work/projects/alpha`）
- `note category list` 列出所有分类及其笔记数量
- `note category tree` 以树形结构展示分类层级
- `note category rename <旧分类> <新分类>` 重命名分类（该分类下所有笔记同步更新）
- `note category delete <分类>` 删除分类（需指定 `--force` 或先清空该分类下笔记）
- 未指定分类的笔记归入默认分类 `default`

### 3.2 标签（Tag）

- 标签为扁平结构，无层级
- `note tag list` 列出所有标签及其使用次数
- `note tag rename <旧标签> <新标签>` 重命名标签（所有引用同步更新）
- `note tag delete <标签>` 从所有笔记中移除该标签
- 标签名限制：小写字母、数字、连字符（`-`），最大长度 64 字符
- 每条笔记最多 20 个标签

---

## 4. 数据存储

### 4.1 存储结构

- 默认数据目录：`~/.notecli/`（可通过环境变量 `NOTECLI_HOME` 或配置文件覆盖）
- 目录结构：
  ```
  ~/.notecli/
  ├── config.toml          # 用户配置文件
  ├── notes/               # 笔记数据目录
  │   ├── <id>.md          # 每条笔记一个 Markdown 文件
  │   └── ...
  └── index.json           # 元数据索引文件（加速查询）
  ```
- 每条笔记的 Markdown 文件采用 front matter 格式存储元数据：
  ```markdown
  ---
  id: "a1b2c3d4"
  title: "会议纪要"
  category: "work/meetings"
  tags: ["weekly", "project-alpha"]
  priority: "normal"
  created: "2024-01-15T14:30:00+08:00"
  modified: "2024-01-15T16:45:00+08:00"
  ---
  
  笔记正文内容...
  ```

### 4.2 索引机制

- 维护一个 JSON 格式的索引文件，缓存所有笔记的元数据
- 在笔记增删改时同步更新索引
- 提供 `note index rebuild` 命令手动重建索引（修复不一致）
- 程序启动时检测索引是否与文件系统一致，不一致时自动提示重建

### 4.3 数据完整性

- 文件写入采用原子操作（写入临时文件后 rename），防止写入中断导致数据损坏
- 笔记 ID 与文件名一一对应，不允许重复
- 并发访问时使用文件锁（`~/.notecli/.lock`）防止数据竞争

---

## 5. 配置系统

### 5.1 配置文件

- 配置文件路径：`~/.notecli/config.toml`
- 首次运行时自动创建默认配置文件
- 支持的配置项：

  ```toml
  [general]
  default_editor = "vim"          # 默认编辑器
  default_priority = "normal"     # 默认优先级
  default_category = "default"    # 默认分类
  language = "zh-CN"              # 界面语言
  pager = "less"                  # 长内容分页器（设为 "" 禁用）

  [display]
  color = true                    # 是否启用彩色输出
  date_format = "%Y-%m-%d %H:%M"  # 日期显示格式
  table_style = "compact"         # 表格样式：compact / expanded / markdown
  title_max_width = 50            # 列表中标题最大显示宽度

  [storage]
  notes_dir = "~/.notecli/notes"  # 笔记存储目录
  index_file = "~/.notecli/index.json"  # 索引文件路径

  [search]
  default_mode = "plain"          # 默认搜索模式
  case_sensitive = false          # 默认大小写敏感
  max_results = 50                # 最大搜索结果数
  ```

### 5.2 配置优先级

命令行参数 > 配置文件 > 内置默认值

---

## 6. 导入与导出

### 6.1 导出

- `note export --format json` 导出为 JSON 格式
- `note export --format markdown` 导出为 Markdown 文件集（一个 zip 或目录）
- `note export --format txt` 导出为纯文本
- `note export --format csv` 导出为 CSV（元数据 + 内容）
- 支持 `--output` / `-o` 指定输出路径（默认当前目录）
- 支持导出时应用筛选条件（`--category` / `--tag` / `--date` 等）
- 支持 `--id <ID1>,<ID2>` 导出指定笔记

### 6.2 导入

- `note import <文件路径>` 从外部文件导入笔记
- 支持导入格式：
  - JSON（NoteCli 自身导出的格式）
  - Markdown 文件（尝试从 front matter 解析元数据）
  - 纯文本文件（每条导入为一个笔记，文件名作为标题）
- 导入时自动去重（基于内容哈希或标题 + 创建时间）
- 导入时指定默认分类/标签：`--category` / `--tags`
- 导入完成后报告成功/跳过/失败的数量

---

## 7. 统计与信息

### 7.1 笔记统计

- `note stats` 显示笔记库统计信息：
  - 总笔记数
  - 各分类笔记数
  - 各优先级笔记数
  - 各标签使用频次（Top 10）
  - 近 7 天 / 30 天新增数
  - 最早和最新笔记的日期
  - 存储空间占用

### 7.2 日报/周报

- `note log` 显示今日创建/修改的笔记列表
- `note log --week` 显示本周笔记
- `note log --date 2024-01-15` 显示指定日期的笔记
- 输出格式为时间线形式，按时间排序

---

## 8. 快捷操作

### 8.1 快速笔记

- `note quick "内容"` / `note q "内容"` — 创建一条最小化笔记（无分类、无标签、normal 优先级）
- 等同于 `note add "内容"` 但跳过所有可选参数的提示

### 8.2 置顶/收藏

- `note pin <ID>` 置顶笔记（列表中始终排在最前）
- `note unpin <ID>` 取消置顶
- 置顶笔记在列表中用特殊标记（如 `*`）标识

### 8.3 完成/归档

- `note done <ID>` 标记笔记为已完成（适用于待办类笔记）
- `note archive <ID>` 归档笔记（从默认列表中隐藏）
- `note ls --archived` 查看已归档笔记
- `note unarchive <ID>` 取消归档

---

## 9. 交互式体验

### 9.1 交互式选择

- `note interactive` / `note i` 进入交互式模式
- 使用方向键浏览笔记列表
- 选中笔记后可查看详情、编辑、删除
- 支持 FZF 风格的实时过滤
- 注意：此功能需要引入终端 UI 库（如 ratatui），可作为可选 feature

### 9.2 Shell 补全

- 提供 `note completion <shell>` 命令生成 shell 补全脚本
- 支持的 shell：bash、zsh、fish、PowerShell、elvish
- 补全内容包括：子命令、参数名、已有分类名、已有标签名

### 9.3 别名支持

- 内置命令别名：`ls` → `list`、`rm` → `delete`、`s` → `show`、`e` → `edit`
- 用户可在配置文件中自定义别名

---

## 10. 命令总览

```
note                          显示帮助信息
note --version                显示版本号

# 笔记 CRUD
note add       [-t TITLE] [-c CAT] [-T TAGS] [-p PRI] [CONTENT]    创建笔记
note show      <ID> [--raw]                                         查看笔记
note edit      <ID> [--title T] [--category C] [--tags T] ...      编辑笔记
note delete    <ID...> [--force]                                    删除笔记

# 列表与搜索
note list      [-c CAT] [-T TAG] [-p PRI] [--sort S] [-n N] ...    列出笔记
note search    <QUERY> [--mode MODE] [--case-sensitive]             搜索笔记

# 分类管理
note category  list | tree | rename | delete                        分类操作

# 标签管理
note tag       list | rename | delete                               标签操作

# 导入导出
note export    [--format FMT] [--output PATH] [FILTERS...]          导出笔记
note import    <PATH> [--category C] [--tags T]                     导入笔记

# 其他
note stats                                                          统计信息
note log       [--week] [--date DATE]                               时间线日志
note index     rebuild                                              重建索引
note config    list | get <KEY> | set <KEY> <VALUE>                 配置管理
note completion <SHELL>                                             生成补全脚本
note pin       <ID>                                                 置顶
note unpin     <ID>                                                 取消置顶
note archive   <ID>                                                 归档
note unarchive <ID>                                                 取消归档
note done      <ID>                                                 标记完成
```

---

## 11. 错误处理

- 笔记 ID 不存在：`Error: 笔记 "abc123" 不存在`
- 分类/标签不存在：`Error: 分类 "xyz" 不存在`
- 空搜索结果：`未找到匹配的笔记。`
- 文件权限错误：`Error: 无法写入 ~/.notecli/notes/ — 权限不足`
- 磁盘空间不足：`Error: 磁盘空间不足，无法保存笔记`
- 索引损坏：`Warning: 索引文件已损坏，正在自动重建...`
- 编辑器启动失败：`Error: 无法启动编辑器 "vim"：程序未找到。请检查 $EDITOR 环境变量或 config.toml 中的 default_editor 设置。`
- 无效参数组合：给出清晰的 usage 提示

所有错误信息使用红色输出，警告使用黄色。错误退出码：0 成功，1 一般错误，2 参数错误。

---

## 12. 非功能性需求

### 12.1 性能

| 操作 | 目标响应时间 | 备注 |
|------|-------------|------|
| 创建笔记 | < 50ms | 含索引更新 |
| 显示笔记 | < 20ms | |
| 列表（1000 条） | < 100ms | |
| 全文搜索（1000 条） | < 200ms | |
| 索引重建（1000 条） | < 500ms | |

### 12.2 容量

- 支持至少 10,000 条笔记无性能明显下降
- 单条笔记正文大小上限：1MB
- 标题长度上限：256 字符

### 12.3 安全

- 不记录任何笔记内容到日志文件
- 不向外部服务发送任何数据
- 文件权限：数据目录和笔记文件设置为仅当前用户可读写（Unix: 600/700）

### 12.4 可测试性

- 所有核心逻辑与 I/O 分离，便于单元测试
- 提供 `--data-dir` 参数指定测试用数据目录
- 集成测试覆盖主要用户流程

---

## 13. 未来扩展（本期不实现，仅预留设计空间）

- **加密笔记**：对指定笔记进行 AES-256 加密存储
- **同步功能**：支持 WebDAV / S3 等协议的跨设备同步
- **笔记关联**：笔记之间的双向链接和反向链接
- **模板系统**：预设模板快速创建特定类型笔记
- **Web UI**：提供本地 Web 界面浏览笔记
- **插件系统**：支持自定义命令和扩展
- **全文索引**：集成 Tantivy 等全文搜索引擎以支持大规模笔记库
- **Git 集成**：自动将笔记目录作为 Git 仓库管理版本历史
- **多语言界面**：支持中文/英文界面切换
