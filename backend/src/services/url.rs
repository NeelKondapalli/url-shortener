use rocket::serde::json::{Json};
use crate::routes::url::URL;
use std::iter;
use std::collections::HashMap;
use rand::{distributions::Alphanumeric, Rng};

pub fn get_long_url(id: String) -> URL {
    let mut url_map: HashMap<String, String> = HashMap::new();
    url_map.insert(String::from("L8QxGvbb"), String::from("https://en.wikipedia.org/wiki/Cattle"));
    let long_url: URL;
    if let Some(url) = url_map.get(&id) {
        long_url = URL {
            url: format!("{}", url)
        };
    } else {
        long_url = URL {
            url: String::from("https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404")
        };
    }
   
    long_url
}

pub fn post_short_url(long_url: Json<URL>) -> URL {
    let mut rng = rand::thread_rng();
    let urlID: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(8)
        .collect();

    let urlSuffix = urlID.to_string();

    let short_url = URL {
        url: format!("http://127.0.0.1:8000/api/url/{}", urlSuffix)
    };
    short_url
}