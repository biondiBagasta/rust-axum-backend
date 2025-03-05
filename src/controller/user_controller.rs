use axum:: {
	extract::{ Path, State },
	http::StatusCode,
	Json
};

use sqlx::postgres::PgPool;
use serde_json::json;

#[path = "../model/user_model.rs"] mod user_model;
#[path = "../model/utils_model.rs"] mod utils_model;

pub async fn search_paginate(
	State(pg_pool): State<PgPool>,
	Json(body): Json<utils_model::PaginationBody>
) -> Result<(StatusCode, String), (StatusCode, String)> {
	let query_search = sqlx::query_as!(
		user_model::UserData,
		"SELECT * FROM user_system WHERE username ILIKE $1 OR full_name ILIKE $2
		LIMIT($3) OFFSET($4)",
		format!("%{}%", body.term),
		format!("%{}%", body.term),
		10,
		(body.page - 1) * 10
	).fetch_all(&pg_pool)
	.await
	.map_err(|e| {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			json!({ "success": false, "message": e.to_string()}).to_string()
		)
	})?;

	Ok((
		StatusCode::OK,
		json!({ "success": true, "data": query_search }).to_string()
	))
}

