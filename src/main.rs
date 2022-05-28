#[macro_use] extern crate rocket;

mod controllers;

use controllers::customer;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![customer::get_customer])
}