use anyhow::Result;
use user_client_sdk::{config::Token, Client, Config};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::builder()
        .endpoint_url("http://localhost:3000/api")
        .behavior_version_latest()
        .build();
    let client = Client::from_conf(config);

    println!("\n--- Calling user_message operation without authentication");
    let ret = client.health().message("example").send().await;
    println!("{:?}", ret);

    println!("\n--- Calling signin operation to get a token");

    let ret = client
        .signin()
        .username("test")
        .password("abcd12345")
        .send()
        .await;
    println!("{:?}", ret);

    let token = ret?.access_token;

    println!("\n-- Calling user_message operation with authentication");

    let config = Config::builder()
        .endpoint_url("http://localhost:3000/api")
        .bearer_token(Token::new(token, None))
        .behavior_version_latest()
        .build();
    let client = Client::from_conf(config);

    let ret = client.health().message("example").send().await?;

    println!("{:?}", ret);

    Ok(())
}
