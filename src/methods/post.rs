pub mod post {
    pub async fn post_data(url: &str, body: &str) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        
        let result = client.post(url)
            .header("Content-Type", "application/json") // Common for APIs
            .body(body.to_string())
            .send()
            .await?;

        // This is the "Magic Line": 
        // If status is 404, 401, or 500, it converts to an Error immediately.
        let result = result.error_for_status()?;

        let text = result.text().await?;
        Ok(text)
    }
}