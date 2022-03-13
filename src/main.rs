#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TemplateContext<'r> {
    title: &'r str
}

#[get("/")]
fn index() -> Template {
    let context = TemplateContext {title: "Index"};
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
