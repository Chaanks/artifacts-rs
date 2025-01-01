pub mod api;

#[cfg(test)]
mod tests {
    use crate::api::Api;

    #[tokio::test]
    async fn test_status() {
        let tokens = "TOKEN";
        let api = Api::new(tokens.to_string());
        let result = api.status().await;
        assert!(result.is_ok());
    }
}
