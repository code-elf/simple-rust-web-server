use actix_cors::Cors;
use actix_web::{get, post, put, web, App, HttpServer, Responder, HttpResponse};
use rand::Rng;
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Serialize, Deserialize};
use sha3::{Digest, Sha3_256};

mod auth;

#[derive(Serialize, Deserialize)]
struct Name {
	name: String
}

#[get("/name")]
async fn get_name(auth: auth::User, db: web::Data<Connection>) -> impl Responder {
	let name: String = db.query_row("SELECT username FROM users WHERE id = ?", params![id], |row| row.get(0)).unwrap();
	HttpResponse::Ok().json(Name { name })
}

#[put("/name")]
async fn put_name(db: web::Data<Connection>, data: web::Json<Name>) -> impl Responder {
	db.execute("UPDATE users SET username = ?", params![data.name]).unwrap();
	actix_web::HttpResponse::NoContent()
}

#[derive(Deserialize)]
struct UserData {
	username: String,
	password: String
}

#[derive(Serialize)]
struct AuthToken {
	token: String
}

fn hash_password(input: &String) -> Vec<u8> {
	let mut password = Sha3_256::default();
	password.update(input.as_bytes());
	password.finalize().to_vec()
}

fn create_token() -> String {
	rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(32).map(char::from).collect()
}

#[post("/login")]
async fn login(data: web::Json<UserData>, db: web::Data<Connection>) -> impl Responder {
	let password = hash_password(&data.password);
	let id: Option<u32> = db.query_row("SELECT id FROM users WHERE username = ? AND password = ?", params![&data.username, &password], |row| row.get(0)).optional().unwrap();
	if let Some(id) = id {
		let token = create_token();
		db.execute("UPDATE users SET token = ? WHERE id = ?", params![&token, id]).unwrap();
		HttpResponse::Ok().json(AuthToken { token })
	} else {
		HttpResponse::Forbidden().finish()
	}
}

#[post("/users")]
async fn register(data: web::Json<UserData>, db: web::Data<Connection>) -> impl Responder {
	let password = hash_password(&data.password);
	let token = create_token();
	db.execute("INSERT INTO users(username, password, token) VALUES(?, ?, ?)", params![data.username, &password, &token]).unwrap();
	web::Json(AuthToken { token })
}

#[derive(Deserialize)]
struct Config {
	port: u16
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let config: Config = serde_json::from_str(&std::fs::read_to_string(&"./config.json").unwrap()).unwrap();
	HttpServer::new(move || {
		let db = Connection::open("./server.db").unwrap();
		App::new()
			.wrap(Cors::permissive())
			.service(get_name)
			.service(put_name)
			.service(register)
			.service(login)
			.app_data(web::Data::new(db))
	}).bind(("127.0.0.1", config.port))?.run().await
}
