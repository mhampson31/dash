#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ServiceCard {
    name: String,
    url: String,
    category: String,
    active: bool
}


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TemplateContext<'r> {
    title: &'r str,
    cards: Vec<ServiceCard>
}

#[get("/")]
fn index() -> Template {
    let mut cards:Vec<ServiceCard> = Vec::new();
    cards.push(ServiceCard {
        name: String::from("Foundry"),
        url: String::from("sigil"),
        category: String::from("Games"),
        active: true
    });
    cards.push(ServiceCard {
        name: String::from("Mealie"),
        url: String::from("kitchen"),
        category: String::from("Home"),
        active: true
    });
    let context = TemplateContext {title: "Index", cards: cards};
    Template::render("index", context)
}

#[get("/test")]
fn test() -> &'static str {
    "This is a test route."
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![test])
        .attach(Template::fairing())
}
