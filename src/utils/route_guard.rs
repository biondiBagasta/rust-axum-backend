use axum:: {
	body:: { Body },
	http:: { StatusCode, Request }, 
	middleware::Next,
	response::Response,
};

use jsonwebtoken::{ decode, DecodingKey, Validation };
use serde_json::json;
use crate::model::user_model::JwtClaims;
use crate::utils::utils::JWT_SECRET;

pub async fn auth_guard(req: Request<Body>, next: Next) -> Result<Response, (StatusCode, String)> {

	if let Some(extracted_header_value) = req.headers().get("Authorization") {
		if let Ok(header_value) = extracted_header_value.to_str() {
			if let Some(jwt_token) = header_value.strip_prefix("Bearer ") {

				let decoded_token = decode::<JwtClaims>(
					jwt_token, 
					&DecodingKey::from_secret(JWT_SECRET.as_ref()), 
					&Validation::default()
				);

				match decoded_token {
					Ok(_) => Ok(next.run(req).await),
					Err(e) => Err((
						StatusCode::UNAUTHORIZED,
						json!({ "success": false, "message": e.to_string() }).to_string()
					))
				}
			} else {
				Err((
					StatusCode::UNAUTHORIZED,
					json!({ "success": false, "message": "Invalid Credentials" }).to_string()
				))	
			}
		} else {
			Err((
				StatusCode::UNAUTHORIZED,
				json!({ "success": false, "message": "Invalid Credentials" }).to_string()
			))	
		}
	} else {
		Err((
			StatusCode::UNAUTHORIZED,
			json!({ "success": false, "message": "Invalid Credentials" }).to_string()
		))	
	}
}