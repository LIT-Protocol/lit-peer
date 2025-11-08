use crate::error::Result;
use crate::models;

#[async_trait::async_trait]
pub trait AuthMethodVerifier {
    async fn verify(
        &self,
        access_token: &str,
        http_client: reqwest::Client,
    ) -> Result<models::AuthMethodResponse>;
}
