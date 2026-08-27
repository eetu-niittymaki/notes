use notes_core::error::Result;

use crate::{
    client::ApiClient,
    credential_manager::{self, Tokens},
    utils::get_user_input::get_user_input,
    
};

use notes_core::models::auth::{
    AuthResponse,
    LoginRequest,
    RegisterRequest
};

pub async fn login(api: &ApiClient) -> Result<Tokens> {
    print!("Username: ");
    let username = get_user_input().trim().to_owned();

    let password = rpassword::prompt_password("Password: ")?;

    let request = LoginRequest {
        username,
        password,
    };

    let response: AuthResponse = api
        .login(request)
        .await?;

    let tokens = Tokens {
        access_token: response.access_token,
        //refresh_token: response.refresh_token,
    };

    credential_manager::save_tokens(&tokens).await?;

    println!("Successfully logged in.");

    Ok(tokens)
}

pub async fn register(api: &ApiClient) -> Result<Tokens> {
    print!("Username: ");
    let username = get_user_input().trim().to_owned();

    let password = rpassword::prompt_password("Password: ")?;

    let request = RegisterRequest {
        username,
        password,
    };

    let response = match api.register(request).await {
        Ok(response) => {
            println!("Registered succesfully");
            response
        }
        Err(e) => {
            println!("Registeration failed: {e}");
            return Err(e);
        }
    };

    let tokens = Tokens {
        access_token: response.access_token,
        //refresh_token: response.refresh_token,
    };

    credential_manager::save_tokens(&tokens).await?;

    Ok(tokens)
}

/* 
pub async fn refresh(
    api: &ApiClient,
    refresh_token: &str,
) -> Result<Tokens> {
    let request = RefreshRequest {
        refresh_token: refresh_token.to_owned(),
    };

    let response: AuthResponse = api
        .post_public("/auth/refresh", &request)
        .await?;

    let tokens = Tokens {
        access_token: response.access_token,
        refresh_token: response.refresh_token,
    };

    credential_manager::save_tokens(&tokens)?;

    Ok(tokens)
}
*/