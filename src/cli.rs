use clap::{Parser, Subcommand};

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
    /// Add new note to database
    New(NewCommand),
    /// Change content of saved note
    Update(UpdateCommand),
    /// Delete saved note
    Delete(DeleteCommand),
    /// Get all notes
    All(AllCommand),
    /// Search for notes for specific content
    Search(SearchCommand),
    /// Prints the programs version number
    Version
}

#[derive(Parser, Debug, Clone)]
pub struct NewCommand {
    #[arg(short, long, help="Notes content")]
    pub note: String,
}

#[derive(Parser, Debug, Clone)]
pub struct UpdateCommand {
    #[arg(short, long, help="ID for note to change")]
    pub id:u32,
    #[arg(short, long, help="Notes new content")]
    pub new_content: String,
}

#[derive(Parser, Debug, Clone)]
pub struct DeleteCommand {
    #[arg(short, long, help="ID for note to delete")]
    pub id: u32,
}

#[derive(Parser, Debug, Clone)]
pub struct AllCommand {}

#[derive(Parser, Debug, Clone)]
pub struct SearchCommand {
    #[arg(short, long, help="Search content from notes")]
    pub content: String,
}
