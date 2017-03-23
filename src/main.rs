#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use rocket_contrib::Template;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

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

// Javascript
#[get("/js/<file..>")]
fn javascript_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("js/").join(file)).ok()
}
// CSS
#[get("/css/<file..>")]
fn css_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("css/").join(file)).ok()
}

// Other files
#[get("/files/<file..>")]
fn other_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![
        sup, 
        greet,
        javascript_files,
        css_files,
        other_files
        ]).launch();
}
