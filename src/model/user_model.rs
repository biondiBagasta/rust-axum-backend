use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;

use crate::model::utils_model::{ PaginationResponse };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserData {
	pub id: i32,
	pub username: String,
	pub password: String,
	pub full_name: String,
	pub address: String,
	pub phone_number: String,
	pub photo: String,
	pub role: String,
	#[serde(with = "time::serde::rfc3339")]
	pub created_at: OffsetDateTime,
	#[serde(with = "time::serde::rfc3339")]
	pub updated_at: OffsetDateTime
}

#[derive(Serialize)]
pub struct UserPaginate {
	pub data: Vec<UserData>,
	pub paginate: PaginationResponse
}

#[derive(Deserialize)]
pub struct UserCreateDto {
	pub username: String,
	pub password: String,
	pub full_name: String,
	pub address: String,
	pub phone_number: String,
	pub photo: String,
	pub role: String
}

#[derive(Deserialize)]
pub struct UserUpdateDto {
	pub username: Option<String>,
	pub password: Option<String>,
	pub full_name: Option<String>,
	pub address: Option<String>,
	pub phone_number: Option<String>,
	pub photo: Option<String>,
	pub role: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtClaims {
	pub user_data: UserData,
	pub exp: usize,
}
