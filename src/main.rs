#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use reqwest;

#[get("/hello")]
fn hello() -> String {
    String::from("Hello")
}

#[get("/hello/world")]
fn hello_world() -> String {
    match reqwest::blocking::get("http://hello-world.hello-world/world/") {
        Ok(resp) => format!("Hello {}", resp.text().unwrap()),
        Err(e) => format!("Error {}", e)
    }
}

fn main() {
    rocket::ignite().mount("/", routes![hello, hello_world]).launch();
}