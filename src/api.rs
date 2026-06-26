use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetadataResponse {
    pub collection: String,
    pub status: String,
    pub vectors_count: Option<u64>,
    pub points_count: u64,
    pub vector_size: usize,
    pub distance: String,
    pub embedding_model: String,
    pub payload_schema: HashMap<String, String>,
}

#[derive(Serialize, Debug)]
pub struct SearchRequest {
    pub action: String,
    pub query: String,
    pub limit: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub score: f64,
    pub payload: ChunkPayload,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChunkPayload {
    pub text: String,
    pub filename: String,
    pub chunk_index: usize,
}

pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: String, api_key: Option<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
            api_key,
        }
    }

    fn add_auth_headers(&self, mut req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(ref key) = self.api_key {
            req = req.header("Authorization", format!("Bearer {}", key));
        }
        req
    }

    pub async fn get_metadata(&self) -> Result<MetadataResponse, reqwest::Error> {
        let url = format!("{}/api/research", self.base_url.trim_end_matches('/'));
        let mut req = self.client.get(&url);
        req = self.add_auth_headers(req);
        req.send().await?.json().await
    }

    pub async fn search(&self, query: &str, limit: usize) -> Result<SearchResponse, reqwest::Error> {
        let url = format!("{}/api/research", self.base_url.trim_end_matches('/'));
        let payload = SearchRequest {
            action: "search".to_string(),
            query: query.to_string(),
            limit,
        };
        let mut req = self.client.post(&url).json(&payload);
        req = self.add_auth_headers(req);
        req.send().await?.json().await
    }
}
