use std::path::PathBuf;

use clap::{Parser,Subcommand,ValueEnum};

#[derive(Parser)]
#[command(version, about = "NoteCli - A lightweight CLI note manager")]
struct Args {
    #[command(subcommand)]
    commands:Option<NoteCommand>
}

#[derive(Subcommand)]
enum NoteCommand{
    /// Create a new note
    Add{
        /// Note title
        #[arg(short = 't',long = "title")]
        title:Option<String>,
        /// Note category
        #[arg(short = 'c',long = "category")]
        category:Option<String>,
        /// Tags for the note (1-20)
        #[arg(short = 'T',long = "tags", num_args = 1..=20)]
        tags:Option<Vec<String>>,
        /// Priority level [low, normal, high, urgent]
        #[arg(short = 'p',long = "priority")]
        priority:Option<Priority>
    },
    /// Show a note by ID
    Show{
        /// Note ID to display
        id:Option<String>,
        /// Output without ANSI colors
        #[arg(short = 'r',long = "raw")]
        raw:bool,
    },
    /// Edit an existing note
    Edit{
        /// Note ID to edit
        id:u32,
        /// New title
        #[arg(short = 't',long = "title")]
        title:Option<String>,
        /// New category
        #[arg(short = 'c',long = "category")]
        category:Option<String>,
        /// New tags (replaces existing)
        #[arg(short = 'T',long = "tags", num_args = 1..=20)]
        tags:Option<Vec<String>>,
        /// New priority level
        #[arg(short = 'p',long = "priority")]
        priority:Option<Priority>,
        /// Append text to note content
        #[arg(short = 'a',long = "append")]
        append:Option<String>,
    },
    /// Delete a note by ID
    Delete{
        /// Note ID to delete
        id:u32,
        /// New title
        #[arg(short = 't',long = "title")]
        title:Option<String>,
        /// New category
        #[arg(short = 'c',long = "category")]
        category:Option<String>,
        /// New tags (replaces existing)
        #[arg(short = 'T',long = "tags", num_args = 1..=20)]
        tags:Option<Vec<String>>,
        /// New priority level
        #[arg(short = 'p',long = "priority")]
        priority:Option<Priority>,
        /// Skip confirmation prompt
        #[arg(short = 'f',long = "force")]
        force:bool,
    },
    /// List all notes with optional filters
    List{
        /// Sort field [created, modified, title, priority]
        #[arg(short = 's',long = "sort")]
        sort:Option<SortType>,
        /// Max number of notes to show
        #[arg(short = 'n',long = "limit")]
        limit:Option<u32>,
        /// Number of notes to skip
        #[arg(short = 'o',long = "offset")]
        offset:Option<u32>,
        /// Filter by category
        #[arg(short = 'c',long = "category")]
        category:Option<String>,
        /// Filter by tag (can specify multiple)
        #[arg(short = 'T',long = "tag", num_args = 1..=20)]
        tag:Option<Vec<String>>,
        /// Filter by priority
        #[arg(short = 'p',long = "priority")]
        priority:Option<String>,
        /// Filter by date expression (e.g. today, last-7d, 2024-01-15)
        #[arg(short = 'd',long = "date")]
        date:Option<String>,
        /// Only show notes that have tags
        #[arg(long = "has-tag")]
        hastag:bool,
        /// Only show notes without tags
        #[arg(long = "no-tag")]
        notag:bool,
    },
    /// Search notes by keyword
    Search{
        /// Search mode [plain, regex, fuzzy]
        #[arg(short = 'm',long = "mode")]
        mode:Option<SearchMode>,
        /// Enable case-sensitive search
        #[arg(long = "case-sensitive")]
        casesensitive:bool
    },
    /// Manage categories
    #[command(subcommand)]
    Category(CategoryCommand),
    /// Manage tags
    #[command(subcommand)]
    Tag(TagCommand),
    
    /// Export notes to file
    Export{
        /// Export format [json, markdown, txt, csv]
        #[arg(short = 'f',long = "format")]
        format: ExportFormat,
        /// Output file or directory path (default: current directory)
        #[arg(short = 'p',long = "path")]
        path: Option<PathBuf>,
        /// Export specific note IDs
        #[arg(short = 'i',long = "id", num_args = 1..=20)]
        id: Option<Vec<String>>,
        /// Export all notes
        #[arg(short = 'a',long = "all")]
        all: bool,
        /// Filter by category
        #[arg(short = 'c',long = "category")]
        category: Option<String>,
        /// Filter by tag
        #[arg(short = 'T',long = "tag", num_args = 1..=20)]
        tag: Option<Vec<String>>,
        /// Filter by date expression
        #[arg(short = 'd',long = "date")]
        date: Option<String>,
    },

    /// Import notes from file
    Import{
        /// Input file path (supports .json, .md, .txt)
        path: PathBuf,
        /// Default category for imported notes
        #[arg(short = 'c',long = "category")]
        category: Option<String>,
        /// Default tags for imported notes
        #[arg(short = 'T',long = "tags", num_args = 1..=20)]
        tags: Option<Vec<String>>,
    },

    /// Show note statistics
    Stats,

    /// Show notes from a specific date or time range
    Log{
        /// Show notes from this week
        #[arg(long = "week")]
        week: bool,
        /// Show notes from specific date (e.g. 2024-01-15)
        #[arg(long = "date")]
        date: Option<String>,
    },

    /// Manage note index
    #[command(subcommand)]
    Index(IndexCommand),

    /// Manage configuration
    #[command(subcommand)]
    Config(ConfigCommand),

    /// Generate shell completion script
    Completion{
        /// Target shell [bash, zsh, fish, powershell, elvish]
        shell: ShellType,
    },

    /// Pin a note to top of list
    Pin{
        /// Note ID to pin
        id: String,
    },

    /// Unpin a note
    Unpin{
        /// Note ID to unpin
        id: String,
    },

    /// Archive a note (hide from default list)
    Archive{
        /// Note ID to archive
        id: String,
    },

    /// Unarchive a note
    Unarchive{
        /// Note ID to unarchive
        id: String,
    },

    /// Mark a note as done
    Done{
        /// Note ID to mark as done
        id: String,
    },
}

#[derive(Subcommand)]
enum CategoryCommand{
    /// List all categories
    List,
    /// Show category hierarchy as tree
    Tree,
    /// Rename a category
    Rename{
        /// Current category name
        old_name:Option<String>,
        /// New category name
        new_name:Option<String>
    },
    /// Delete a category
    Delete{
        /// Skip confirmation prompt
        #[arg(short = 'f',long = "force")]
        force:bool,
    }
}
#[derive(Subcommand)]
enum TagCommand{
    /// List all tags
    List,
    /// Rename a tag
    Rename{
        /// Current tag name
        old_name:Option<String>,
        /// New tag name
        new_name:Option<String>
    },
    /// Delete a tag
    Delete{
        /// Skip confirmation prompt
        #[arg(short = 'f',long = "force")]
        force:bool,
    }
}


#[derive(Debug, Clone, Copy,ValueEnum)]
enum Priority{
    LOW,
    NORMAL,
    HIGH,
    URGENT,
}
#[derive(Debug, Clone, Copy,ValueEnum)]
enum SortType{
    Created,
    Modified,
    Title,
    Priority,
}


#[derive(Debug, Clone, Copy,ValueEnum)]
enum SearchMode{
    Plain,
    Regex,
    Fuzzy,
}

#[derive(Debug, Clone, Copy,ValueEnum)]
enum ExportFormat{
    Json,
    Markdown,
    Txt,
    Csv,
}

#[derive(Subcommand)]
enum IndexCommand{
    /// Rebuild the note index
    Rebuild,
}

#[derive(Subcommand)]
enum ConfigCommand{
    /// List all configuration values
    List,
    /// Get a specific config value by key
    Get{
        /// Config key name
        key: String,
    },
    /// Set a config value
    Set{
        /// Config key name
        key: String,
        /// Config value
        value: String,
    },
}

#[derive(Debug, Clone, Copy,ValueEnum)]
enum ShellType{
    Bash,
    Zsh,
    Fish,
    Powershell,
    Elvish,
}


pub fn arg_setup(){
    let cli: Args = Args::parse();
    match cli.commands {
        None => {
            println!("未指定子命令，使用 --help 查看帮助");
        }
        Some(NoteCommand::Add { title, category, tags, priority }) => {
            println!("命令: Add");
            println!("  title: {:?}", title);
            println!("  category: {:?}", category);
            println!("  tags: {:?}", tags);
            println!("  priority: {:?}", priority);
        }
        Some(NoteCommand::Show { id, raw }) => {
            println!("命令: Show");
            println!("  id: {:?}", id);
            println!("  raw: {}", raw);
        }
        Some(NoteCommand::Edit { id, title, category, tags, priority, append }) => {
            println!("命令: Edit");
            println!("  id: {}", id);
            println!("  title: {:?}", title);
            println!("  category: {:?}", category);
            println!("  tags: {:?}", tags);
            println!("  priority: {:?}", priority);
            println!("  append: {:?}", append);
        }
        Some(NoteCommand::Delete { id, title, category, tags, priority, force }) => {
            println!("命令: Delete");
            println!("  id: {}", id);
            println!("  title: {:?}", title);
            println!("  category: {:?}", category);
            println!("  tags: {:?}", tags);
            println!("  priority: {:?}", priority);
            println!("  force: {}", force);
        }
        Some(NoteCommand::List { sort, limit, offset, category, tag, priority, date, hastag, notag }) => {
            println!("命令: List");
            println!("  sort: {:?}", sort);
            println!("  limit: {:?}", limit);
            println!("  offset: {:?}", offset);
            println!("  category: {:?}", category);
            println!("  tag: {:?}", tag);
            println!("  priority: {:?}", priority);
            println!("  date: {:?}", date);
            println!("  hastag: {}", hastag);
            println!("  notag: {}", notag);
        }
        Some(NoteCommand::Search { mode, casesensitive }) => {
            println!("命令: Search");
            println!("  mode: {:?}", mode);
            println!("  casesensitive: {}", casesensitive);
        }
        Some(NoteCommand::Category(cmd)) => {
            match cmd {
                CategoryCommand::List => {
                    println!("命令: Category List");
                }
                CategoryCommand::Tree => {
                    println!("命令: Category Tree");
                }
                CategoryCommand::Rename { old_name, new_name } => {
                    println!("命令: Category Rename");
                    println!("  old_name: {:?}", old_name);
                    println!("  new_name: {:?}", new_name);
                }
                CategoryCommand::Delete { force } => {
                    println!("命令: Category Delete");
                    println!("  force: {}", force);
                }
            }
        }
        Some(NoteCommand::Tag(cmd)) => {
            match cmd {
                TagCommand::List => {
                    println!("命令: Tag List");
                }
                TagCommand::Rename { old_name, new_name } => {
                    println!("命令: Tag Rename");
                    println!("  old_name: {:?}", old_name);
                    println!("  new_name: {:?}", new_name);
                }
                TagCommand::Delete { force } => {
                    println!("命令: Tag Delete");
                    println!("  force: {}", force);
                }
            }
        }
        Some(NoteCommand::Export { format, path, id, all, category, tag, date }) => {
            println!("命令: Export");
            println!("  format: {:?}", format);
            println!("  path: {:?}", path);
            println!("  id: {:?}", id);
            println!("  all: {}", all);
            println!("  category: {:?}", category);
            println!("  tag: {:?}", tag);
            println!("  date: {:?}", date);
        }
        Some(NoteCommand::Import { path, category, tags }) => {
            println!("命令: Import");
            println!("  path: {:?}", path);
            println!("  category: {:?}", category);
            println!("  tags: {:?}", tags);
        }
        Some(NoteCommand::Stats) => {
            println!("命令: Stats");
        }
        Some(NoteCommand::Log { week, date }) => {
            println!("命令: Log");
            println!("  week: {}", week);
            println!("  date: {:?}", date);
        }
        Some(NoteCommand::Index(cmd)) => {
            match cmd {
                IndexCommand::Rebuild => {
                    println!("命令: Index Rebuild");
                }
            }
        }
        Some(NoteCommand::Config(cmd)) => {
            match cmd {
                ConfigCommand::List => {
                    println!("命令: Config List");
                }
                ConfigCommand::Get { key } => {
                    println!("命令: Config Get");
                    println!("  key: {}", key);
                }
                ConfigCommand::Set { key, value } => {
                    println!("命令: Config Set");
                    println!("  key: {}", key);
                    println!("  value: {}", value);
                }
            }
        }
        Some(NoteCommand::Completion { shell }) => {
            println!("命令: Completion");
            println!("  shell: {:?}", shell);
        }
        Some(NoteCommand::Pin { id }) => {
            println!("命令: Pin");
            println!("  id: {}", id);
        }
        Some(NoteCommand::Unpin { id }) => {
            println!("命令: Unpin");
            println!("  id: {}", id);
        }
        Some(NoteCommand::Archive { id }) => {
            println!("命令: Archive");
            println!("  id: {}", id);
        }
        Some(NoteCommand::Unarchive { id }) => {
            println!("命令: Unarchive");
            println!("  id: {}", id);
        }
        Some(NoteCommand::Done { id }) => {
            println!("命令: Done");
            println!("  id: {}", id);
        }
    }
}






#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_no_command() {
        let cli = Args::try_parse_from(["note"]);
        assert!(cli.is_ok());
        assert!(cli.unwrap().commands.is_none());
    }

    // ---- Add ----
    #[test]
    fn test_add_no_options() {
        let cli = Args::try_parse_from(["note", "add"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Add { title, category, tags, priority }) = cli.unwrap().commands {
            assert!(title.is_none());
            assert!(category.is_none());
            assert!(tags.is_none());
            assert!(priority.is_none());
        } else {
            panic!("期望 Add 命令");
        }
    }

    #[test]
    fn test_add_with_options() {
        let cli = Args::try_parse_from([
            "note", "add",
            "-t", "测试标题",
            "-c", "work",
            "-T", "tag1", "-T", "tag2",
            "-p", "high",
        ]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Add { title, category, tags, priority }) = cli.unwrap().commands {
            assert_eq!(title.unwrap(), "测试标题");
            assert_eq!(category.unwrap(), "work");
            assert_eq!(tags.unwrap(), vec!["tag1", "tag2"]);
            assert!(matches!(priority, Some(Priority::HIGH)));
        } else {
            panic!("期望 Add 命令");
        }
    }

    #[test]
    fn test_add_invalid_priority() {
        let result = Args::try_parse_from(["note", "add", "-p", "invalid"]);
        assert!(result.is_err());
    }

    // ---- Show ----
    #[test]
    fn test_show_with_id() {
        let cli = Args::try_parse_from(["note", "show", "abc123"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Show { id, raw }) = cli.unwrap().commands {
            assert_eq!(id.unwrap(), "abc123");
            assert!(!raw);
        } else {
            panic!("期望 Show 命令");
        }
    }

    #[test]
    fn test_show_raw() {
        let cli = Args::try_parse_from(["note", "show", "abc123", "--raw"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Show { id, raw }) = cli.unwrap().commands {
            assert_eq!(id.unwrap(), "abc123");
            assert!(raw);
        } else {
            panic!("期望 Show 命令");
        }
    }

    #[test]
    fn test_show_no_id() {
        let cli = Args::try_parse_from(["note", "show"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Show { id, .. }) = cli.unwrap().commands {
            assert!(id.is_none());
        } else {
            panic!("期望 Show 命令");
        }
    }

    // ---- Edit ----
    #[test]
    fn test_edit_partial() {
        let cli = Args::try_parse_from(["note", "edit", "42", "-t", "新标题", "-a", "追加内容"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Edit { id, title, append, category, tags, priority }) = cli.unwrap().commands {
            assert_eq!(id, 42);
            assert_eq!(title.unwrap(), "新标题");
            assert_eq!(append.unwrap(), "追加内容");
            assert!(category.is_none());
            assert!(tags.is_none());
            assert!(priority.is_none());
        } else {
            panic!("期望 Edit 命令");
        }
    }

    #[test]
    fn test_edit_missing_id() {
        let result = Args::try_parse_from(["note", "edit"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_edit_full_options() {
        let cli = Args::try_parse_from([
            "note", "edit", "1",
            "-t", "标题",
            "-c", "dev",
            "-T", "rust",
            "-p", "urgent",
            "-a", "append text",
        ]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Edit { id, title, category, tags, priority, append }) = cli.unwrap().commands {
            assert_eq!(id, 1);
            assert_eq!(title.unwrap(), "标题");
            assert_eq!(category.unwrap(), "dev");
            assert_eq!(tags.unwrap(), vec!["rust"]);
            assert!(matches!(priority, Some(Priority::URGENT)));
            assert_eq!(append.unwrap(), "append text");
        } else {
            panic!("期望 Edit 命令");
        }
    }

    // ---- Delete ----
    #[test]
    fn test_delete_with_force() {
        let cli = Args::try_parse_from(["note", "delete", "5", "-f"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Delete { id, force, .. }) = cli.unwrap().commands {
            assert_eq!(id, 5);
            assert!(force);
        } else {
            panic!("期望 Delete 命令");
        }
    }

    #[test]
    fn test_delete_without_force() {
        let cli = Args::try_parse_from(["note", "delete", "5"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Delete { id, force, .. }) = cli.unwrap().commands {
            assert_eq!(id, 5);
            assert!(!force);
        } else {
            panic!("期望 Delete 命令");
        }
    }

    #[test]
    fn test_delete_missing_id() {
        let result = Args::try_parse_from(["note", "delete"]);
        assert!(result.is_err());
    }

    // ---- List ----
    #[test]
    fn test_list_defaults() {
        let cli = Args::try_parse_from(["note", "list"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::List { sort, limit, offset, category, tag, priority, date, hastag, notag }) = cli.unwrap().commands {
            assert!(sort.is_none());
            assert!(limit.is_none());
            assert!(offset.is_none());
            assert!(category.is_none());
            assert!(tag.is_none());
            assert!(priority.is_none());
            assert!(date.is_none());
            assert!(!hastag);
            assert!(!notag);
        } else {
            panic!("期望 List 命令");
        }
    }

    #[test]
    fn test_list_with_filters() {
        let cli = Args::try_parse_from([
            "note", "list",
            "-s", "created",
            "-n", "10",
            "-o", "5",
            "-c", "work",
            "-T", "tag1", "-T", "tag2",
            "-p", "high",
            "-d", "today",
            "--has-tag",
        ]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::List { sort, limit, offset, category, tag, priority, date, hastag, notag }) = cli.unwrap().commands {
            assert!(matches!(sort, Some(SortType::Created)));
            assert_eq!(limit.unwrap(), 10);
            assert_eq!(offset.unwrap(), 5);
            assert_eq!(category.unwrap(), "work");
            assert_eq!(tag.unwrap(), vec!["tag1", "tag2"]);
            assert_eq!(priority.unwrap(), "high");
            assert_eq!(date.unwrap(), "today");
            assert!(hastag);
            assert!(!notag);
        } else {
            panic!("期望 List 命令");
        }
    }

    #[test]
    fn test_list_invalid_sort() {
        let result = Args::try_parse_from(["note", "list", "-s", "invalid"]);
        assert!(result.is_err());
    }

    // ---- Search ----
    #[test]
    fn test_search_defaults() {
        let cli = Args::try_parse_from(["note", "search"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Search { mode, casesensitive }) = cli.unwrap().commands {
            assert!(mode.is_none());
            assert!(!casesensitive);
        } else {
            panic!("期望 Search 命令");
        }
    }

    #[test]
    fn test_search_with_options() {
        let cli = Args::try_parse_from(["note", "search", "-m", "regex", "--case-sensitive"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Search { mode, casesensitive }) = cli.unwrap().commands {
            assert!(matches!(mode, Some(SearchMode::Regex)));
            assert!(casesensitive);
        } else {
            panic!("期望 Search 命令");
        }
    }

    #[test]
    fn test_search_invalid_mode() {
        let result = Args::try_parse_from(["note", "search", "-m", "invalid"]);
        assert!(result.is_err());
    }

    // ---- Category ----
    #[test]
    fn test_category_list() {
        let cli = Args::try_parse_from(["note", "category", "list"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Category(CategoryCommand::List)) = cli.unwrap().commands {
        } else {
            panic!("期望 Category List 命令");
        }
    }

    #[test]
    fn test_category_tree() {
        let cli = Args::try_parse_from(["note", "category", "tree"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Category(CategoryCommand::Tree)) = cli.unwrap().commands {
        } else {
            panic!("期望 Category Tree 命令");
        }
    }

    #[test]
    fn test_category_rename() {
        let cli = Args::try_parse_from(["note", "category", "rename", "old", "new"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Category(CategoryCommand::Rename { old_name, new_name })) = cli.unwrap().commands {
            assert_eq!(old_name.unwrap(), "old");
            assert_eq!(new_name.unwrap(), "new");
        } else {
            panic!("期望 Category Rename 命令");
        }
    }

    #[test]
    fn test_category_delete_force() {
        let cli = Args::try_parse_from(["note", "category", "delete", "-f"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Category(CategoryCommand::Delete { force })) = cli.unwrap().commands {
            assert!(force);
        } else {
            panic!("期望 Category Delete 命令");
        }
    }

    // ---- Tag ----
    #[test]
    fn test_tag_list() {
        let cli = Args::try_parse_from(["note", "tag", "list"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Tag(TagCommand::List)) = cli.unwrap().commands {
        } else {
            panic!("期望 Tag List 命令");
        }
    }

    #[test]
    fn test_tag_rename() {
        let cli = Args::try_parse_from(["note", "tag", "rename", "old-tag", "new-tag"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Tag(TagCommand::Rename { old_name, new_name })) = cli.unwrap().commands {
            assert_eq!(old_name.unwrap(), "old-tag");
            assert_eq!(new_name.unwrap(), "new-tag");
        } else {
            panic!("期望 Tag Rename 命令");
        }
    }

    #[test]
    fn test_tag_delete_force() {
        let cli = Args::try_parse_from(["note", "tag", "delete", "-f"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Tag(TagCommand::Delete { force })) = cli.unwrap().commands {
            assert!(force);
        } else {
            panic!("期望 Tag Delete 命令");
        }
    }

    // ---- 枚举值完整性 ----
    #[test]
    fn test_priority_variants() {
        for input in ["low", "normal", "high", "urgent"] {
            let cli = Args::try_parse_from(["note", "add", "-p", input]);
            assert!(cli.is_ok(), "priority={} 应该合法", input);
        }
    }

    #[test]
    fn test_sort_variants() {
        for input in ["created", "modified", "title", "priority"] {
            let cli = Args::try_parse_from(["note", "list", "-s", input]);
            assert!(cli.is_ok(), "sort={} 应该合法", input);
        }
    }

    #[test]
    fn test_search_mode_variants() {
        for input in ["plain", "regex", "fuzzy"] {
            let cli = Args::try_parse_from(["note", "search", "-m", input]);
            assert!(cli.is_ok(), "mode={} 应该合法", input);
        }
    }

    // ---- Export ----
    #[test]
    fn test_export_basic() {
        let cli = Args::try_parse_from(["note", "export", "-f", "json"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Export { format, path, id, all, category, tag, date }) = cli.unwrap().commands {
            assert!(matches!(format, ExportFormat::Json));
            assert!(path.is_none());
            assert!(id.is_none());
            assert!(!all);
            assert!(category.is_none());
            assert!(tag.is_none());
            assert!(date.is_none());
        } else {
            panic!("期望 Export 命令");
        }
    }

    #[test]
    fn test_export_with_all_options() {
        let cli = Args::try_parse_from([
            "note", "export",
            "-f", "csv",
            "-p", "./out",
            "-i", "1", "-i", "2",
            "--all",
            "-c", "work",
            "-T", "tag1",
            "-d", "today",
        ]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Export { format, path, id, all, category, tag, date }) = cli.unwrap().commands {
            assert!(matches!(format, ExportFormat::Csv));
            assert_eq!(path.unwrap().to_str().unwrap(), "./out");
            assert_eq!(id.unwrap(), vec!["1", "2"]);
            assert!(all);
            assert_eq!(category.unwrap(), "work");
            assert_eq!(tag.unwrap(), vec!["tag1"]);
            assert_eq!(date.unwrap(), "today");
        } else {
            panic!("期望 Export 命令");
        }
    }

    #[test]
    fn test_export_missing_format() {
        let result = Args::try_parse_from(["note", "export"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_export_format_variants() {
        for input in ["json", "markdown", "txt", "csv"] {
            let cli = Args::try_parse_from(["note", "export", "-f", input]);
            assert!(cli.is_ok(), "format={} 应该合法", input);
        }
    }

    // ---- Import ----
    #[test]
    fn test_import_basic() {
        let cli = Args::try_parse_from(["note", "import", "notes.json"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Import { path, category, tags }) = cli.unwrap().commands {
            assert_eq!(path.to_str().unwrap(), "notes.json");
            assert!(category.is_none());
            assert!(tags.is_none());
        } else {
            panic!("期望 Import 命令");
        }
    }

    #[test]
    fn test_import_with_options() {
        let cli = Args::try_parse_from([
            "note", "import", "data.json",
            "-c", "imported",
            "-T", "tag1", "-T", "tag2",
        ]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Import { path, category, tags }) = cli.unwrap().commands {
            assert_eq!(path.to_str().unwrap(), "data.json");
            assert_eq!(category.unwrap(), "imported");
            assert_eq!(tags.unwrap(), vec!["tag1", "tag2"]);
        } else {
            panic!("期望 Import 命令");
        }
    }

    #[test]
    fn test_import_missing_path() {
        let result = Args::try_parse_from(["note", "import"]);
        assert!(result.is_err());
    }

    // ---- Stats ----
    #[test]
    fn test_stats() {
        let cli = Args::try_parse_from(["note", "stats"]);
        assert!(cli.is_ok());
        assert!(matches!(cli.unwrap().commands, Some(NoteCommand::Stats)));
    }

    // ---- Log ----
    #[test]
    fn test_log_defaults() {
        let cli = Args::try_parse_from(["note", "log"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Log { week, date }) = cli.unwrap().commands {
            assert!(!week);
            assert!(date.is_none());
        } else {
            panic!("期望 Log 命令");
        }
    }

    #[test]
    fn test_log_week() {
        let cli = Args::try_parse_from(["note", "log", "--week"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Log { week, date }) = cli.unwrap().commands {
            assert!(week);
            assert!(date.is_none());
        } else {
            panic!("期望 Log 命令");
        }
    }

    #[test]
    fn test_log_with_date() {
        let cli = Args::try_parse_from(["note", "log", "--date", "2024-01-15"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Log { week, date }) = cli.unwrap().commands {
            assert!(!week);
            assert_eq!(date.unwrap(), "2024-01-15");
        } else {
            panic!("期望 Log 命令");
        }
    }

    // ---- Index ----
    #[test]
    fn test_index_rebuild() {
        let cli = Args::try_parse_from(["note", "index", "rebuild"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Index(IndexCommand::Rebuild)) = cli.unwrap().commands {
        } else {
            panic!("期望 Index Rebuild 命令");
        }
    }

    // ---- Config ----
    #[test]
    fn test_config_list() {
        let cli = Args::try_parse_from(["note", "config", "list"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Config(ConfigCommand::List)) = cli.unwrap().commands {
        } else {
            panic!("期望 Config List 命令");
        }
    }

    #[test]
    fn test_config_get() {
        let cli = Args::try_parse_from(["note", "config", "get", "display.color"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Config(ConfigCommand::Get { key })) = cli.unwrap().commands {
            assert_eq!(key, "display.color");
        } else {
            panic!("期望 Config Get 命令");
        }
    }

    #[test]
    fn test_config_set() {
        let cli = Args::try_parse_from(["note", "config", "set", "display.color", "false"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Config(ConfigCommand::Set { key, value })) = cli.unwrap().commands {
            assert_eq!(key, "display.color");
            assert_eq!(value, "false");
        } else {
            panic!("期望 Config Set 命令");
        }
    }

    #[test]
    fn test_config_get_missing_key() {
        let result = Args::try_parse_from(["note", "config", "get"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_set_missing_value() {
        let result = Args::try_parse_from(["note", "config", "set", "key"]);
        assert!(result.is_err());
    }

    // ---- Completion ----
    #[test]
    fn test_completion() {
        let cli = Args::try_parse_from(["note", "completion", "bash"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Completion { shell }) = cli.unwrap().commands {
            assert!(matches!(shell, ShellType::Bash));
        } else {
            panic!("期望 Completion 命令");
        }
    }

    #[test]
    fn test_completion_missing_shell() {
        let result = Args::try_parse_from(["note", "completion"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_completion_shell_variants() {
        for input in ["bash", "zsh", "fish", "powershell", "elvish"] {
            let cli = Args::try_parse_from(["note", "completion", input]);
            assert!(cli.is_ok(), "shell={} 应该合法", input);
        }
    }

    #[test]
    fn test_completion_invalid_shell() {
        let result = Args::try_parse_from(["note", "completion", "invalid"]);
        assert!(result.is_err());
    }

    // ---- Pin / Unpin ----
    #[test]
    fn test_pin() {
        let cli = Args::try_parse_from(["note", "pin", "abc123"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Pin { id }) = cli.unwrap().commands {
            assert_eq!(id, "abc123");
        } else {
            panic!("期望 Pin 命令");
        }
    }

    #[test]
    fn test_pin_missing_id() {
        let result = Args::try_parse_from(["note", "pin"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_unpin() {
        let cli = Args::try_parse_from(["note", "unpin", "abc123"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Unpin { id }) = cli.unwrap().commands {
            assert_eq!(id, "abc123");
        } else {
            panic!("期望 Unpin 命令");
        }
    }

    // ---- Archive / Unarchive ----
    #[test]
    fn test_archive() {
        let cli = Args::try_parse_from(["note", "archive", "42"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Archive { id }) = cli.unwrap().commands {
            assert_eq!(id, "42");
        } else {
            panic!("期望 Archive 命令");
        }
    }

    #[test]
    fn test_archive_missing_id() {
        let result = Args::try_parse_from(["note", "archive"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_unarchive() {
        let cli = Args::try_parse_from(["note", "unarchive", "42"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Unarchive { id }) = cli.unwrap().commands {
            assert_eq!(id, "42");
        } else {
            panic!("期望 Unarchive 命令");
        }
    }

    // ---- Done ----
    #[test]
    fn test_done() {
        let cli = Args::try_parse_from(["note", "done", "99"]);
        assert!(cli.is_ok());
        if let Some(NoteCommand::Done { id }) = cli.unwrap().commands {
            assert_eq!(id, "99");
        } else {
            panic!("期望 Done 命令");
        }
    }

    #[test]
    fn test_done_missing_id() {
        let result = Args::try_parse_from(["note", "done"]);
        assert!(result.is_err());
    }
}