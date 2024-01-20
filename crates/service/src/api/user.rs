use tracing::info;
use user_server_sdk::{error, input, output};

pub async fn create_user(
    input: input::CreateUserInput,
) -> Result<output::CreateUserOutput, error::CreateUserError> {
    info!("create_user: {:?}", input);
    let output = output::CreateUserOutput {
        id: "user_id".to_string(),
    };
    Ok(output)
}

pub async fn get_user(
    input: input::GetUserInput,
) -> Result<output::GetUserOutput, error::GetUserError> {
    info!("get_user: {:?}", input);

    let output = output::GetUserOutput {
        id: "user_id".to_string(),
        email: "tchen@abc.com".to_string(),
        name: "Tian Chen".to_string(),
    };
    Ok(output)
}
