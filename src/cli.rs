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
    /// Add new note to database
    New(NewCommand),
    /// Change content of saved note
    Update(UpdateCommand),
    /// Delete saved note
    Delete(DeleteCommand),
    /// Search notes for specific content
    Search(SearchCommand),
    /// Export all notes to file
    Out(OutCommand),
    /// Prints the programs version number
    Version
}

#[derive(Parser, Debug, Clone)]
pub struct AllCommand {}

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
    pub new_content: String,
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
}

#[derive(Parser, Debug, Clone)]
pub struct OutCommand {
    pub filetype: String,
}
