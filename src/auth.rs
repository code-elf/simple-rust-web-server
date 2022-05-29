use std::fmt::{Display, Formatter};
use actix_web::{FromRequest, HttpRequest, Error, web};
use actix_web::dev::Payload;
use rusqlite::{Connection, OptionalExtension, params};

pub struct User {
	pub id: u32
}

#[derive(Debug)]
struct AuthError {
}

impl Display for AuthError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Invalid authentication token.")
	}
}

impl FromRequest for User {
	type Error = Error;
	type Future = futures_util::future::Ready<Result<Self, Self::Error>>;
	
	fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
		let db: &web::Data<Connection> = req.app_data().unwrap();
		let id: Option<u32> = req.headers().get("Authorization").and_then(|header| header.to_str().ok()).and_then(|token| {
			db.query_row("SELECT id FROM users WHERE token = ?", params![token], |row| row.get(0)).optional().unwrap()
		});
		if let Some(id) = id {
			futures_util::future::ready(Ok(User { id }))
		} else {
			futures_util::future::ready(Err(actix_web::error::ErrorForbidden(AuthError {})))
		}
	}
}
