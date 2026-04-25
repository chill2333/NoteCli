use std::path::PathBuf;
use clap::{Parser,Subcommand,ValueEnum};
use crate::notes::storage::DataBaseStorage;
use crate::notes::output::Output;
use crate::notes::handle;
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
        content:String,
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
        /// Replace note content entirely
        #[arg(short = 'C',long = "content")]
        content:Option<String>,
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
        /// Search keyword
        keyword: String,
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
        /// Category name to delete
        name: String,
        /// Skip confirmation prompt
        #[arg(short = 'f',long = "force")]
        force:bool,
        /// Keep note files but remove their category association
        #[arg(short = 'k',long = "keep")]
        keep:bool,
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
        /// Tag name to delete
        name: String,
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


pub fn arg_setup(storage: &mut DataBaseStorage, output: &Output, config_path: &PathBuf){
    let cli: Args = Args::parse();
    match cli.commands {
        None => {
            println!("未指定子命令，使用 --help 查看帮助");
        }
        Some(NoteCommand::Add {content, title, category, tags, priority }) => {
            let p = priority.map(|p| format!("{:?}", p).to_lowercase());
            handle::add::handle(&content, &title, &category, &tags, &p, storage, output);
        }
        Some(NoteCommand::Show { id, raw }) => {
            match id {
                Some(id_str) => match id_str.parse::<u32>() {
                    Ok(id) => handle::show::handle(&Some(id), raw, storage, output),
                    Err(_) => output.error(format!("无效的笔记ID: '{}'", id_str)),
                },
                None => handle::show::handle(&None, raw, storage, output),
            }
        }
        Some(NoteCommand::Edit { id, title, category, tags, priority, content, append }) => {
            let p = priority.map(|p| format!("{:?}", p).to_lowercase());
            handle::edit::handle(id, &title, &category, &tags, &p, &content, &append, storage, output);
        }
        Some(NoteCommand::Delete { id, force, .. }) => {
            handle::delete::handle(id, force, storage, output);
        }
        Some(NoteCommand::List { sort, limit, offset, category, tag, priority, date, hastag, notag }) => {
            let s = sort.map(|s| format!("{:?}", s).to_lowercase());
            handle::list::handle(&s, &limit, &offset, &category, &tag, &priority, &date, hastag, notag, storage, output);
        }
        Some(NoteCommand::Search { keyword, mode, casesensitive }) => {
            let m = mode.map(|m| format!("{:?}", m).to_lowercase());
            handle::search::handle(&keyword, &m, casesensitive, storage, output);
        }
        Some(NoteCommand::Category(cmd)) => {
            match cmd {
                CategoryCommand::List => {
                    handle::category::list(storage, output);
                }
                CategoryCommand::Tree => {
                    handle::category::tree(storage, output);
                }
                CategoryCommand::Rename { old_name, new_name } => {
                    handle::category::rename(&old_name, &new_name, storage, output);
                }
                CategoryCommand::Delete { name, force, keep } => {
                    handle::category::delete(&name, force, keep, storage, output);
                }
            }
        }
        Some(NoteCommand::Tag(cmd)) => {
            match cmd {
                TagCommand::List => {
                    handle::tag::list(storage, output);
                }
                TagCommand::Rename { old_name, new_name } => {
                    handle::tag::rename(&old_name, &new_name, storage, output);
                }
                TagCommand::Delete { name, force } => {
                    handle::tag::delete(&name, force, storage, output);
                }
            }
        }
        Some(NoteCommand::Export { format, path, id, all, category, tag, date }) => {
            let f = match format {
                ExportFormat::Json => "json",
                ExportFormat::Markdown => "markdown",
                ExportFormat::Txt => "txt",
                ExportFormat::Csv => "csv",
            };
            handle::export::handle(f, &path, &id, all, &category, &tag, &date, storage, output);
        }
        Some(NoteCommand::Import { path, category, tags }) => {
            handle::import::handle(&path, &category, &tags, storage, output);
        }
        Some(NoteCommand::Stats) => {
            handle::stats::handle(storage, output);
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
                    handle::config::list(output, config_path);
                }
                ConfigCommand::Get { key } => {
                    handle::config::get(&key, output, config_path);
                }
                ConfigCommand::Set { key, value } => {
                    handle::config::set(&key, &value, output, config_path);
                }
            }
        }
        Some(NoteCommand::Completion { shell }) => {
            println!("命令: Completion");
            println!("  shell: {:?}", shell);
        }
        Some(NoteCommand::Pin { id }) => {
            match id.parse::<u32>() {
                Ok(id) => handle::pin::pin(id, storage, output),
                Err(_) => output.error(format!("无效的笔记ID: '{}'", id)),
            }
        }
        Some(NoteCommand::Unpin { id }) => {
            match id.parse::<u32>() {
                Ok(id) => handle::pin::unpin(id, storage, output),
                Err(_) => output.error(format!("无效的笔记ID: '{}'", id)),
            }
        }
        Some(NoteCommand::Archive { id }) => {
            match id.parse::<u32>() {
                Ok(id) => handle::archive::archive(id, storage, output),
                Err(_) => output.error(format!("无效的笔记ID: '{}'", id)),
            }
        }
        Some(NoteCommand::Unarchive { id }) => {
            match id.parse::<u32>() {
                Ok(id) => handle::archive::unarchive(id, storage, output),
                Err(_) => output.error(format!("无效的笔记ID: '{}'", id)),
            }
        }
        Some(NoteCommand::Done { id }) => {
            match id.parse::<u32>() {
                Ok(id) => handle::done::handle(id, storage, output),
                Err(_) => output.error(format!("无效的笔记ID: '{}'", id)),
            }
        }
    }
}
