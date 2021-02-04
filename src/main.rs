#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> String {
    String::from("Hello")
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}