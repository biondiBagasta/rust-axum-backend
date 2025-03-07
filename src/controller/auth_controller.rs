use axum:: {
	extract::State,
	http:: {HeaderMap, StatusCode},
	Json
};

use sqlx::{postgres::PgPool, query, query_as};
use serde_json::json;

use bcrypt::{hash, verify, DEFAULT_COST};

use jsonwebtoken::{ decode, encode, DecodingKey, EncodingKey, Header, Validation };

use crate::model::auth_model::{ LoginBody, ChangePasswordBody };
use crate::model::user_model::{ UserData, JwtClaims };

use crate::utils::utils::JWT_SECRET;

use std::time::{SystemTime, UNIX_EPOCH};

pub async fn login(
	State(pg_pool): State<PgPool>,
	Json(body): Json<LoginBody>
) -> Result<(StatusCode, String), (StatusCode, String)> {
	let query_find_first = sqlx::query_as!(
		UserData,
		"SELECT * FROM user_system WHERE username = $1 LIMIT 1",
		body.username
	).fetch_one(&pg_pool)
	.await
	.map_err(|e| {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			json!({ "success": false, "message": e.to_string() }).to_string()
		)
	})?;

	let compared_password = verify(&body.password, &query_find_first.password);

	if compared_password.unwrap() == false {
		Ok(
			(
				StatusCode::UNAUTHORIZED,
				json!({ "success": false, "message": "Data User Tidak Ditemukan." }).to_string()
			)
		)
	} else {
	    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

        let user_data_clone = query_find_first.clone();

        let jwt_claim = JwtClaims {
        	user_data: UserData {
        		password: String::from(""),
        		..user_data_clone
        	},
        	exp: (now + 360000) as usize
        };

		let jwt_token = encode(&Header::default(), &jwt_claim, &EncodingKey::from_secret(JWT_SECRET.as_ref()))
		.expect("Failed to Create Token");

		Ok((
			StatusCode::ACCEPTED,
			json!({ "success": true, "data": query_find_first, "token": jwt_token }).to_string()
		))
	}
}

pub async fn authenticated(
	headers: HeaderMap
) -> Result<(StatusCode, String), (StatusCode, String)> {
	if let Some(auth_header) = headers.get("Authorization") {
		if let Ok(header_value) = auth_header.to_str() {
			
			if let Some(jwt_token) = header_value.strip_prefix("Bearer ") {

		       let decoded = decode::<JwtClaims>(
			        jwt_token,
			        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
			        &Validation::default(),
			    ).unwrap().claims;

				Ok((
					StatusCode::OK,
					json!({ "status": true,  "data": decoded }).to_string()
				))
			} else {
				Ok((
					StatusCode::UNAUTHORIZED,
					json!({ "status": false, "message": "Failed to Strip Bearer Prefix" }).to_string()
				))
			}

		} else {
			Ok((
				StatusCode::UNAUTHORIZED,
				json!({ "status": false, "message": "Invalid Credentials" }).to_string()
			))
		}
	} else {
		Ok((
			StatusCode::UNAUTHORIZED,
			json!({ "status": false, "message": "Invalid Credentials" }).to_string()
		))
	}
}

pub async fn change_password(
	State(pg_pool): State<PgPool>,
	Json(body): Json<ChangePasswordBody>
) -> Result<(StatusCode, String), (StatusCode, String)> {
	let query_find_first = query_as!(
		UserData,
		"SELECT * FROM user_system WHERE username = $1",
		body.username
	).fetch_one(&pg_pool)
	.await
	.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "success": false, "message": e.to_string() }).to_string()
        )
	})?;

	let compare_password = verify(body.old_password, &query_find_first.password);

	match compare_password {
	    Ok(value) => {
	    	if value == true {
	    		let new_password = hash(body.new_password, DEFAULT_COST).unwrap();

	    		query!(
	    			"UPDATE user_system SET password = $1 WHERE username = $2",
	    			new_password,
	    			body.username
	    		).execute(&pg_pool)
	    		.await
	    		.map_err(|e| {
			        (
			            StatusCode::INTERNAL_SERVER_ERROR,
			            json!({ "success": false, "message": e.to_string() }).to_string()
			        )
	    		})?;

		        Ok(
		        	(
			            StatusCode::OK,
			            json!({ "success": true, "message": "Password Anda Berhasil Diperbaharui" }).to_string()
			        )
		        )
	    	} else {
	    		Ok(
	    			(
			            StatusCode::FORBIDDEN,
			            json!({ "success": false, "message": "Password Lama Salah." }).to_string()
			        )
	    		)	
	    	}
	    },
	    Err(_) => {
	        Ok(
	        	(
	            	StatusCode::FORBIDDEN,
		            json!({ "success": false, "message": "Terjadi Kesalahan Ketika Mengupdate Password. Silahkan Coba Lagi." }).to_string()
		        )
	        )	
	    }
	}
}