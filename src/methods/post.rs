pub mod post {
    pub async fn post_data(url: &str, body: &str) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        
        let result = client.post(url)
            .header("Content-Type", "application/json") // Common for APIs
            .body(body.to_string())
            .send()
            .await?;

        let result = result.error_for_status()?;

        let text = result.text().await?;
        Ok(text)
    }
}