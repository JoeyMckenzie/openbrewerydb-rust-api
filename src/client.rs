//! A top-level client for coordinating interactions with the Open Brewery DB API.

use reqwest::StatusCode;
use std::time::Duration;

use serde::Deserialize;

use crate::errors::OpenBreweryResult;

/// Default the reqwest HTTP timeout to 5 seconds, overridable if provided.
const DEFAULT_TIMEOUT_SECONDS: u8 = 5;

/// Default API base URL for the Open Brewery DB API.
const API_BASE_URL: &str = "https://api.openbrewerydb.org/v1/breweries";

/// The primary BubbleHearth client, acting as the gateway for connecting.
///
/// ```rust
/// use std::time::Duration;
///
/// use openbrewerydb::client::OpenBreweryClient;
/// use openbrewerydb::errors::OpenBreweryResult;
///
/// #[tokio::main]
/// async fn main() -> OpenBreweryResult<()> {
/// let client = OpenBreweryClient::new();
///     let brewery = client.find("b54b16e1-ac3b-4bff-a11f-f7ae9ddc27e0").await?;
///     dbg!(brewery);
///     Ok(())
/// }
#[derive(Debug)]
pub struct OpenBreweryClient {
    http: reqwest::Client,
}

impl Default for OpenBreweryClient {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenBreweryClient {
    /// Constructs a new client with the default timeout.
    pub fn new() -> Self {
        let default_timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECONDS.into());
        Self::new_with_timeout(default_timeout)
    }

    /// Constructs a new client with a configurable timeout.
    pub fn new_with_timeout(timeout: Duration) -> Self {
        let http = reqwest::ClientBuilder::new()
            .timeout(timeout)
            .build()
            .unwrap();

        Self { http }
    }

    /// Sends a request with the required namespace and authentication token.
    async fn send_request(&self, url: &str) -> OpenBreweryResult<reqwest::Response> {
        let uri = format!("{API_BASE_URL}/{url}");
        let response = self.http.get(uri).send().await?;
        Ok(response)
    }

    /// Sends a request with the required namespace and authentication token and deserializes the response.
    pub(crate) async fn send_request_and_deserialize<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
    ) -> OpenBreweryResult<T> {
        let response = self.send_request(url).await?.json::<T>().await?;
        Ok(response)
    }

    /// Sends a request with the required namespace and authentication token and deserializes the response.
    pub(crate) async fn send_request_and_optionally_deserialize<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
    ) -> OpenBreweryResult<Option<T>> {
        let response = self.send_request(url).await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let response = response.json::<T>().await?;

        Ok(Some(response))
    }
}
