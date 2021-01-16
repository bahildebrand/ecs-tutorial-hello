#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/item/<item>")]
fn item(item: String) -> String {
    format!("Item: {}", item)
}

fn main() {
    rocket::ignite().mount("/", routes![item]).launch();
}