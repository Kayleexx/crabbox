use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateUrlRequest {
    pub original: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Url {
    pub original: String,
    pub short: String,
}
