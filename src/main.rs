use clap::Parser;
use cli::{Cli, Commands};

use crate::config::get_db_path;
use crate::db::Database;
use crate::error::Result;

mod config;
mod cli;
mod commands;
mod models;
mod error;
mod db;
mod utils;

fn main() {
    // Program wide error catcher
    if let Err(e) = try_main() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let cli = Cli::parse();
    let db_path = get_db_path();

    let db = Database::open(&db_path)?;

    run(cli, &db)
}

fn run(cli: Cli, db: &Database) -> Result<()> {
    match cli.command {
        Some(Commands::All(cmd)) => commands::all::all(cmd, db)?,
        Some(Commands::Get(cmd)) => commands::get::get(cmd, db)?,
        Some(Commands::New(cmd)) => commands::new::new(cmd, db)?,
        Some(Commands::Edit(cmd)) => commands::edit::edit(cmd, db)?,
        Some(Commands::Delete(cmd)) => commands::delete::delete(cmd, db)?,
        Some(Commands::Search(cmd)) => commands::search::search(cmd, db)?,      
        Some(Commands::Export(cmd)) => commands::export::export(cmd, db)?,
        Some(Commands::Import(cmd)) => commands::import::import(cmd, db)?,
        Some(Commands::Tag(cmd)) => commands::tag::tag(cmd, db)?,
        Some(Commands::Version) => commands::version::version(),
        None => {}
    }

    Ok(())
}