#[macro_use]
extern crate rocket;

use rocket::{launch, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello from Rocket on Vercel!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/hello", routes![index])
}