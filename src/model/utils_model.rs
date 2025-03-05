use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PaginationBody {
	pub term: String,
	pub page: i64
}

#[derive(Deserialize)]
pub struct PaginationResponse {
	pub per_page: i32,
	pub total_page: i32,
	pub count: i64,
	pub current_page: i32
}