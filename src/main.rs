#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn sup() -> &'static str {
    "Sup"
}

#[get("/greet/<name>")]
fn greet(name: &str) -> String {
    format!("Hello {}!", name)
}

fn main() {
    rocket::ignite().mount("/", routes![sup, greet]).launch();
}
