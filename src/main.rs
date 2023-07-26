use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::json::{json, Value};
use rocket::State;
use rocket::{Request, Response};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{MySql, Pool, Row};

#[macro_use]
extern crate rocket;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/messages")]
async fn get_messages(pool: &State<Pool<MySql>>) -> Value {
    match sqlx::query("SELECT * FROM messages")
        .fetch_all(pool.inner())
        .await
    {
        Ok(rows) => {
            let messages: Vec<Value> = rows
                .into_iter()
                .map(|row| {
                    let id: i32 = row.get("id");
                    let message: String = row.get("message");
                    let message_time: DateTime<Utc> = row.get("message_time");
                    json!({
                        "id": id,
                        "message": message,
                        "message_time": message_time.to_rfc3339(), // Convert DateTime to RFC 3339 format
                    })
                })
                .collect();
            json!({
                "messages": messages
            })
        }
        Err(e) => {
            panic!("Error querying the database: {}", e);
        }
    }
}

#[post("/<user_id>", data = "<message>")]
async fn insert_message(message: String, user_id: i32, pool: &State<Pool<MySql>>) {
    match sqlx::query("INSERT INTO messages VALUES (?, ?, NOW())")
        .bind(user_id)
        .bind(message)
        .execute(pool.inner())
        .await
    {
        Ok(_) => (),
        Err(e) => panic!("Error inserting message: {}", e),
    }
}

#[launch]
async fn rocket() -> _ {
    let pool = match MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:thhz1234@localhost/chatsite")
        .await
    {
        Ok(pool) => pool,
        Err(e) => panic!("{}", e),
    };
    match sqlx::query(
        "CREATE TABLE IF NOT EXISTS messages (
            id INT NOT NULL,
            message VARCHAR(10000) NOT NULL,
            message_time DATETIME NOT NULL
        );",
    )
    .execute(&pool)
    .await
    {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    };
    rocket::build()
        .attach(Cors)
        .mount("/", routes![get_messages, insert_message])
        .manage(pool)
}

//todo:
//1. possibly make get_messages only return the x last message or the messages from the last hour
