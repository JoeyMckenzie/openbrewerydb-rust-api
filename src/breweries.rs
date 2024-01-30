//! Primary brewery client module, containing various models and endpoints for retrieving breweries.

use serde::Deserialize;

use crate::client::OpenBreweryClient;
use crate::errors::OpenBreweryResult;
use crate::search::BreweryListQueryOptions;

/// Represents a brewery registered within the Open Brewery DB API dataset.
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Brewery {
    /// The generated UUID of the brewery.
    pub id: String,
    /// The name of the brewery.
    pub name: String,
    /// Type of the brewery, may not be included for all breweries.
    pub brewery_type: Option<String>,
    /// Primary address of the brewery, optional.
    pub address_1: Option<String>,
    /// Secondary address of the brewery, optional.
    pub address_2: Option<String>,
    /// Additional address of the brewery, optional.
    pub address_3: Option<String>,
    /// City the brewery is located in.
    pub city: String,
    /// State the brewery is located in, primarily for non-U.S. breweries.
    pub state_province: String,
    /// U.S. State the brewery is located in.
    pub state: String,
    /// Zip code of the brewery.
    pub postal_code: String,
    /// Country the brewery is located in.
    pub country: String,
    /// Geographical longitude of the brewery, optional.
    pub longitude: Option<String>,
    /// Geographical latitude of the brewery, optional.
    pub latitude: Option<String>,
    /// Primary phone number of the brewery.
    pub phone: String,
    /// Website for the brewery, optional.
    pub website_url: Option<String>,
    /// Street address of the brewery, optional.
    pub street: Option<String>,
}

impl OpenBreweryClient {
    /// Retrieves a single brewery based on the UUID of the brewery.
    pub async fn find(&self, uuid: &str) -> OpenBreweryResult<Option<Brewery>> {
        self.send_request_and_optionally_deserialize::<Brewery>(uuid)
            .await
    }

    /// Retrieves one or more random breweries.
    pub async fn random(&self, size: Option<u32>) -> OpenBreweryResult<Vec<Brewery>> {
        self.send_request_and_deserialize::<Vec<Brewery>>(&format!(
            "random?size={}",
            size.unwrap_or(1)
        ))
        .await
    }

    /// Retrieves a list of breweries optionally based on available filters.
    pub async fn list(&self, filters: Option<BreweryListQueryOptions>) -> OpenBreweryResult<Vec<Brewery>> {
        self.send_request_and_deserialize::<Vec<Brewery>>("").await
    }
}
