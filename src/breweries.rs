use serde::Deserialize;

use crate::client::OpenBreweryClient;
use crate::errors::OpenBreweryResult;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Brewery {
    pub id: String,
    pub name: String,
    pub brewery_type: String,
    pub address_1: String,
    pub address_2: Option<String>,
    pub address_3: Option<String>,
    pub city: String,
    pub state_province: String,
    pub postal_code: String,
    pub country: String,
    pub longitude: String,
    pub latitude: String,
    pub phone: String,
    pub website_url: String,
    pub state: String,
    pub street: String,
}

impl OpenBreweryClient {
    /// Retrieves a single brewery based on the UUID of the brewery.
    pub async fn get_brewery(&self, uuid: &str) -> OpenBreweryResult<Brewery> {
        self.send_request_and_deserialize::<Brewery>(uuid).await
    }
}
