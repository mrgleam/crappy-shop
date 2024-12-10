use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// The view for a ProductView query, for a standard http application this should
// be designed to reflect the response dto that will be returned to a product.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProductView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub price: f32,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}
