#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::content;

#[get("/")]
fn index() -> content::Html<&'static str> {
    content::Html("<h1>Welcome to Rust Web Development!</h1>")
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[catch(404)]
fn not_found() -> content::Html<&'static str> {
    content::Html("<h1>404 Not Found</h1>")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello])
        .register("/", catchers![not_found])
}
