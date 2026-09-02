use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

/// How long to wait for the research API before giving up. Without this the
/// client waits forever on a server that accepts the connection and then
/// stalls, while the spinner keeps signalling progress that is not happening.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

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
    pub chunk_index: Option<usize>,
}

/// What went wrong talking to the research API.
///
/// The point of this type is that the HTTP status is inspected *before* the
/// body is parsed. Deserializing first turns every 401, 404 and 500 -- and
/// every HTML error page from the hosting layer -- into "error decoding
/// response body", which sends the reader looking in entirely the wrong place.
#[derive(Debug)]
pub enum ApiError {
    /// The request never completed: DNS, TLS, connection refused.
    Transport(reqwest::Error),
    /// The server did not answer within `REQUEST_TIMEOUT`.
    Timeout,
    /// 401 or 403 -- the API key is missing, wrong, or lacks access.
    Unauthorized,
    /// Any other non-success status, with the body when the server sent one.
    Http {
        status: reqwest::StatusCode,
        body: Option<String>,
    },
    /// The status was a success, but the body was not the JSON we expect.
    Decode(reqwest::Error),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Transport(e) => write!(f, "could not reach the API: {e}"),
            ApiError::Timeout => write!(
                f,
                "the API did not respond within {} seconds",
                REQUEST_TIMEOUT.as_secs()
            ),
            ApiError::Unauthorized => write!(
                f,
                "not authorized. Set RESEARCH_API_KEY, or check that the key is valid"
            ),
            ApiError::Http { status, body } => match body {
                Some(b) if !b.trim().is_empty() => {
                    write!(f, "the API returned {status}: {}", flatten(b, 200))
                }
                _ => write!(f, "the API returned {status}"),
            },
            ApiError::Decode(e) => {
                write!(f, "the API returned data in an unexpected shape: {e}")
            }
        }
    }
}

impl std::error::Error for ApiError {}

impl From<reqwest::Error> for ApiError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            ApiError::Timeout
        } else if e.is_decode() {
            ApiError::Decode(e)
        } else {
            ApiError::Transport(e)
        }
    }
}

/// Collapse an error body to a single line and cap its length, so a full HTML
/// page does not spill into the terminal.
fn flatten(s: &str, max: usize) -> String {
    let flat = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() <= max {
        flat
    } else {
        let kept: String = flat.chars().take(max).collect();
        format!("{kept}…")
    }
}

pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: String, api_key: Option<String>) -> Self {
        let client = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
            // Only fails if the TLS backend cannot be initialised, which is not
            // something the caller can act on; the default client is no better.
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            client,
            base_url,
            api_key,
        }
    }

    fn endpoint(&self) -> String {
        format!("{}/api/research", self.base_url.trim_end_matches('/'))
    }

    fn add_auth_headers(&self, mut req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(ref key) = self.api_key {
            req = req.header("Authorization", format!("Bearer {key}"));
        }
        req
    }

    /// Send a prepared request, classify the status, and only then parse.
    async fn send<T: serde::de::DeserializeOwned>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<T, ApiError> {
        let response = self.add_auth_headers(req).send().await?;
        let status = response.status();

        if !status.is_success() {
            if status == reqwest::StatusCode::UNAUTHORIZED
                || status == reqwest::StatusCode::FORBIDDEN
            {
                return Err(ApiError::Unauthorized);
            }
            // A failed body read is not worth masking the status we already have.
            let body = response.text().await.ok();
            return Err(ApiError::Http { status, body });
        }

        response.json().await.map_err(ApiError::from)
    }

    pub async fn get_metadata(&self) -> Result<MetadataResponse, ApiError> {
        self.send(self.client.get(self.endpoint())).await
    }

    /// Semantic similarity search.
    pub async fn search(&self, query: &str, limit: usize) -> Result<SearchResponse, ApiError> {
        self.query("search", query, limit).await
    }

    /// Exact keyword (full-text) search.
    pub async fn keyword_search(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<SearchResponse, ApiError> {
        self.query("keyword", query, limit).await
    }

    async fn query(
        &self,
        action: &str,
        query: &str,
        limit: usize,
    ) -> Result<SearchResponse, ApiError> {
        let payload = SearchRequest {
            action: action.to_string(),
            query: query.to_string(),
            limit,
        };
        self.send(self.client.post(self.endpoint()).json(&payload))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_tolerates_a_trailing_slash() {
        let with = ApiClient::new("https://example.com/".to_string(), None);
        let without = ApiClient::new("https://example.com".to_string(), None);
        assert_eq!(with.endpoint(), "https://example.com/api/research");
        assert_eq!(with.endpoint(), without.endpoint());
    }

    #[test]
    fn unauthorized_names_the_environment_variable() {
        // The whole point of the variant: the reader is told what to do next.
        let msg = ApiError::Unauthorized.to_string();
        assert!(msg.contains("RESEARCH_API_KEY"), "got: {msg}");
    }

    #[test]
    fn http_error_reports_the_status() {
        let msg = ApiError::Http {
            status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            body: None,
        }
        .to_string();
        assert!(msg.contains("500"), "got: {msg}");
    }

    #[test]
    fn http_error_flattens_and_truncates_an_html_body() {
        let body = format!("<html>\n  <body>{}</body>\n</html>", "x".repeat(500));
        let msg = ApiError::Http {
            status: reqwest::StatusCode::BAD_GATEWAY,
            body: Some(body),
        }
        .to_string();
        assert!(!msg.contains('\n'), "error should stay on one line: {msg}");
        assert!(msg.ends_with('…'), "long body should be truncated: {msg}");
    }

    #[test]
    fn timeout_names_the_limit() {
        let msg = ApiError::Timeout.to_string();
        assert!(
            msg.contains(&REQUEST_TIMEOUT.as_secs().to_string()),
            "got: {msg}"
        );
    }
}
