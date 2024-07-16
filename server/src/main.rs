use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the credentials file
    let secret = yup_oauth2::read_application_secret("credentials.json")
        .await
        .expect("Failed to read application secret");

    // Set up the authenticator
    let auth = InstalledFlowAuthenticator::builder(
        secret,
        InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .expect("Failed to set up authenticator");

    Ok(())
}