use openbrewerydb::client::OpenBreweryClient;
use openbrewerydb::errors::OpenBreweryResult;

#[tokio::main]
async fn main() -> OpenBreweryResult<()> {
    let client = OpenBreweryClient::new();
    let breweries = client.find("b54b16e1-ac3b-4bff-a11f-f7ae9ddc27e0").await?;
    dbg!(breweries);

    Ok(())
}
