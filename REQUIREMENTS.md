# NoteCli — Rust 备忘录 CLI 需求文档

## 1. 项目概述

NoteCli 是一个基于 Rust 开发的命令行备忘录管理工具。它以本地文件作为存储后端，提供快速的笔记创建、检索、编辑和整理能力。目标是成为开发者在终端中随手记录想法、待办、片段信息的首选工具——轻量、快速、零依赖外部服务。

### 1.1 设计原则

- **离线优先**：所有数据存储在本地，不依赖网络或远程服务
- **纯文本友好**：笔记内容以 Markdown 格式存储，可用任何编辑器直接查看和编辑
- **快速响应**：常用操作（新增、搜索、列表）应在 100ms 内完成
- **可组合性**：输出设计为可被其他 Unix 工具（grep、fzf、bat 等）管道处理
- **跨平台**：支持 Windows / macOS / Linux

### 1.2 技术栈

| 依赖 | 版本 | 用途 |
|------|------|------|
| clap | 4.6.1 (derive) | 命令行参数解析 |
| serde / serde_json | 1.0.228 | 序列化/反序列化 |
| chrono | 0.4.44 | 日期时间处理 |
| comfy-table | 7.1.4 | 表格渲染 |
| colored | 3.1.1 | 终端彩色输出 |
| regex | 1.11.1 | 正则搜索 |
| toml | 1.1.2 | 配置文件解析 |
| tempfile | 3.27.0 | 临时文件处理 |

---

## 2. 数据模型

### 2.1 核心模型

```
NoteIndexModel {
    id: u32,                    // 自增整数 ID
    title: String,
    category: CategoryModel,
    tags: Vec<TagModel>,
    priority: String,           // "low" | "normal" | "high" | "urgent"
    created: DateTime<Local>,
    modified: DateTime<Local>,
}

NoteModel {
    index: NoteIndexModel,
    content: String,            // Markdown 正文
}

CategoryModel {
    id: u32,
    name: String,
    parentid: u32,              // 0 表示顶层分类
}

TagModel {
    id: u32,
    name: String,
}
```

### 2.2 状态模型

```
NoteStatus {
    notes: Vec<NoteIndexModel>,
    pinned_notes_id: Vec<u32>,
    archived_notes: Vec<u32>,
}

CategoryStatus {
    categories: Vec<CategoryModel>,
}

TagStatus {
    tags: Vec<TagModel>,
}
```

---

## 3. 核心功能需求

### 3.1 笔记管理（CRUD）

#### 3.1.1 创建笔记

- `note add <内容>` 创建一条笔记（内容为必填参数）
- 创建时自动生成以下元数据：
  - 自增 ID（u32，基于当前最大 ID + 1）
  - 创建时间（精确到秒，本地时区）
  - 修改时间（初始等于创建时间）
- 创建时可选指定：
  - 标题（`--title` / `-t`），未指定时以内容前 20 字符作为标题
  - 分类（`--category` / `-c`），未指定时使用配置中的默认分类
  - 标签（`--tags` / `-T`，多个标签独立指定，上限 20 个）
  - 优先级（`--priority` / `-p`，可选值：low / normal / high / urgent）
- 创建时自动处理重名标题（在标题后追加 `_1`, `_2` 等后缀）
- 创建成功后输出新笔记的 ID

#### 3.1.2 查看笔记

- `note show <ID>` 显示单条笔记的完整内容
- 显示内容包括：ID、标题、分类、标签、优先级、创建时间、修改时间、正文
- 支持终端彩色输出，关键元数据（ID、优先级、标签）使用颜色高亮
- 支持 `--raw` 参数以纯文本输出（不包含 ANSI 颜色码），方便管道处理
- 当笔记 ID 不存在时，给出明确错误提示

#### 3.1.3 编辑笔记

- `note edit <ID> --title "新标题"` 直接修改标题
- `note edit <ID> --category "新分类"` 直接修改分类
- `note edit <ID> --tags "tag1" "tag2"` 直接替换标签
- `note edit <ID> --priority high` 直接修改优先级
- `note edit <ID> --content "新内容"` 替换笔记正文
- `note edit <ID> --append "追加内容"` 在笔记末尾追加文本
- 编辑后自动更新修改时间
- 已归档的笔记不允许编辑
- 当指定 ID 不存在时，给出明确错误提示

#### 3.1.4 删除笔记

- `note delete <ID>` 删除指定笔记
- `--force` / `-f` 跳过确认（当前实现直接删除，无交互确认）
- 已归档的笔记不允许直接删除
- 删除操作为永久删除，同时移除磁盘文件、索引记录、置顶和归档状态

### 3.2 笔记列表与浏览

#### 3.2.1 列表展示

- `note list` / `note ls` 列出所有笔记（不含已归档笔记）
- 默认显示格式为表格，每行包含：ID、标题、分类、优先级、标签、创建时间
- 置顶笔记在列表中有 `*` 标记，排在最前面
- 已完成（done）的笔记有 `[DONE]` 标记
- 支持以下排序方式（`--sort` / `-s` 参数）：
  - `created` — 按创建时间（默认）
  - `modified` — 按修改时间
  - `title` — 按标题字母序
  - `priority` — 按优先级
- 支持分页：`--limit` / `-n` 控制每页条数，`--offset` / `-o` 控制偏移
- 当没有任何笔记时，显示友好的空状态提示

#### 3.2.2 筛选过滤

- `--category <分类>` / `-c` 按分类筛选
- `--tag <标签>` / `-T` 按标签筛选（可多次指定）
- `--priority <优先级>` / `-p` 按优先级筛选
- `--date <日期表达式>` / `-d` 按创建日期筛选，支持以下格式：
  - `2024-01-15` — 精确日期
  - `2024-01` — 某月全部
  - `today` / `yesterday` — 相对日期
  - `last-7d` / `last-30d` — 近 N 天
- `--has-tag` 仅显示有标签的笔记
- `--no-tag` 仅显示无标签的笔记
- 多个筛选条件之间为 AND 关系

### 3.3 搜索功能

- `note search <关键词>` 全文搜索笔记内容
- 搜索范围覆盖：标题、正文、分类、标签
- 匹配结果中高亮显示关键词
- 支持以下搜索模式（`--mode` / `-m` 参数）：
  - `plain` — 纯文本匹配（默认）
  - `regex` — 正则表达式匹配
  - `fuzzy` — 模糊匹配
- 支持 `--case-sensitive` 大小写敏感搜索（默认不敏感）
- 搜索结果显示匹配笔记的表格列表

---

## 4. 分类与标签系统

### 4.1 分类（Category）

- 分类为扁平结构，每个分类有唯一 ID、名称和父级 ID（预留层级设计空间）
- `note category list` 列出所有分类及其笔记数量
- `note category tree` 以树形结构展示分类层级
- `note category rename <旧分类> <新分类>` 重命名分类（该分类下所有笔记同步更新，包括磁盘文件）
- `note category delete <分类>` 删除分类：
  - `--force` / `-f` 跳过确认
  - `--keep` / `-k` 保留笔记文件，将其分类重置为 "default"
  - 不指定 `--keep` 时同时删除该分类下的所有笔记
- 未指定分类的笔记归入默认分类 `default`

### 4.2 标签（Tag）

- 标签为扁平结构，无层级
- `note tag list` 列出所有标签及其使用次数
- `note tag rename <旧标签> <新标签>` 重命名标签（所有引用同步更新，包括磁盘文件）
- `note tag delete` 从所有笔记中移除该标签（实现为占位符，待完善）

---

## 5. 数据存储

### 5.1 存储结构

- 默认数据目录：`./.notecli/`（相对于项目工作目录）
- 目录结构：
  ```
  .notecli/
  ├── notes/               # 笔记数据目录
  │   ├── <title>.md       # 每条笔记以标题命名
  │   └── ...
  └── index.json           # 元数据索引文件
  ```
- 每条笔记的 Markdown 文件采用 JSON front matter 格式存储元数据：
  ```markdown
  ---
  {"id":1,"title":"会议纪要","category":{"id":1,"name":"work","parentid":0},"tags":[{"id":1,"name":"weekly"}],"priority":"normal","created":"2024-01-15T14:30:00+08:00","modified":"2024-01-15T16:45:00+08:00"}
  ---

  笔记正文内容...
  ```

### 5.2 文件命名

- 笔记文件以标题的净化形式命名（`<title>.md`）
- 净化规则：`/ \ : * ? " < > |` 替换为 `_`，去除首尾的点和空格
- 若净化后为空，使用 `untitled` 作为文件名
- 标题重复时自动追加 `_1`, `_2` 等后缀

### 5.3 索引机制

- 维护一个 JSON 格式的索引文件（`index.json`），缓存所有笔记的元数据、分类状态和标签状态
- 程序启动时自动执行 `sync_notes`：扫描磁盘文件与索引进行双向同步
  - 磁盘新增但索引缺失的笔记 → 从文件读取元数据并添加到索引
  - 索引存在但磁盘缺失的笔记 → 从索引中移除
  - 分类和标签状态从笔记元数据中重建
- 在笔记增删改时同步更新索引

### 5.4 数据操作

- 笔记更新采用"写新删旧"策略：先写入新文件成功后再删除旧文件
- 笔记 ID 与自增计数器绑定，不回收已删除的 ID
- 并发访问保护待实现

---

## 6. 配置系统

### 6.1 配置文件

- 默认配置文件路径：`./.notecli/config.toml`
- 程序启动时自动尝试加载配置文件，不存在时使用内置默认值
- 支持的配置项：

  ```toml
  [general]
  default_editor = "vim"          # 默认编辑器（预留，未使用）
  default_priority = "normal"     # 默认优先级
  default_category = "default"    # 默认分类
  language = "zh-CN"              # 界面语言（预留，未使用）
  pager = "less"                  # 分页器（预留，未使用）

  [display]
  color = true                    # 是否启用彩色输出
  date_format = "%Y-%m-%d %H:%M"  # 日期显示格式
  table_style = "compact"         # 表格样式
  title_max_width = 50            # 标题最大显示宽度

  [storage]
  notes_dir = "./.notecli/notes"      # 笔记存储目录
  index_file = "./.notecli/index.json" # 索引文件路径

  [search]
  default_mode = "plain"          # 默认搜索模式
  case_sensitive = false          # 默认大小写敏感
  max_results = 50                # 最大搜索结果数

  [theme]
  title = "cyan bold"             # 标题样式
  id = "yellow"                   # ID 样式
  tag = "green"                   # 标签样式
  category = "blue"               # 分类样式
  priority_low = "white"          # 低优先级样式
  priority_normal = "green"       # 普通优先级样式
  priority_high = "yellow bold"   # 高优先级样式
  priority_urgent = "red bold"    # 紧急优先级样式
  separator = "dark_gray"         # 分隔线样式
  date = "dark_gray"              # 日期样式
  ```

### 6.2 配置管理命令

- `note config list` — 以表格形式列出所有配置项及其当前值，底部显示配置文件路径
- `note config get <key>` — 获取指定配置项的值
  - key 格式为 `section.field`（如 `general.default_editor`）
  - 不存在的 key 会提示错误并列出正确格式
- `note config set <key> <value>` — 设置配置项的值并持久化到配置文件
  - 设置时自动进行值校验：
    - `general.default_priority` 限 low/normal/high/urgent
    - `display.color`、`search.case_sensitive` 限布尔值
    - `display.title_max_width`、`search.max_results` 限正整数
    - `display.table_style` 限 compact/expanded/markdown
    - `search.default_mode` 限 plain/regex/fuzzy
  - 配置文件不存在时自动创建（含目录）
  - 成功后显示新值和保存路径

### 6.3 配置优先级

命令行参数 > 配置文件 > 内置默认值

---

## 7. 主题系统

### 7.1 样式定义

- 主题系统支持以下样式属性：
  - 前景色：black / red / green / yellow / blue / magenta / cyan / white 及其 bright 变体
  - 加粗：bold
  - 下划线：underline
- 样式以空格分隔的字符串定义（如 `"cyan bold"`、`"red bold"`）
- 当 `display.color = false` 或使用 `--raw` 时，自动禁用颜色输出

### 7.2 输出系统

- `Output` 结构封装所有终端输出逻辑
- 提供消息级别：`success`、`error`（红色）、`warn`（黄色）、`hint`、`info`
- 表格输出基于 comfy-table，表头加粗
- 提供主题化的单元格方法：`cell_id`、`cell_title`、`cell_category`、`cell_priority`、`cell_date`、`cell_tag`

---

## 8. 导入与导出

### 8.1 导出

- `note export --format json` 导出为 JSON 格式
- `note export --format markdown` 导出为 Markdown 文件
- `note export --format txt` 导出为纯文本
- `note export --format csv` 导出为 CSV
- 支持 `--path` / `-p` 指定输出路径（默认为 `./export/` 目录）
- 支持导出时应用筛选条件：
  - `--category` / `-c` 按分类筛选
  - `--tag` / `-T` 按标签筛选
  - `--date` / `-d` 按日期筛选
  - `--id` / `-i` 导出指定笔记 ID
  - `--all` / `-a` 导出所有笔记

### 8.2 导入

- `note import <文件路径>` 从外部文件导入笔记
- 支持导入格式：纯文本文件
  - 文件中以 `---` 分隔多条笔记，每段作为一个独立笔记导入
  - 无分隔时整个文件作为一条笔记
- 导入时可指定默认分类和标签：`--category` / `-c`、`--tags` / `-T`

---

## 9. 统计与信息

### 9.1 笔记统计

- `note stats` 显示笔记库统计信息：
  - 总笔记数
  - 各优先级笔记数
  - 置顶笔记数
  - 归档笔记数
  - 分类数量
  - 标签数量
  - 各分类的笔记数
  - 各标签的使用次数

### 9.2 日志查看

- 时间线日志功能已实现处理逻辑，但尚未接入 CLI 命令
- 支持查看今日、本周或指定日期创建/修改的笔记

---

## 10. 快捷操作

### 10.1 置顶/收藏

- `note pin <ID>` 置顶笔记（列表中始终排在最前，重复置顶不重复添加）
- `note unpin <ID>` 取消置顶
- 置顶笔记在列表中用 `*` 标记标识

### 10.2 归档

- `note archive <ID>` 归档笔记（从默认列表中隐藏）
- `note unarchive <ID>` 取消归档
- 已归档的笔记不允许编辑和删除

### 10.3 完成

- `note done <ID>` 标记笔记为已完成（当前实现等同于归档）
- 完成的笔记在列表中显示 `[DONE]` 标记

---

## 11. 命令总览

```
note                                                    显示提示信息
note --version                                          显示版本号

# 笔记 CRUD
note add       <CONTENT> [-t TITLE] [-c CAT] [-T TAGS] [-p PRI]    创建笔记
note show      [ID] [--raw]                                         查看笔记
note edit      <ID> [--title T] [--category C] [--tags T] ...      编辑笔记
note delete    <ID> [--force]                                       删除笔记

# 列表与搜索
note list      [-c CAT] [-T TAG] [-p PRI] [--sort S] [-n N] ...    列出笔记
note search    <QUERY> [--mode MODE] [--case-sensitive]             搜索笔记

# 分类管理
note category  list | tree | rename | delete                        分类操作

# 标签管理
note tag       list | rename | delete                               标签操作

# 导入导出
note export    --format FMT [--path PATH] [FILTERS...]              导出笔记
note import    <PATH> [--category C] [--tags T]                     导入笔记

# 其他
note stats                                                          统计信息
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

## 12. 错误处理

- 笔记 ID 不存在：`错误: 笔记 <ID> 不存在`
- 分类不存在：`错误: 分类 "xyz" 不存在`
- 空搜索结果：`未找到匹配的笔记。`
- 编辑/删除已归档笔记：`错误: 无法操作已归档的笔记`
- 无效笔记 ID：`无效的笔记ID`（parse 错误 panic）
- 错误信息使用红色输出，警告使用黄色
- 错误退出码：0 成功，1 一般错误

---

## 13. 非功能性需求

### 13.1 性能

| 操作 | 目标响应时间 | 备注 |
|------|-------------|------|
| 创建笔记 | < 50ms | 含索引更新 |
| 显示笔记 | < 20ms | |
| 列表（1000 条） | < 100ms | |
| 全文搜索（1000 条） | < 200ms | |
| 索引重建（1000 条） | < 500ms | |

### 13.2 容量

- 支持至少 10,000 条笔记无性能明显下降
- 单条笔记正文大小上限：1MB
- 标题长度上限：256 字符

### 13.3 安全

- 不记录任何笔记内容到日志文件
- 不向外部服务发送任何数据

---

## 14. 已知限制与待完善项

| 项目 | 当前状态 | 说明 |
|------|---------|------|
| `note add` 不带内容打开编辑器 | 未实现 | 内容为必填参数，需后续支持 |
| 批量删除 | 未实现 | 当前仅支持单个 ID 删除 |
| 删除确认交互 | 未实现 | 仅提供 `--force` 参数，无交互确认流程 |
| `note tag delete` | **已实现** | 支持按名称删除标签，需 `-f` 确认 |
| `note index rebuild` | 占位符 | 仅打印命令名称，未实现重建逻辑 |
| `note config` 子命令 | **已实现** | 支持配置的 list/get/set，含值校验和持久化 |
| `note completion` | 占位符 | 仅打印 shell 类型，未生成补全脚本 |
| 无效 ID 错误处理 | 不完善 | ID 解析失败时 panic，应改为友好错误提示 |
| 文件锁 / 并发安全 | 未实现 | 多实例同时运行可能导致数据竞争 |
| Markdown 终端渲染 | 未实现 | `--raw` 已支持，但无 Markdown 渲染功能 |
| `note quick` 快捷命令 | 未实现 | 原需求中的快捷笔记功能 |
| 交互式模式 | 未实现 | 原需求中的 FZF 风格交互浏览 |

---

## 15. 未来扩展（本期不实现，仅预留设计空间）

- **加密笔记**：对指定笔记进行 AES-256 加密存储
- **同步功能**：支持 WebDAV / S3 等协议的跨设备同步
- **笔记关联**：笔记之间的双向链接和反向链接
- **模板系统**：预设模板快速创建特定类型笔记
- **Web UI**：提供本地 Web 界面浏览笔记
- **插件系统**：支持自定义命令和扩展
- **全文索引**：集成 Tantivy 等全文搜索引擎以支持大规模笔记库
- **Git 集成**：自动将笔记目录作为 Git 仓库管理版本历史
- **多语言界面**：支持中文/英文界面切换
- **交互式模式**：基于 ratatui 的终端 UI 交互浏览
