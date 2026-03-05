pub mod get {
    pub async fn fetch_data(url : &str) -> Result<String, reqwest::Error> {
        let body = reqwest::get(url)
            .await?
            .text()
            .await?;
        Ok(body)
    }
}