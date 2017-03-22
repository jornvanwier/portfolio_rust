#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use rocket_contrib::Template;

#[derive(Debug, Serialize)]
struct Page {
    title: &'static str
}

#[get("/")]
fn sup() -> Template {
    let context = Page{title: "test"};

    Template::render("home", &context)
}

#[get("/greet/<name>")]
fn greet(name: &str) -> String {
    format!("Hello {}!", name)
}

fn main() {
    rocket::ignite().mount("/", routes![sup, greet]).launch();
}
