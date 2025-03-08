use axum:: {
	body:: { Body },
	http:: { StatusCode, Request }, 
	middleware::Next,
	response::Response,
};

use jsonwebtoken::{ decode, DecodingKey, Validation };
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
						e.to_string()
					))
				}
			} else {
				Err((
					StatusCode::UNAUTHORIZED,
					"You aren't authorized.".to_string()
				))	
			}
		} else {
			Err((
				StatusCode::UNAUTHORIZED,
				"You aren't authorized.".to_string()
			))
		}
	} else {
		Err((
			StatusCode::UNAUTHORIZED,
			"You aren't authorized.".to_string()
		))
	}
}