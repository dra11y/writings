use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Pagination {
    pub limit: usize,
    pub offset: usize,
    pub total: usize,
}
