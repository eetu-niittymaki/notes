use rusqlite::{Connection, Result};
use clap::Parser;
use cli::{Cli, Commands};

use crate::config::DB_NAME;
use crate::config::get_db_path;

mod config;
mod cli;
mod commands;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let db_path = get_db_path();

    if !db_path.exists() {
        let conn = Connection::open(DB_NAME)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                ID INTEGER PRIMARY KEY,
                NAME TEXT NOT NULL
            )",
            (),
        )?;
    }

    match &cli.command {
        Some(Commands::New(cmd)) => {
            commands::new::new(cmd.clone());
        }
        Some(Commands::Update(cmd)) => {
            commands::update::update(cmd.clone());
        }
        Some(Commands::Delete(cmd)) => {
            commands::delete::delete(cmd.clone());
        }
        Some(Commands::All(cmd)) => {
            commands::all::all(cmd.clone());
        }
        Some(Commands::Search(cmd)) => {
            commands::search::search(cmd.clone());
        }
        Some(Commands::Version) => {
            commands::version::version();
        }
        None => {}
    }

    Ok(())
}
