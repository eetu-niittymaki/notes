use clap::Parser;

use notes_core::error::Result;

use models::cli::{Cli, Commands};

use crate::client::ApiClient;
use crate::auth::credential_manager;
use crate::auth::auth::{login, register};
use crate::utils::get_user_input::get_user_input;

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

    let mut api = ApiClient::new(config::BASE_URL, config::AUTH_URL, None);

    // So that these commands can be run without tokens set
    match &cli.command {
        Some(Commands::Login)
        | Some(Commands::Register)
        | Some(Commands::Logout)
        | Some(Commands::Version) => {
            return run(cli, &api).await;
        }

        _ => {}
    }

    let token = match credential_manager::load_tokens().await? {
        Some(token) => token,
        None => {
            loop {
                println!("Not logged in.");
                println!("[L]ogin");
                println!("[R]egister new user");
                println!("[Q]uit");

                match get_user_input().as_str() {
                    "l" => {
                        break login(&api).await?;
                    }
                    "r" => {
                        break register(&api).await?;
                    }
                    "q" => std::process::exit(0),
                    _ => {
                        println!("Please give valid input");
                    }
                }
            }
        }
    };

    api.set_token(token.access_token);
    
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
        Some(Commands::Register) => commands::register::register(api).await?,
        Some(Commands::Login) => commands::login::login(api).await?,
        Some(Commands::Logout) => commands::logout::logout().await?,
        Some(Commands::Version) => commands::version::version(),
        None => {}
    }

    Ok(())
}