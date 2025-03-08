use std::collections::HashMap;

use axum::{
	http::StatusCode
};

use serde_json::json;

use crate::utils::utils::CLIENT;

pub async fn get_http_example() -> Result<(StatusCode, String), (StatusCode, String)> {

	let response = CLIENT.get("https://jsonplaceholder.typicode.com/posts")
	.send()
	.await
	.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let json_response = response.json::<serde_json::Value>() // Bisa di Replace Value dengan Struct
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok((
        StatusCode::OK,
        json!({
            "success": true,
            "data": json_response
        }).to_string()
    ))
}

pub async fn post_http_example() -> Result<(StatusCode, String), (StatusCode, String)> {

	let mut map_body = HashMap::new();

	map_body.insert("body", json!("Hi From Rust"));
	map_body.insert("id", json!(100));
	map_body.insert("title", json!("Belajar Rust HTTP"));
	map_body.insert("userId", json!(100));

	let response = CLIENT.post("https://jsonplaceholder.typicode.com/todos")
	.json(&map_body)
	.send()
	.await
	.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

	let json_response = response.json::<serde_json::Value>()
	.await
	.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((
        StatusCode::OK,
        json!({
            "success": true,
            "data": json_response
        }).to_string()
    ))
}