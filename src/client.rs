//! A top-level client client for interacting with Blizzard Game Data APIs,
//! including authentication and all publicly available APIs for Blizzard games.

use std::time::Duration;

use reqwest::header::HeaderMap;
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
/// #[tokio::main]
/// async fn main() {
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
        let mut headers = HeaderMap::new();
        let uri = format!("{API_BASE_URL}/{url}");
        let response = self.http.get(uri).headers(headers).send().await?;

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
}
