use clap::{Parser, Args, Subcommand};

#[derive(Parser)]
#[command(name = "notes")]
#[command(about = "CLI tool for saving and exporting notes")]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>
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
    Update(UpdateCommand),
    /// Delete saved note
    Delete(DeleteCommand),
    /// Search notes for specific content
    Search(SearchCommand),
    /// Export all notes to file (txt, md, html, png, pdf)
    Export(ExportCommand),
    /// Import a files (txt, md, html) contents as plain text to database 
    Import(ImportCommand),
    #[command(subcommand)]
    /// Attach a tag to a note, delete tag, list all tags
    Tag(TagCommand),
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
    #[arg(short='i', long="id", help="ID of the note to fetch")]
    pub id: Option<i64>,
    #[arg(short='t', long="title", help="Title of the note to fetch")]
    pub title: Option<String>,
}

#[derive(Args, Clone)]
pub struct NewCommand {
    pub title: String,
    pub content: String,
}

#[derive(Parser, Debug, Clone)]
pub struct UpdateCommand {
    #[arg(short='i', long="id", help="ID of the note to update")]
    pub id: Option<i64>,
    #[arg(short='t', long="title", help="Title of the note to update")]
    pub title: Option<String>,
    #[arg(short='n', long="new-title", help="New title for note")]
    pub new_title: Option<String>,
    #[arg(short='c', long="content", help="New content for note")]
    pub new_content: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct DeleteCommand {
    #[arg(short='t', long="title", help="Title of the note to delete")]
    pub title: Option<String>,
    #[arg(short='i', long="id", help="ID of the note to delete")]
    pub id: Option<i64>,
    #[arg(short='a', long="all", help="Delete all notes")]
    pub all: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct SearchCommand {
    #[arg(short='t', long="title", help="Title of the note to search")]
    pub title: Option<String>,
    #[arg(short='c', long="content", help="Content to search for in notes")]
    pub content: Option<String>,
    #[arg(long="tag", help="Search for notes that have specific tag")]
    pub tag: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct ExportCommand {
    pub filetype: String,
}

#[derive(Parser, Debug, Clone)]
pub struct ImportCommand {
    pub file: String,
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
