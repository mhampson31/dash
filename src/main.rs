#[macro_use]
extern crate rocket;

use dotenv;
use reqwest::Url;
use rocket::http::Header;
use rocket::routes;
use rocket::serde::{Deserialize, Serialize};
use rocket_dyn_templates::Template;

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

    let client = reqwest::Client::new();
    let body = client
        .get(user_api)
        .bearer_auth(api_key)
        .send()
        .await
        .unwrap()
        .text()
        .await;

    println!("body = {:?}", body);

    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
