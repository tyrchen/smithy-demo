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
        email: "tchen@abc.com".to_string().try_into().unwrap(),
        name: "Tian Chen".to_string(),
    };
    Ok(output)
}

pub async fn list_users(
    input: input::ListUsersInput,
) -> Result<output::ListUsersOutput, error::ListUsersError> {
    info!("list_users: {:?}", input);
    let output = output::ListUsersOutput {
        users: vec![],
        next_token: None,
    };
    Ok(output)
}

pub async fn update_user(
    input: input::UpdateUserInput,
) -> Result<output::UpdateUserOutput, error::UpdateUserError> {
    info!("update_user: {:?}", input);
    let output = output::UpdateUserOutput {};
    Ok(output)
}

pub async fn delete_user(
    input: input::DeleteUserInput,
) -> Result<output::DeleteUserOutput, error::DeleteUserError> {
    info!("delete_user: {:?}", input);
    let output = output::DeleteUserOutput {};
    Ok(output)
}

pub async fn change_password(
    input: input::ChangePasswordInput,
) -> Result<output::ChangePasswordOutput, error::ChangePasswordError> {
    info!("change_password: {:?}", input);
    let output = output::ChangePasswordOutput {};
    Ok(output)
}

pub async fn get_user_by_email(
    input: input::GetUserByEmailInput,
) -> Result<output::GetUserByEmailOutput, error::GetUserByEmailError> {
    info!("get_user_by_email: {:?}", input);
    let output = output::GetUserByEmailOutput {
        id: "user_id".to_string(),
        name: "Tian Chen".to_string(),
        email: "tchen@abc.com".to_string().try_into().unwrap(),
    };
    Ok(output)
}
