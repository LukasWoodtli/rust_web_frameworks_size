#[macro_use] extern crate rocket;

use rocket::figment::Figment;
use rocket::fs::FileServer;

#[get("/")]
fn api_call() -> &'static str {
    "Hello from API"
}


#[launch]
fn rocket() -> _ {
    let figment = Figment::from(rocket::Config::default())
        .merge(("address", "127.0.0.1"))
        .merge(("port", 3000));
    rocket::custom(figment)
        .mount("/api", routes![api_call])
        .mount("/", FileServer::from("www"))
}
