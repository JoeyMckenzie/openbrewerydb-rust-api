mod brewery_find_tests {
    use openbrewerydb::client::OpenBreweryClient;

    #[tokio::test]
    async fn returns_ok_when_brewery_exists() {
        // Arrange
        let client = OpenBreweryClient::new();

        // Act
        let brewery = client.find("b54b16e1-ac3b-4bff-a11f-f7ae9ddc27e0").await;

        // Assert
        assert!(brewery.is_ok());
        assert!(brewery.unwrap().is_some())
    }

    #[tokio::test]
    async fn returns_none_when_brewery_does_not_exist() {
        // Arrange
        let client = OpenBreweryClient::new();

        // Act
        let brewery = client.find("does-not-exist").await;

        // Assert
        assert!(brewery.is_ok());
        assert!(brewery.unwrap().is_none());
    }
}
