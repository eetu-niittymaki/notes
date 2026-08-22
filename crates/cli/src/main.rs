use clap::Parser;

use notes_core::error::Result;

use models::cli::{Cli, Commands};

use crate::client::ApiClient;
use crate::auth::credential_manager;

mod models;
mod config;
mod commands;
mod client;
mod auth;
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

    let  api = ApiClient::new(config::SERVER_BASE_URL);

    match credential_manager::load_token()? {
        Some(token) => {
            println!("Token found")
            //run(cli, &api).await;
        }
        
        None => {
            println!("Not logged in.");
        }
    }

    run(cli, &api).await
}

async fn run(cli: Cli, api: &ApiClient) -> Result<()> {
    match cli.command {
        Some(Commands::All(cmd)) => commands::all::all(cmd, api).await?,
        Some(Commands::Get(cmd)) => commands::get::get(cmd, api).await?,
        Some(Commands::New(cmd)) => commands::new::new(cmd, api).await?,
        Some(Commands::Edit(cmd)) => commands::edit::edit(cmd, api).await?,
        Some(Commands::Delete(cmd)) => commands::delete::delete(cmd, api).await?,
        Some(Commands::Search(cmd)) => commands::search::search(cmd, api).await?,      
        Some(Commands::Export(cmd)) => commands::export::export(cmd, api).await?,
        Some(Commands::Import(cmd)) => commands::import::import(cmd, api).await?,
        Some(Commands::Tag(cmd)) => commands::tag::tag(cmd, api).await?,
        Some(Commands::Version) => commands::version::version(),
        None => {}
    }

    Ok(())
}