#[macro_use]
extern crate rocket;

use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::Redirect;
use rocket::serde::{Deserialize, Serialize};
use rocket_dyn_templates::Template;
use rocket_oauth2::{OAuth2, TokenResponse};
use serde_yaml::from_str;
use std::fs;

// This struct will only be used as a type-level key. Multiple
// instances of OAuth2 can be used in the same application by
// using different key types.
struct Authentik;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Service {
    name: String,
    url: String,
    category: String,
    active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ServiceList {
    services: Vec<Service>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TemplateContext<'r> {
    title: &'r str,
    service_list: ServiceList,
}

#[get("/")]
fn index() -> Template {
    let config = fs::read_to_string("dash.yaml").expect("Failed to read input");
    let services = from_str(&config).unwrap();

    let context = TemplateContext {
        title: "Index",
        service_list: services,
    };
    Template::render("index", context)
}

// This route calls `get_redirect`, which sets up a token request and
// returns a `Redirect` to the authorization endpoint.
#[get("/login")]
fn authentik_login(oauth2: OAuth2<Authentik>, cookies: &CookieJar<'_>) -> Redirect {
    // We want the "user:read" scope. For some providers, scopes may be
    // pre-selected or restricted during application registration. We could
    // use `&[]` instead to not request any scopes, but usually scopes
    // should be requested during registation, in the redirect, or both.
    oauth2.get_redirect(cookies, &["user:read"]).unwrap()
}

// This route, mounted at the application's Redirect URI, uses the
// `TokenResponse` request guard to complete the token exchange and obtain
// the token.
//
// The order is important here! If Cookies is positioned before
// TokenResponse, TokenResponse will be unable to verify the token exchange
// and return a failure.
#[get("/auth")]
fn authentik_callback(token: TokenResponse<Authentik>, cookies: &CookieJar<'_>) -> Redirect {
    // Set a private cookie with the access token
    cookies.add_private(
        Cookie::build("token", token.access_token().to_string())
            .same_site(SameSite::Lax)
            .finish(),
    );
    Redirect::to("/")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, authentik_login, authentik_callback])
        .attach(OAuth2::<Authentik>::fairing("authentik"))
        .attach(Template::fairing())
}
