#[macro_use] extern crate rocket;

use std::path::{Path, PathBuf};
use rocket::tokio::time::{sleep, Duration};
use rocket::{Build, Request, Rocket};
use rocket::fs::{FileServer, NamedFile};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn noob() -> &'static str {
    "Hello, noob!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[catch(404)]
fn noob_not_found(req: &Request) -> String {
    format!("I'm sorry, but '{}' is not a valid path!!", req.uri().path())
}

#[catch(default)]
fn general_error(req: &Request) -> String {
    format!("I'm sorry, but '{}' is not a valid path!!!", req.uri().path())
}


#[launch]
fn rocket() ->  Rocket<Build> {
    rocket::build().mount("/", routes![index])
        .mount("/noob", routes![noob])
        .mount("/", routes![delay])
        .mount("/", routes![hello])
        .register("/noob/not_found", catchers![noob_not_found])
        .register("/", catchers![general_error])

}

