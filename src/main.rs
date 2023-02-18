#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/noob")]
fn noob() -> &'static str {
    "Hello, noob!"
}

#[launch]
fn rocket() ->  Rocket<Build> {
    rocket::build().mount("/", routes![index])
        .mount("/", routes![noob])
}

