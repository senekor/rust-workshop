use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SendRequest {
    #[schema(example = "A pair of socks")]
    pub content: String,
    #[schema(example = "Sophia")]
    pub recipient: Option<String>,
    #[serde(default)]
    pub express: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReceiveRequest {
    #[schema(example = "Sophia")]
    pub recipient: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ReceiveResponse {
    pub content: String,
}
