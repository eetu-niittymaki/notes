use rusqlite::{Connection};
use clap::Parser;
use cli::{Cli, Commands};

use crate::config::get_db_path;

mod config;
mod cli;
mod commands;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn try_main() -> rusqlite::Result<()> {
    let cli = Cli::parse();
    let db_path = get_db_path();

    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            ID INTEGER PRIMARY KEY,
            NOTE TEXT NOT NULL
        )",
        (),
    )?;

    run(cli, &conn)
}

fn run(cli: Cli, conn: &Connection) -> rusqlite::Result<()> {
    match cli.command {
        Some(Commands::New(cmd)) => commands::new::new(cmd, conn)?,
        Some(Commands::Update(cmd)) => commands::update::update(cmd, conn)?,
        Some(Commands::Delete(cmd)) => commands::delete::delete(cmd, conn)?,
        Some(Commands::All(_)) => commands::all::all(conn)?,
        Some(Commands::Search(cmd)) => commands::search::search(cmd, conn)?,
        Some(Commands::Version) => commands::version::version(),
        None => {}
    }

    Ok(())
}