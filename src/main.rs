use rusqlite::{Connection};
use clap::Parser;
use cli::{Cli, Commands};

use crate::config::get_db_path;
use crate::db::create_tables::create_tables;

mod config;
mod cli;
mod commands;
mod models;
mod db;
mod utils;

fn main() {
    // Program wide error catcher
    if let Err(e) = try_main() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn try_main() -> rusqlite::Result<()> {
    let cli = Cli::parse();
    let db_path = get_db_path();

    let conn = Connection::open(&db_path)?;

    create_tables(&conn)?;

    run(cli, &conn)
}

fn run(cli: Cli, conn: &Connection) -> rusqlite::Result<()> {
    match cli.command {
        Some(Commands::All(_)) => commands::all::all(conn)?,
        Some(Commands::New(cmd)) => commands::new::new(cmd, conn)?,
        Some(Commands::Update(cmd)) => commands::update::update(cmd, conn)?,
        Some(Commands::Delete(cmd)) => commands::delete::delete(cmd, conn)?,
        Some(Commands::Search(cmd)) => commands::search::search(cmd, conn)?,      
        Some(Commands::Export(cmd)) => commands::export::export(cmd, conn)?,
        Some(Commands::Import(cmd)) => commands::import::import(cmd, conn)?,
        Some(Commands::Tag(cmd)) => commands::tag::tag(cmd, conn)?,
        Some(Commands::Version) => commands::version::version(),
        None => {}
    }

    Ok(())
}