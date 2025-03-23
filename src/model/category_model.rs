use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use crate::model::utils_model::PaginationResponse;

#[derive(Serialize)]
pub struct CategoryData {
	pub id: i32,
	pub name: String,
	#[serde(with = "time::serde::rfc3339")]
	pub created_at: OffsetDateTime,
	#[serde(with = "time::serde::rfc3339")]
	pub updated_at: OffsetDateTime
}

#[derive(Serialize)]
pub struct CategoryPaginate {
	pub data: Vec<CategoryData>,
	pub paginate: PaginationResponse
}

#[derive(Deserialize)]
pub struct CategoryCreateBody {
	pub name: String
} 

#[derive(Deserialize)]
pub struct CategoryUpdateBody {
	pub name: Option<String>
}

#[derive(Serialize)]
pub struct ReturningId {
    pub id: i32
}
