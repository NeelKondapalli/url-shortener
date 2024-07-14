use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket::response::Redirect;
use crate::services;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct URL {
    pub url: String
}

// #[get("/url/get-long")]
// pub fn get_long_url() -> Json<URL> {
//     Json(services::url::get_long_url())
// }

#[get("/url/<id>")]
pub fn get_long_url(id: String) -> Redirect {
    let response = services::url::get_long_url(id);
    // Redirect::to("https://doc.rust-lang.org/rust-by-example/std/hash.html")
    Redirect::to(response.url)
}

#[post("/url/post-short", format = "json", data = "<long_url>")]
pub fn post_short_url(long_url: Json<URL>) -> Json<URL> {
    Json(services::url::post_short_url(long_url))
}

