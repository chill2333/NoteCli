# NoteCli

[English](#english) | 中文

<p align="center">
  <strong>轻量级、交互式命令行笔记管理工具，使用 Rust 构建。</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-2024-orange.svg" alt="Rust Edition 2024" />
  <img src="https://img.shields.io/badge/platform-Windows-green.svg" alt="Platform: Windows" />
  <img src="https://img.shields.io/badge/license-Apache%202.0-blue.svg" alt="License: Apache 2.0" />
</p>

---

> **声明**：这是个人 Rust 练习项目，也是第一次尝试开源。代码中可能存在不成熟的设计和实现，部分重复工作由AI实现, 或许有一定纰漏，非常欢迎各位提出建议和指导，感谢！

NoteCli 是一款面向开发者的命令行笔记管理工具。所有数据以 Markdown 文件 + JSON 元数据存储在本地——无需数据库，无需网络，无厂商锁定。每条命令同时支持参数直接调用和交互式操作，既可以脚本化使用，也可以手动交互。

## 特性

- **完整 CRUD** — 创建、查看、编辑、删除笔记，支持丰富元数据
- **交互式模式** — 所有命令在缺少参数时自动启动交互式引导（基于 `dialoguer`）
- **分类与标签** — 灵活的分类/标签系统组织笔记
- **优先级** — low / normal / high / urgent，优先级优先排序
- **全文搜索** — 支持纯文本、正则、模糊三种搜索模式，结果高亮
- **置顶、归档、完成** — 快速状态操作
- **导入导出** — 支持 JSON、Markdown、纯文本、CSV 格式
- **可配置** — 基于 TOML 的配置系统，支持主题、显示、存储等选项
- **离线优先** — 所有数据保存在本地 `.md` 文件中
- **Windows 支持** — 目前支持 Windows 平台

## 安装

### 从源码构建

```bash
git clone https://github.com/chilling2333/NoteCli.git
cd NoteCli
cargo build --release
```

编译产物位于 `target/release/note.exe`，将其加入 `PATH` 即可全局使用。

### 前置要求

- [Rust](https://www.rust-lang.org/tools/install) 1.85+ (Edition 2024)

## 快速上手

```bash
# 创建笔记
note add "会议纪要：讨论 Q3 路线图"

# 创建带元数据的笔记
note add "修复登录 Bug" -t "Bug: 登录" -c work -T bug urgent -p high

# 交互式（省略必要参数即可触发）
note add              # 提示输入内容
note edit             # 显示笔记列表供选择
note delete           # 选择删除方式

# 列出所有笔记
note list

# 搜索
note search "路线图"
note search "error.*timeout" -m regex

# 查看笔记
note show 1

# 置顶、归档、完成
note pin 1
note archive 2
note done 3
```

## 命令详解

### 创建笔记

```bash
note add "笔记内容"                           # 快速创建
note add "内容" -t "标题" -c 分类 -T 标签1 标签2 -p high  # 指定所有选项
note add                                      # 交互式多行输入
```

### 查看笔记

```bash
note show 1           # 按 ID 查看
note show             # 交互式选择笔记
note show 1 --raw     # 纯文本输出（无 ANSI 颜色）
```

### 编辑笔记

```bash
note edit 1 -t "新标题"                 # 修改标题
note edit 1 -c work -T rust cli         # 修改分类和标签
note edit 1 -p urgent                   # 修改优先级
note edit 1 -C "替换内容"               # 替换正文
note edit 1 -a "追加内容"               # 追加正文
note edit                               # 交互式：选择笔记 → 选择字段
```

### 删除笔记

```bash
note delete 1                    # 按 ID 删除
note delete -T bug               # 删除所有含标签 "bug" 的笔记
note delete -c archive           # 删除分类 "archive" 下的所有笔记
note delete                      # 交互式选择删除方式
```

### 列出笔记

```bash
note list                        # 所有笔记（优先级优先排序）
note list -c work                # 按分类筛选
note list -T rust -T cli         # 按标签筛选
note list -p high                # 按优先级筛选
note list -s modified -n 20      # 按修改时间排序，显示 20 条
note list -d today               # 今天创建的笔记
note list --has-tag              # 只显示有标签的笔记
```

### 搜索

```bash
note search "关键词"             # 纯文本搜索
note search "pattern" -m regex   # 正则搜索
note search "模糊匹配" -m fuzzy  # 模糊搜索
note search "Error" --case-sensitive  # 大小写敏感
```

### 分类与标签

```bash
note category list               # 列出所有分类
note category tree               # 树形展示
note category rename 旧名称 新名称  # 重命名

note tag list                    # 列出所有标签
note tag rename 旧名称 新名称       # 重命名
```

### 置顶 / 归档 / 完成

```bash
note pin 1                       # 置顶
note unpin 1                     # 取消置顶
note archive 1                   # 归档（默认列表中隐藏）
note unarchive 1                 # 取消归档
note done 1                      # 标记为已完成
```

### 导入导出

```bash
# 导出
note export -f json -a           # 导出全部为 JSON
note export -f markdown -c work  # 导出分类 "work" 为 Markdown
note export -f csv -T rust       # 导出标签 "rust" 为 CSV
note export                      # 交互式：选择格式和笔记

# 导入
note import notes.json           # 从 JSON 导入
note import notes.md             # 从 Markdown 导入（按 ## 标题分割）
note import notes.txt            # 从纯文本导入（按 --- 分割）
note import                      # 交互式：选择文件
```

### 统计与配置

```bash
note stats                       # 笔记库统计信息

note config list                 # 查看所有配置
note config get display.color    # 获取配置值
note config set general.default_priority high   # 设置配置值
```

## 存储结构

所有数据存储在工作目录下的 `.notecli/` 中：

```
.notecli/
├── config.toml          # 配置文件
├── index.json           # 元数据索引缓存
└── notes/
    ├── 会议纪要.md
    ├── Bug修复.md
    └── ...
```

每条笔记为 Markdown 文件，头部使用 JSON front matter 存储元数据：

```markdown
---
{"id":1,"title":"会议纪要","category":{"id":1,"name":"work","parentid":0},"tags":[{"id":1,"name":"weekly"}],"priority":"normal","created":"2024-01-15T14:30:00+08:00","modified":"2024-01-15T16:45:00+08:00"}
---

会议讨论要点和行动项...
```

笔记文件始终可用任何文本编辑器直接查看和编辑，也可以用 Git 进行版本控制。

## 配置

默认配置位于 `.notecli/config.toml`：

```toml
[general]
default_priority = "normal"     # 新建笔记的默认优先级
default_category = "default"    # 新建笔记的默认分类

[display]
color = true                    # 启用彩色输出
date_format = "%Y-%m-%d %H:%M"  # 日期显示格式
table_style = "compact"         # 表格样式
title_max_width = 50            # 标题最大显示宽度

[storage]
notes_dir = "./.notecli/notes"
index_file = "./.notecli/index.json"

[search]
default_mode = "plain"          # 默认搜索模式
case_sensitive = false          # 默认大小写敏感
max_results = 50                # 最大搜索结果数

[theme]
title = "cyan bold"
id = "yellow"
tag = "green"
category = "blue"
priority_high = "yellow bold"
priority_urgent = "red bold"
```

## 项目结构

```
src/
├── main.rs                  # 入口
├── arg.rs                   # CLI 参数解析（clap derive）
└── notes/
    ├── mod.rs               # 模块根
    ├── model.rs             # 数据模型（Note, Category, Tag）
    ├── storage.rs           # 文件 I/O 与索引管理
    ├── output.rs            # 终端输出与主题
    ├── theme.rs             # 颜色/样式定义
    ├── config.rs            # 配置系统
    ├── input.rs             # 交互式输入工具
    └── handle/
        ├── add.rs           # 创建笔记
        ├── show.rs          # 查看笔记
        ├── edit.rs          # 编辑笔记
        ├── delete.rs        # 删除笔记
        ├── list.rs          # 列表与筛选
        ├── search.rs        # 全文搜索
        ├── category.rs      # 分类管理
        ├── tag.rs           # 标签管理
        ├── export.rs        # 导出
        ├── import.rs        # 导入
        ├── stats.rs         # 统计信息
        ├── pin.rs           # 置顶/取消置顶
        ├── archive.rs       # 归档/取消归档
        ├── done.rs          # 标记完成
        ├── config.rs        # 配置管理
```

## 技术栈

| 依赖 | 用途 |
|------|------|
| [clap](https://docs.rs/clap) 4.6 | 命令行参数解析（derive） |
| [dialoguer](https://docs.rs/dialoguer) 0.12 | 交互式提示 |
| [comfy-table](https://docs.rs/comfy-table) 7.1 | 表格渲染 |
| [colored](https://docs.rs/colored) 3.1 | 终端彩色输出 |
| [serde](https://docs.rs/serde) + [serde_json](https://docs.rs/serde_json) | 序列化/反序列化 |
| [chrono](https://docs.rs/chrono) 0.4 | 日期时间处理 |
| [regex](https://docs.rs/regex) 1.11 | 正则搜索 |
| [toml](https://docs.rs/toml) 1.1 | 配置文件解析 |
| [tempfile](https://docs.rs/tempfile) 3.27 | 安全文件操作 |

## 许可证

本项目基于 [Apache License 2.0](LICENSE) 开源。

## 贡献

欢迎贡献！请随时提交 Issue 报告 Bug 或提出功能建议，也欢迎提交 Pull Request。

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/my-feature`)
3. 提交修改 (`git commit -m 'Add my feature'`)
4. 推送到分支 (`git push origin feature/my-feature`)
5. 发起 Pull Request

---

<a id="english"></a>

# NoteCli

English | [中文](#notecli)

<p align="center">
  <strong>A lightweight, interactive CLI note manager built with Rust.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-2024-orange.svg" alt="Rust Edition 2024" />
  <img src="https://img.shields.io/badge/platform-Windows-green.svg" alt="Platform: Windows" />
  <img src="https://img.shields.io/badge/license-Apache%202.0-blue.svg" alt="License: Apache 2.0" />
</p>

---

> **Disclaimer**: This is a personal Rust practice project and my first attempt at open source. The code may contain immature designs and implementations — some repetitive work was done with AI assistance, so there might be oversights. Suggestions and guidance are very welcome, thank you!

NoteCli is a command-line note manager designed for developers. All data is stored locally as Markdown files with JSON metadata — no database, no network, no vendor lock-in. Every command supports both direct parameter invocation and interactive operation, making it suitable for both scripting and manual use.

## Features

- **Full CRUD** — Create, view, edit, and delete notes with rich metadata
- **Interactive Mode** — All commands automatically launch interactive prompts when arguments are missing (powered by `dialoguer`)
- **Categories & Tags** — Flexible category/tag system for organizing notes
- **Priority Levels** — low / normal / high / urgent, with priority-based sorting
- **Full-text Search** — Supports plain text, regex, and fuzzy search modes with highlighted results
- **Pin, Archive, Done** — Quick status operations
- **Import & Export** — Supports JSON, Markdown, plain text, and CSV formats
- **Configurable** — TOML-based config system for themes, display, storage, and more
- **Offline-first** — All data saved as local `.md` files
- **Windows Support** — Currently supports the Windows platform

## Installation

### Build from Source

```bash
git clone https://github.com/chilling2333/NoteCli.git
cd NoteCli
cargo build --release
```

The compiled binary is at `target/release/note.exe`. Add it to your `PATH` for global access.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.85+ (Edition 2024)

## Quick Start

```bash
# Create a note
note add "Meeting notes: Q3 roadmap discussion"

# Create a note with metadata
note add "Fix login bug" -t "Bug: Login" -c work -T bug urgent -p high

# Interactive mode (omit required arguments to trigger)
note add              # Prompts for content
note edit             # Shows note list for selection
note delete           # Choose deletion method

# List all notes
note list

# Search
note search "roadmap"
note search "error.*timeout" -m regex

# View a note
note show 1

# Pin, archive, mark done
note pin 1
note archive 2
note done 3
```

## Command Reference

### Create Notes

```bash
note add "Note content"                            # Quick create
note add "Content" -t "Title" -c category -T tag1 tag2 -p high  # Full options
note add                                           # Interactive multi-line input
```

### View Notes

```bash
note show 1           # View by ID
note show             # Interactive note selection
note show 1 --raw     # Plain text output (no ANSI colors)
```

### Edit Notes

```bash
note edit 1 -t "New title"               # Change title
note edit 1 -c work -T rust cli           # Change category and tags
note edit 1 -p urgent                     # Change priority
note edit 1 -C "Replacement content"      # Replace body
note edit 1 -a "Appended content"         # Append to body
note edit                                 # Interactive: select note → select field
```

### Delete Notes

```bash
note delete 1                    # Delete by ID
note delete -T bug               # Delete all notes with tag "bug"
note delete -c archive           # Delete all notes in category "archive"
note delete                      # Interactive deletion
```

### List Notes

```bash
note list                        # All notes (sorted by priority)
note list -c work                # Filter by category
note list -T rust -T cli         # Filter by tags
note list -p high                # Filter by priority
note list -s modified -n 20      # Sort by modified time, show 20
note list -d today               # Notes created today
note list --has-tag              # Only show notes with tags
```

### Search

```bash
note search "keyword"             # Plain text search
note search "pattern" -m regex    # Regex search
note search "fuzzy match" -m fuzzy # Fuzzy search
note search "Error" --case-sensitive  # Case-sensitive
```

### Categories & Tags

```bash
note category list                # List all categories
note category tree                # Tree view
note category rename old_name new_name  # Rename

note tag list                     # List all tags
note tag rename old_name new_name    # Rename
```

### Pin / Archive / Done

```bash
note pin 1                        # Pin to top
note unpin 1                      # Unpin
note archive 1                    # Archive (hidden from default list)
note unarchive 1                  # Unarchive
note done 1                       # Mark as done
```

### Import & Export

```bash
# Export
note export -f json -a            # Export all as JSON
note export -f markdown -c work   # Export category "work" as Markdown
note export -f csv -T rust        # Export tag "rust" as CSV
note export                       # Interactive: select format and notes

# Import
note import notes.json            # Import from JSON
note import notes.md              # Import from Markdown (split by ## headings)
note import notes.txt             # Import from plain text (split by ---)
note import                       # Interactive: select file
```

### Stats & Config

```bash
note stats                        # Note database statistics

note config list                  # View all config
note config get display.color     # Get a config value
note config set general.default_priority high   # Set a config value
```

## Storage Structure

All data is stored under `.notecli/` in the working directory:

```
.notecli/
├── config.toml          # Configuration file
├── index.json           # Metadata index cache
└── notes/
    ├── meeting-notes.md
    ├── bug-fix.md
    └── ...
```

Each note is a Markdown file with JSON front matter for metadata:

```markdown
---
{"id":1,"title":"Meeting Notes","category":{"id":1,"name":"work","parentid":0},"tags":[{"id":1,"name":"weekly"}],"priority":"normal","created":"2024-01-15T14:30:00+08:00","modified":"2024-01-15T16:45:00+08:00"}
---

Meeting discussion points and action items...
```

Note files can always be viewed and edited directly with any text editor, and version-controlled with Git.

## Configuration

Default configuration is at `.notecli/config.toml`:

```toml
[general]
default_priority = "normal"     # Default priority for new notes
default_category = "default"    # Default category for new notes

[display]
color = true                    # Enable colored output
date_format = "%Y-%m-%d %H:%M"  # Date display format
table_style = "compact"         # Table style
title_max_width = 50            # Max display width for titles

[storage]
notes_dir = "./.notecli/notes"
index_file = "./.notecli/index.json"

[search]
default_mode = "plain"          # Default search mode
case_sensitive = false          # Default case sensitivity
max_results = 50                # Max search results

[theme]
title = "cyan bold"
id = "yellow"
tag = "green"
category = "blue"
priority_high = "yellow bold"
priority_urgent = "red bold"
```

## Project Structure

```
src/
├── main.rs                  # Entry point
├── arg.rs                   # CLI argument parsing (clap derive)
└── notes/
    ├── mod.rs               # Module root
    ├── model.rs             # Data models (Note, Category, Tag)
    ├── storage.rs           # File I/O and index management
    ├── output.rs            # Terminal output and theming
    ├── theme.rs             # Color/style definitions
    ├── config.rs            # Configuration system
    ├── input.rs             # Interactive input utilities
    └── handle/
        ├── add.rs           # Create notes
        ├── show.rs          # View notes
        ├── edit.rs          # Edit notes
        ├── delete.rs        # Delete notes
        ├── list.rs          # List and filter
        ├── search.rs        # Full-text search
        ├── category.rs      # Category management
        ├── tag.rs           # Tag management
        ├── export.rs        # Export
        ├── import.rs        # Import
        ├── stats.rs         # Statistics
        ├── pin.rs           # Pin/unpin
        ├── archive.rs       # Archive/unarchive
        ├── done.rs          # Mark done
        ├── config.rs        # Config management
```

## Tech Stack

| Dependency | Purpose |
|------------|---------|
| [clap](https://docs.rs/clap) 4.6 | CLI argument parsing (derive) |
| [dialoguer](https://docs.rs/dialoguer) 0.12 | Interactive prompts |
| [comfy-table](https://docs.rs/comfy-table) 7.1 | Table rendering |
| [colored](https://docs.rs/colored) 3.1 | Colored terminal output |
| [serde](https://docs.rs/serde) + [serde_json](https://docs.rs/serde_json) | Serialization/deserialization |
| [chrono](https://docs.rs/chrono) 0.4 | Date and time handling |
| [regex](https://docs.rs/regex) 1.11 | Regex search |
| [toml](https://docs.rs/toml) 1.1 | Config file parsing |
| [tempfile](https://docs.rs/tempfile) 3.27 | Safe file operations |

## License

This project is licensed under the [Apache License 2.0](LICENSE).

## Contributing

Contributions are welcome! Feel free to submit Issues for bug reports or feature suggestions, and Pull Requests are also welcome.

1. Fork this repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -m 'Add my feature'`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Open a Pull Request
