#[macro_use] extern crate rocket;


#[get("/")]
fn say_hello() -> &'static str {
    "Hello, welcome!"
}

// Mod adds the module to the crate root, makes it available for all files under.
mod routes;
mod services;

use routes::url::get_long_url;
use routes::url::post_short_url;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![say_hello, get_long_url, post_short_url])
}

