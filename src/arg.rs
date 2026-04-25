use std::path::PathBuf;
use clap::{Parser,Subcommand,ValueEnum};
use crate::notes::storage::DataBaseStorage;
use crate::notes::output::Output;
use crate::notes::config::Config;
use crate::notes::handle;
#[derive(Parser)]
#[command(name = "note", bin_name = "note", version, about = "NoteCli - A lightweight CLI note manager")]
struct Args {
    #[command(subcommand)]
    commands:Option<NoteCommand>
}

#[derive(Subcommand)]
enum NoteCommand{
    /// Create a new note
    Add{
        /// Note content
        content: Option<String>,
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
        id: Option<u32>,
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
    /// Delete notes by ID, tag, or category
    Delete{
        /// Note ID to delete
        id: Option<u32>,
        /// Delete all notes with this tag (can specify multiple)
        #[arg(short = 'T', long = "tag", num_args = 1..=20)]
        tag: Option<Vec<String>>,
        /// Delete all notes in this category
        #[arg(short = 'c', long = "category")]
        category: Option<String>,
        /// Skip confirmation prompt
        #[arg(short = 'f', long = "force")]
        force: bool,
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
        keyword: Option<String>,
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
        format: Option<ExportFormat>,
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
        path: Option<PathBuf>,
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

    /// Pin a note to top of list
    Pin{
        /// Note ID to pin
        id: Option<String>,
    },

    /// Unpin a note
    Unpin{
        /// Note ID to unpin
        id: Option<String>,
    },

    /// Archive a note (hide from default list)
    Archive{
        /// Note ID to archive
        id: Option<String>,
    },

    /// Unarchive a note
    Unarchive{
        /// Note ID to unarchive
        id: Option<String>,
    },

    /// Mark a note as done
    Done{
        /// Note ID to mark as done
        id: Option<String>,
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
        key: Option<String>,
    },
    /// Set a config value
    Set{
        /// Config key name
        key: Option<String>,
        /// Config value
        value: Option<String>,
    },
}


pub fn arg_setup(storage: &mut DataBaseStorage, output: &Output, config_path: &PathBuf, config: &Config){
    let cli: Args = Args::parse();
    match cli.commands {
        None => {
            println!("未指定子命令，使用 --help 查看帮助");
        }
        Some(NoteCommand::Add {content, title, category, tags, priority }) => {
            let p = priority.map(|p| format!("{:?}", p).to_lowercase());
            handle::add::handle(&content, &title, &category, &tags, &p, storage, output, config);
        }
        Some(NoteCommand::Show { id, raw }) => {
            let id = id.and_then(|s| s.parse::<u32>().ok());
            handle::show::handle(&id, raw, storage, output, config);
        }
        Some(NoteCommand::Edit { id, title, category, tags, priority, content, append }) => {
            let p = priority.map(|p| format!("{:?}", p).to_lowercase());
            handle::edit::handle(id, &title, &category, &tags, &p, &content, &append, storage, output);
        }
        Some(NoteCommand::Delete { id, tag, category, force }) => {
            handle::delete::handle(id, &tag, &category, force, storage, output);
        }
        Some(NoteCommand::List { sort, limit, offset, category, tag, priority, date, hastag, notag }) => {
            let s = sort.map(|s| format!("{:?}", s).to_lowercase());
            handle::list::handle(&s, &limit, &offset, &category, &tag, &priority, &date, hastag, notag, storage, output, config);
        }
        Some(NoteCommand::Search { keyword, mode, casesensitive }) => {
            let m = mode.map(|m| format!("{:?}", m).to_lowercase());
            let cs = casesensitive || config.search.case_sensitive;
            handle::search::handle(&keyword, &m, cs, storage, output, config);
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
            }
        }
        Some(NoteCommand::Export { format, path, id, all, category, tag, date }) => {
            let f = format.map(|f| match f {
                ExportFormat::Json => "json",
                ExportFormat::Markdown => "markdown",
                ExportFormat::Txt => "txt",
                ExportFormat::Csv => "csv",
            }.to_string());
            handle::export::handle(&f, &path, &id, all, &category, &tag, &date, storage, output);
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
        Some(NoteCommand::Pin { id }) => {
            let id = id.as_ref().and_then(|s| s.parse::<u32>().ok());
            handle::pin::pin(id, storage, output);
        }
        Some(NoteCommand::Unpin { id }) => {
            let id = id.as_ref().and_then(|s| s.parse::<u32>().ok());
            handle::pin::unpin(id, storage, output);
        }
        Some(NoteCommand::Archive { id }) => {
            let id = id.as_ref().and_then(|s| s.parse::<u32>().ok());
            handle::archive::archive(id, storage, output);
        }
        Some(NoteCommand::Unarchive { id }) => {
            let id = id.as_ref().and_then(|s| s.parse::<u32>().ok());
            handle::archive::unarchive(id, storage, output);
        }
        Some(NoteCommand::Done { id }) => {
            let id = id.as_ref().and_then(|s| s.parse::<u32>().ok());
            handle::done::handle(id, storage, output);
        }
    }
}
