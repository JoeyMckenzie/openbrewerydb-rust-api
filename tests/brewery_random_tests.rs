use openbrewerydb::client::OpenBreweryClient;

#[tokio::test]
async fn returns_ok_with_single_brewery() {
    // Arrange
    let client = OpenBreweryClient::new();

    // Act
    let brewery = client.random(None).await;

    // Assert
    assert!(brewery.is_ok());
    assert_eq!(brewery.unwrap().len(), 1);
}

#[tokio::test]
async fn returns_ok_with_multiple_breweries_when_given_size() {
    // Arrange
    let client = OpenBreweryClient::new();

    // Act
    let brewery = client.random(Some(5)).await;

    // Assert
    assert!(brewery.is_ok());
    assert_eq!(brewery.unwrap().len(), 5);
}
