// api/handler.rs
use rocket::{get, routes, serde::json::Json};
use vercel_runtime::{Body, Error, Event, Request, Response, run};
use http::StatusCode;

#[get("/hello")]
fn hello() -> Json<&'static str> {
    Json("Hello from Rocket on Vercel!")
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(event: Event) -> Result<Response<Body>, Error> {
    let rocket_app = rocket::build().mount("/", routes![hello]);
    /* 
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
                "message": "Hello from Rust on Vercel!"
            })
            .to_string()
            .into(),
        )?)
    */
}