# NoteCli

中文 | [English](docs/README_EN.md)

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
title_max_width = 50            # 标题最大显示宽度

[storage]
notes_dir = "./.notecli/notes"
index_file = "./.notecli/index.json"

[search]
default_mode = "plain"          # 默认搜索模式
case_sensitive = false          # 默认大小写敏感
max_results = 50                # 最大搜索结果数

[theme]
title = "cyan bold"             # 标题样式
id = "yellow"                   # ID 样式
tag = "green"                   # 标签样式
category = "blue"               # 分类样式
date = "dark_gray"              # 日期样式
separator = "dark_gray"         # 分隔线样式
priority_low = "white"          # 低优先级样式
priority_normal = "green"       # 普通优先级样式
priority_high = "yellow bold"   # 高优先级样式
priority_urgent = "red bold"    # 紧急优先级样式
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
        └── completion.rs    # Shell 补全
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
