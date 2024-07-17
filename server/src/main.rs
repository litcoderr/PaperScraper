use hyper;
use hyper_rustls;
use tokio;
use google_sheets4::{oauth2, Sheets};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the credentials file
    let secret = oauth2::read_application_secret("credentials.json").await.expect("Failed to read application secret");

    // Set up the authenticator
    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();

    let client = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);

    Ok(())
}