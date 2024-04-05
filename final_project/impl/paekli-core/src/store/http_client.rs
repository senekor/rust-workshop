use reqwest::blocking as reqwest;

use crate::http_api;

use super::DistributionCenter;

pub struct HttpClient;

impl DistributionCenter for HttpClient {
    fn store(&self, recipient: String, content: String, express: bool) {
        let client = reqwest::Client::new();
        client
            .post("https://paekli.buenzli.dev/paekli")
            .json(&http_api::SendRequest {
                content,
                recipient: Some(recipient),
                express,
            })
            .send()
            .unwrap();
    }

    fn retrieve(&self, recipient: String) -> Option<String> {
        let client = reqwest::Client::new();
        let resp = client
            .delete("https://paekli.buenzli.dev/paekli")
            .json(&http_api::ReceiveRequest {
                recipient,
            })
            .send()
            .unwrap()
            .json::<http_api::ReceiveResponse>()
            .unwrap();
        Some(resp.content)
    }
}
