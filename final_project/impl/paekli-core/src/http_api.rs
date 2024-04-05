use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SendRequest {
    #[schema(example = "A pair of socks")]
    pub content: String,
    #[schema(example = "Sophia")]
    pub receiver: Option<String>,
    #[serde(default)]
    pub express: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReceiveRequest {
    #[schema(example = "Sophia")]
    pub receiver: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ReceiveResponse {
    pub content: String,
}
