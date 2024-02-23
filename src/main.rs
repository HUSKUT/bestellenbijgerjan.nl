
#[macro_use] extern crate rocket;

use reqwest::{Client, Error};
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderMap, USER_AGENT};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::hyper::header::ACCEPT_ENCODING;
use rocket::http::{Header, Status};
use rocket::{Request, Response};
use rocket::response::content::RawJson;
use rocket::response::{content, status};
use rocket_dyn_templates::{Template, context};
use rocket_dyn_templates::handlebars::JsonValue;
use rocket::serde::json::{Json, json};

fn result_to_string(result: Result<JsonValue, Error>) -> JsonValue {
    result.unwrap_or_else(|error| format!("Error: {}", error).parse().unwrap())
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/listrestaurants")]
async fn listRestaurants() -> JsonValue {

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT_ENCODING, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("X-Country-Code", "nl".parse().unwrap());
    headers.insert("X-Language-Code", "nl".parse().unwrap());
    headers.insert(USER_AGENT, "PostmanRuntime/7.36.3".parse().unwrap()); // Thuisbezorgd api throws 403 if you dont do this, lol.
    headers.insert("X-Session-Id", "816ef134-c0c0-4021-858b-b48b06bdd73a".parse().unwrap()); // Pretty sure Thuisbezorgd doesn't give a fuck about session-ids as long as they exist
    let client = Client::new();
    let request = client
        .get("http://cw-api.takeaway.com/api/v33/restaurants?deliveryAreaId=936640&postalCode=8021&lat=52.5172709&lng=6.086479499999999&limit=0&isAccurate=true&filterShowTestRestaurants=false")
        .headers(headers);

    let response = request.send().await;
    let body = response.unwrap().json().await;

    let result = result_to_string(body);
    json!( { "body": result["restaurants"]} )
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, listRestaurants, all_options])
        .attach(Cors)
        .attach(Template::fairing())
}

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