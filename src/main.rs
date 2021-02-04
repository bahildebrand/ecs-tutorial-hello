#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn item() -> String {
    String::from("Hello")
}

fn main() {
    rocket::ignite().mount("/", routes![item]).launch();
}