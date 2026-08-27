use clap::{Parser,
    Args, 
    Subcommand, 
    ValueEnum
};

const LOGO: &str = "\x1b[1;38;2;46;49;146m
  ███▄▄  ███  ▄██████▄  █████████ ████████ ▄███████▄
  ███▀██▄███ ███▀  ▀███    ███    ███      ███▄▄▄▄
  ███  ▀▀███ ███    ███    ███    ███▀▀▀    ▀██████▄
  ███    ███ ▀███▄▄███▀    ███    ███▄▄▄▄▄ ▄▄▄▄▄▄███
  ▀▀▀    ▀▀▀   ▀▀▀▀▀▀      ▀▀▀    ▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀\x1b[0m";

#[derive(Parser)]
#[command(name = "notes")]
#[command(about = "  CLI tool for locally saving, viewing, editing and exporting notes")]
#[command(before_help = LOGO)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get all notes
    All(AllCommand),
    /// Get specific note
    Get(GetCommand),
    /// Add new note to database
    New(NewCommand),
    /// Change content of saved note
    Edit(EditCommand),
    /// Delete saved note
    Delete(DeleteCommand),
    /// Search notes for specific content
    Search(SearchCommand),
    /// Export all notes to file (txt, md, html, json, png, pdf)
    Export(ExportCommand),
    /// Import a files (txt, md, html) contents as plain text to database 
    Import(ImportCommand),
    #[command(subcommand)]
    /// Attach a tag to a note, delete tag, list all tags
    Tag(TagCommand),
    /// Create user account for serivce
    Register,
    /// Login to service, save credentials to local machine
    Login,
    /// Delete saved login credentials from local machine 
    Logout,
    /// Prints the programs version number
    Version
}


#[derive(Parser, Debug, Clone)]
pub struct AllCommand {
    #[arg(short='c', long="content", help="Optional flag to also display content of all notes")]
    pub content: bool,
}

#[derive(Args, Clone)]
pub struct GetCommand {
    #[arg(help="ID of the note to fetch")]
    pub id: i64,
}

#[derive(Args, Clone)]
pub struct NewCommand {
    pub title: String,
    pub content: String,
}

#[derive(Parser, Debug)]
pub struct EditCommand {
    pub id: i64,
    pub field: EditField,
}

#[derive(Parser, Debug, Clone)]
pub struct DeleteCommand {
    #[arg(help="ID of the note to delete")]
    pub id: i64,
    #[arg(short='a', long="all", help="Delete all notes")]
    pub all: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct SearchCommand {
    pub field: SearchField,
    #[arg(help="Pattern to search matches for")]
    pub pattern: String
}

#[derive(Parser, Debug, Clone)]
pub struct ExportCommand {
    pub filetype: String,
}

#[derive(Parser, Debug, Clone)]
pub struct ImportCommand {
    #[arg(help="Give file path from command line or choose through file dialog")]
    pub file: Option<String>,
}

#[derive(Subcommand)]
pub enum TagCommand {
    #[command(about = "Add new tag to note with note ID")]
    Add {
        note_id: i64,
        name: String,
    },

    #[command(about = "Delete a tag by tag name")]
    Delete {
        name: String,
    },

    #[command(about = "List all tags and how many notes use them")]
    List,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum EditField {
    Title,
    Content,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum SearchField {
    Title,
    Content,
    Tag
}