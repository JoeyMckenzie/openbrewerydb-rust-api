use reqwest::header::HeaderMap;

/// Various query options to refine searching and listing breweries.
#[derive(Debug)]
pub struct BreweryListQueryOptions {
    ///  City breweries are located in.
    pub by_city: Option<String>,
    /// A list of UUIDs to search for.
    pub by_ids: Option<Vec<String>>,
    /// Name of breweries, used as a needle in the haystack.
    pub by_name: Option<String>,
    /// U.S. state breweries are located in.
    pub by_state: Option<String>,
    /// Zip code breweries are located in.
    pub by_postal: Option<String>,
    /// Type of breweries.
    pub by_type: Option<String>,
    /// Distance from the brewery, based on the latitude and longitude.
    pub by_dist: Option<String>,
    /// Page of brewery results.
    pub page: Option<String>,
    /// Amount of breweries to return from each page.
    pub per_page: Option<String>,
    /// Sort order of the breweries, used in conjunction with the search fields.
    pub sort: Option<String>,
}

/// A fluent builder for adding list options.
#[derive(Debug)]
pub struct BreweryListQueryOptionsBuilder {
    query: Option<BreweryListQueryOptions>,
}

impl BreweryListQueryOptionsBuilder {
    pub fn new() -> Self {
        Self { query: None }
    }
}

impl Default for BreweryListQueryOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BreweryListQueryOptionsBuilder {
    pub fn with_city(self, city: String) -> Self {
        if let Some(current_query) = self.query {
            let options = BreweryListQueryOptions {
                by_city: Some(city),
                ..current_query
            };

            return Self {
                query: Some(options),
            };
        }

        self
    }

    pub fn with_state(self, state: String) -> Self {
        if let Some(current_query) = self.query {
            let options = BreweryListQueryOptions {
                by_state: Some(state),
                ..current_query
            };

            return Self {
                query: Some(options),
            };
        }

        self
    }

    /// Consumes the options struct returning a query parameter mapping used on the request.
    pub(crate) fn build(self) {
        let test = [("foo", "a"), ("foo", "b")];
        let mut query_params: Vec<(&str, &str)> = Vec::new();

        if let Some(filters) = self.query {
            if let Some(city) = filters.by_city {
                query_params.push(("by_city", &city));
            }
        }

        let query = reqwest::RequestBuilder::query(&query_params);
    }
}
