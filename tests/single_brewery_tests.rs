mod single_brewery_tests {
    use openbrewerydb::client::OpenBreweryClient;

    #[tokio::test]
    async fn returns_ok_when_brewery_exists() {
        // Arrange
        let client = OpenBreweryClient::new();

        // Act
        let brewery = client
            .get_brewery("b54b16e1-ac3b-4bff-a11f-f7ae9ddc27e0")
            .await;
        dbg!(&brewery);

        // Assert
        assert!(brewery.is_ok());
    }
}
