use axum::{
	extract::{Path, State},
	http::StatusCode,
	Json
};

use sqlx::postgres::PgPool;
use serde_json::json;

use crate::model::category_model::{ CategoryCreateBody, CategoryData, CategoryUpdateBody, CategoryPaginate };
use crate::model::utils_model::{ PaginationBody, PaginationResponse };

pub async fn search_paginate(State(pg_pool): State<PgPool>,
	Json(body): Json<PaginationBody>
) -> Result<(StatusCode, String), (StatusCode, String)> {
	const PAGE_TAKE: i64 = 10;

	let query_count: i64 = sqlx::query_scalar(
		"SELECT COUNT(id) from category"
	).fetch_one(&pg_pool)
	.await
	.map_err(|e| {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			json!({ "success": false, "message": e.to_string() }).to_string()
		)
	})?;

	let query_search = sqlx::query_as!(
		CategoryData,
		"SELECT * FROM category WHERE name ILIKE $1 LIMIT($2) OFFSET($3)",
		format!("%{}%", body.term),
		PAGE_TAKE,
		(&body.page - 1) * PAGE_TAKE
	).fetch_all(&pg_pool)
	.await
	.map_err(|e| {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			json!({ "success": false, "message": e.to_string() }).to_string()
		)
	})?;

	let pagination_response = CategoryPaginate {
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


pub async fn find_many(State(pg_pool): State<PgPool>) -> Result<(StatusCode, String), (StatusCode, String)> {
	let query_find_many = sqlx::query_as!(
		CategoryData, 
		"SELECT * FROM category ORDER BY name ASC"
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
		json!({ "success": true, "data": query_find_many }).to_string()
	))
}

pub async fn create(
    State(pg_pool): State<PgPool>,
    Json(body): Json<CategoryCreateBody>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query_as!(CategoryData, 
    	"INSERT INTO category (name) VALUES($1) RETURNING *",
        body.name,
    ).fetch_one(&pg_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string()
        )
    })?;

    Ok((
        StatusCode::CREATED,
        // json!({ "success": true, "data": query_insert, "message": "" }).to_string()
        json!({ "success": true, "message": "Data Category Berhasil Ditambahkan" }).to_string()
    ))
}

pub async fn update(
	State(pg_pool): State<PgPool>,
	Path(id): Path<i32>,
	Json(body): Json<CategoryUpdateBody>
) -> Result<(StatusCode, String), (StatusCode, String)> {
	sqlx::query!(
		"UPDATE category set name = $1 WHERE id = $2",
		body.name,
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
		json!({ "success": true, "message": "Data Category Berhasil Diupdate." }).to_string()
	))
}

pub async fn delete(
	State(pg_pool): State<PgPool>,
	Path(id): Path<i32>
) -> Result<(StatusCode, String), (StatusCode, String)> {
	sqlx::query!(
		"DELETE from category WHERE id = $1",
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
		json!({ "success": true, "message": "Data Category Berhasil Dihapus." }).to_string()
	))
}
