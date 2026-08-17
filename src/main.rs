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

#[tokio::main]
async fn main() {
    // Program wide error catcher
    if let Err(e) = try_main().await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

async fn try_main() -> Result<()> {
    let cli = Cli::parse();
    let db_path = get_db_path();

    let db = Database::open(&db_path).await?;

    run(cli, &db).await
}

async fn run(cli: Cli, db: &Database) -> Result<()> {
    match cli.command {
        Some(Commands::All(cmd)) => commands::all::all(cmd, db).await?,
        Some(Commands::Get(cmd)) => commands::get::get(cmd, db).await?,
        Some(Commands::New(cmd)) => commands::new::new(cmd, db).await?,
        Some(Commands::Edit(cmd)) => commands::edit::edit(cmd, db).await?,
        Some(Commands::Delete(cmd)) => commands::delete::delete(cmd, db).await?,
        Some(Commands::Search(cmd)) => commands::search::search(cmd, db).await?,      
        Some(Commands::Export(cmd)) => commands::export::export(cmd, db).await?,
        Some(Commands::Import(cmd)) => commands::import::import(cmd, db).await?,
        Some(Commands::Tag(cmd)) => commands::tag::tag(cmd, db).await?,
        Some(Commands::Version) => commands::version::version(),
        None => {}
    }

    Ok(())
}