use axum:: {
	http:: { HeaderMap, StatusCode }
}


pub fn use_auth_guard(headers: HeaderMap) {
	if let Some(authorization_header) = headers.get("Authorization") {
		
	} else {
		return Err((
			StatusCode::UNAUTHORIZED,
			"You aren't authorized."
		))
	}
}