use reqwest;
use serde_json;

#[derive(Default)]
pub struct AnkiClient {
    client: reqwest::Client,
    url: &'static str,
}

impl AnkiClient {
    pub fn new(url: &'static str) -> Self {
        AnkiClient {
            client: reqwest::Client::new(),
            url,
        }
    }
    
    pub async fn create_note_template(&self, template: &serde_json::Value) -> Result<(), reqwest::Error> {
        let res = self.client.post(self.url)
            .json(&template)
            .send()
            .await?;

        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());
        let body = res.text().await?;
        println!("Body:\n{}", body);
        
        Ok(())
    }
}