mod user;

pub use user::*;

use crate::{forbidden, AppState};
use aws_smithy_http_server::Extension;
use jwt_simple::reexports::coarsetime::Duration;
use std::sync::Arc;
use tracing::info;
use user_server_sdk::{error, input, output};

pub async fn health(input: input::HealthInput) -> Result<output::HealthOutput, error::HealthError> {
    info!("user: {:?}", input);
    let message = input.message;
    let output = output::HealthOutput { message };
    Ok(output)
}

const ACCESS_TOKEN_DURATION: u64 = 14;
const REFRESH_TOKEN_DURATION: u64 = 365;
pub async fn signin(
    input: input::SigninInput,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<output::SigninOutput, error::SigninError> {
    info!("signin: {:?}", input);
    let signer = &state.signer;
    let username = input.username;
    if input.password.len() < 8 {
        forbidden!("invalid password");
    }
    let duration = Duration::from_days(ACCESS_TOKEN_DURATION);
    let access_token = signer.sign(username.clone(), duration)?;
    let refresh_token = signer.sign(username, Duration::from_days(REFRESH_TOKEN_DURATION))?;
    Ok(output::SigninOutput {
        access_token,
        refresh_token,
        expires_in: duration.as_secs() as _,
    })
}
