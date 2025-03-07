use axum:: {
	extract::{ Path, State },
	http::StatusCode,
	Json
};

use sqlx::postgres::PgPool;
use serde_json::json;

use bcrypt::{ DEFAULT_COST, hash, verify };

use crate::model::user_model::{ UserCreateDto, UserUpdateDto, UserData, UserPaginate };
use crate::model::utils_model::{ PaginationBody, PaginationResponse };

pub async fn search_paginate(
	State(pg_pool): State<PgPool>,
	Json(body): Json<PaginationBody>
) -> Result<(StatusCode, String), (StatusCode, String)> {

	const PAGE_TAKE: i64 = 10;

	let query_count: i64 = sqlx::query_scalar(
		"SELECT COUNT(id) from user_system"
	).fetch_one(&pg_pool)
	.await
	.map_err(|e| {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			json!({ "success": false, "message": e.to_string() }).to_string()
		)
	})?;

	let query_search = sqlx::query_as!(
		UserData,
		"SELECT * FROM user_system WHERE username ILIKE $1 OR full_name ILIKE $2
		LIMIT($3) OFFSET($4)",
		format!("%{}%", body.term),
		format!("%{}%", body.term),
		PAGE_TAKE,
		(&body.page - 1) * PAGE_TAKE
	).fetch_all(&pg_pool)
	.await
	.map_err(|e| {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			json!({ "success": false, "message": e.to_string()}).to_string()
		)
	})?;

	let pagination_response = UserPaginate {
		data: query_search,
		paginate: PaginationResponse {
			per_page: PAGE_TAKE,
			total_page: ((query_count as f64 / PAGE_TAKE as f64) * 0.4).round() as i64,
			count: query_count,
			current_page: body.page
		}
	};

	Ok((
		StatusCode::OK,
		json!({ "success": true, "data": pagination_response.data, "paginate": pagination_response.paginate }).to_string()
	))
}

pub async fn create(
	State(pg_pool): State<PgPool>,
	Json(body): Json<UserCreateDto>
) -> Result<(StatusCode, String), (StatusCode, String)> {
	
	let hashed_password = hash(body.password, DEFAULT_COST).unwrap();

	sqlx::query!(
		"INSERT INTO user_system (username, password, full_name, address, phone_number, role, photo) VALUES ($1, $2, $3, $4, $5, $6, $7)",
		body.username,
		hashed_password,
		body.full_name,
		body.address,
		body.phone_number,
		body.role,
		body.photo
	).execute(&pg_pool)
	.await
	.map_err(|e| {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			json!({ "success": false, "message": e.to_string() }).to_string()
		)
	})?;

	Ok((
		StatusCode::CREATED,
		json!({ "success": true, "message": format!("Data User {} berhasil ditambahkan.", body.full_name) }).to_string()
	))
}

pub async fn update(
	State(pg_pool): State<PgPool>,
	Path(id): Path<i32>,
	Json(body): Json<UserUpdateDto>
) -> Result<(StatusCode, String), (StatusCode, String)> {

	if body.password.is_none() {
		sqlx::query!(
			"UPDATE user_system SET username = $1, full_name = $2, address = $3, phone_number = $4, role = $5, photo = $6 WHERE 
			id = $7",
			body.username,
			body.full_name,
			body.address,
			body.phone_number,
			body.role,
			body.photo,
			id
		).execute(&pg_pool)
		.await
		.map_err(|e| {
			(
				StatusCode::INTERNAL_SERVER_ERROR,
				json!({ "success": false, "message": e.to_string() }).to_string()
			)
		})?;

		Ok((
			StatusCode::OK,
			json!({ "success": true, "message": "Data User berhasil diperbaharui."}).to_string()
		))

	} else {
		let query_find_first = sqlx::query_as!(
			UserData,
			"SELECT * FROM user_system WHERE id = $1 LIMIT 1",
			id
		).fetch_one(&pg_pool)
		.await
		.map_err(|e| {
			(
				StatusCode::INTERNAL_SERVER_ERROR,
				json!({ "success": false, "message": e.to_string() }).to_string()
			)
		})?;

		let compare_password = verify(&body.password.as_ref().unwrap(), &query_find_first.password);

		if compare_password.unwrap() == true {
			sqlx::query!(
				"UPDATE user_system SET username = $1, full_name = $2, address = $3, phone_number = $4, role = $5, photo = $6 WHERE 
				id = $7",
				body.username,
				body.full_name,
				body.address,
				body.phone_number,
				body.role,
				body.photo,
				id
			).execute(&pg_pool)
			.await
			.map_err(|e| {
				(
					StatusCode::INTERNAL_SERVER_ERROR,
					json!({ "success": false, "message": e.to_string() }).to_string()
				)
			})?;

			Ok((
				StatusCode::OK,
				json!({ "success": true, "message": "Data User berhasil diperbaharui."}).to_string()
			))
		} else {
			sqlx::query!(
			"UPDATE user_system SET username = $1, password = $8, full_name = $2, address = $3, phone_number = $4, role = $5, photo = $6 WHERE 
				id = $7",
				body.username,
				body.full_name,
				body.address,
				body.phone_number,
				body.role,
				body.photo,
				id,
				&body.password.unwrap()
			).execute(&pg_pool)
			.await
			.map_err(|e| {
				(
					StatusCode::INTERNAL_SERVER_ERROR,
					json!({ "success": false, "message": e.to_string() }).to_string()
				)
			})?;

			Ok((
				StatusCode::OK,
				json!({ "success": true, "message": "Data User berhasil diperbaharui."}).to_string()
			))
		}
	}
}

pub async fn delete(
	State(pg_pool): State<PgPool>,
	Path(id): Path<i32>
) -> Result<(StatusCode, String), (StatusCode, String)> {
	sqlx::query!(
		"DELETE FROM user_system WHERE id = $1",
		id
	).execute(&pg_pool)
	.await
	.map_err(|e| {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			json!({ "success": false, "message": e.to_string() }).to_string()
		)
	})?;

	Ok((
		StatusCode::OK,
		json!({ "success": true, "message": "Data User berhasil dihapus." }).to_string()
	))
}

