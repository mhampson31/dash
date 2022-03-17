#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize};

use rocket_dyn_templates::Template;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Service {
    name: String,
    url: String,
    category: String,
    active: bool
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TemplateContext<'r> {
    title: &'r str,
    services: Vec<Service>
}

#[get("/")]
fn index() -> Template {
    let mut services:Vec<Service> = Vec::new();
    let snames = [
        ("Foundry", "sigil", "Games", true),
        ("Mealie", "kitchen", "Home", true),
        ("Authentik", "id", "Admin", true),
        ("Bookstack", "wiki", "Home", true)
    ];

    for s in snames {
        services.push(Service {
            name: String::from(s.0),
            url: String::from(s.1),
            category: String::from(s.2),
            active: s.3
            }
        )
    }

    let context = TemplateContext {title: "Index", services: services};
    Template::render("index", context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
