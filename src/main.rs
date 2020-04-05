#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;

mod controllers;
mod db;
mod models;
mod schema;
mod utils;

use rocket_contrib::json::Json;

fn main() {
  rocket::ignite()
    .manage(db::connect())
    .mount("/user", controllers::user::routes())
    .register(utils::catcher::catchers())
    .launch();
}
