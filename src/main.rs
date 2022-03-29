#[macro_use]
extern crate rocket;

use rocket::serde::{Deserialize, Serialize};
use rocket_dyn_templates::Template;
use serde_yaml::from_str;
use std::fs;

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
