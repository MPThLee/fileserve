#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#![feature(custom_derive)]

extern crate rand;
extern crate mime;
extern crate mime_guess;
extern crate multipart;
extern crate rocket;
extern crate dotenv;

#[macro_use] extern crate lazy_static;

use dotenv::dotenv;

pub mod id;
pub mod api;
pub mod error;

#[get("/i/<filename>")]
fn item(filename: id::ItemID) -> Option<rocket::response::NamedFile> {
    let filename = format!("content/files/{id}", id = filename);
    rocket::response::NamedFile::open(&filename).ok()
}

#[get("/r/<short>")]
fn urls(short: id::ItemID) -> Option<rocket::response::Redirect> {
    use std::io::Read;
    let filename = format!("content/urls/{id}", id = short);
    let mut contents = String::new();
    std::fs::File::open(filename).ok()?.read_to_string(&mut contents).ok()?;
    Some(rocket::response::Redirect::to(&contents))
}

// #[error()]
// fn error(req: &Request) -> content::Json{
    
// }

fn main() {
    let _ = dotenv().ok();
    
    rocket::ignite()
        .mount("/", routes![item, urls])
        .mount("/v1/api", routes![api::api_index])
        .catch(error::get_routes())
        .launch();
}
