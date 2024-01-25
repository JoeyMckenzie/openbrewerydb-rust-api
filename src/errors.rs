//! API errors that can occur during at any point during the request cycle to Open Brewery, mappings, builders, etc.

use thiserror::Error;

/// Wrapped result type useful for marshalling between library and dependencies errors.
pub type OpenBreweryResult<T> = Result<T, OpenBreweryError>;

/// Errors that can occur within the client, including mapped errors from reqwest.
#[derive(Debug, Error)]
pub enum OpenBreweryError {
    /// Represents an error occurring when attempting to build with a configured client.
    #[error("A client instance was unable to be constructed.")]
    ClientInstanceRequired,
    /// Represents any reqwest that has failed, propagating the error context.
    #[error("{0}")]
    ClientRequestFailed(#[from] reqwest::Error),
    /// Represents an error occurring when attempting construct queries.
    #[error("Query struct was not configured with any optional filters.")]
    SearchParametersNotProvided,
}
