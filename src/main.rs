#[macro_use]
extern crate rocket;

use dotenv;
use reqwest::Url;
use rocket::fairing::AdHoc;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::Redirect;
use rocket::routes;
use rocket::serde::{Deserialize, Serialize};
use rocket_dyn_templates::Template;
use rocket_oauth2::{HyperRustlsAdapter, OAuth2, OAuthConfig, StaticProvider, TokenResponse};

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

fn get_user_applications() -> ServiceList {
    let app = Service {
        name: String::from("Test"),
        url: String::from("https://id.flyingseamonsters.com"),
        category: String::from("Home"),
        active: true,
    };
    let all_apps = ServiceList {
        services: vec![app],
    };
    all_apps
}

#[get("/login/authentik")]
fn authentik_login(oauth2: OAuth2<Authentik>, mut cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(&mut cookies, &["user:read"]).unwrap()
}

#[get("/auth/authentik")]
fn authentik_callback(token: TokenResponse<Authentik>, mut cookies: &CookieJar<'_>) -> Redirect {
    cookies.add_private(
        Cookie::build("token", token.access_token().to_string())
            .same_site(SameSite::Lax)
            .finish(),
    );
    Redirect::to("/")
}

#[get("/")]
fn index() -> Template {
    let applications = get_user_applications();

    let context = TemplateContext {
        title: "Index",
        service_list: applications,
    };
    Template::render("index", context)
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();
    let api_key = dotenv::var("API_KEY").expect("Cannot get API key");
    let ak_root = dotenv::var("AUTHENTIK_URL").expect("Cannot get Authentik URL");
    let authentik_url: Url = Url::parse(&ak_root).expect("Could not parse Authentik URL");

    let user_api: Url = authentik_url.join("api/v3/core/users/me").unwrap();

    //let header = Header::new("Authentication", api_key);

    /*    let client = reqwest::Client::new();
    let body = client
        .get(user_api)
        .bearer_auth(api_key)
        .send()
        .await
        .unwrap()
        .text()
        .await;

    println!("body = {:?}", body); */

    rocket::build()
        .mount("/", routes![index, authentik_callback, authentik_login])
        .attach(Template::fairing())
        .attach(AdHoc::on_ignite("OAuth Config", |mut rocket| async {
            let provider = StaticProvider {
                auth_uri: dotenv::var("OAUTH2_AUTH_URI")
                    .expect("Cannot get OAuth2 Auth URI")
                    .into(),
                token_uri: dotenv::var("OAUTH2_TOKEN_URI")
                    .expect("Cannot get OAuth2 Token URI")
                    .into(),
            };
            let client_id = dotenv::var("OAUTH2_CLIENT_ID").expect("Cannot get OAuth2 Client ID");
            let client_secret =
                dotenv::var("OAUTH2_CLIENT_SECRET").expect("Cannot get OAuth2 Client Secret");
            let redirect_uri = Some(dotenv::var("OAUTH2_REDIRECT_URI")
                .expect("Cannot get OAuth2 Redirect URI")
                .to_string());

            let config = OAuthConfig::new(provider, client_id, client_secret, redirect_uri);

            rocket.attach(OAuth2::<Authentik>::custom(
                HyperRustlsAdapter::default(),
                config,
            ))
        }))
}
