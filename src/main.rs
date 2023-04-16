#![feature(decl_macro)]

mod packing;
mod schema;

use std::time::Duration;

use rocket::serde::json::Json;
use schema::{PackingArea, PackingPlan};

#[macro_use]
extern crate rocket;

#[post("/packing-plans?<timeout>", data = "<data>")]
fn make_packing_plan(data: Json<PackingArea>, timeout: Option<u64>) -> Json<PackingPlan> {
    let plan = packing::fit(
        data.palett_size,
        data.palettes_n,
        data.boxes.clone(),
        timeout
            .map(|t| Duration::from_millis(t))
            .unwrap_or(Duration::MAX),
    );
    Json(plan)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![make_packing_plan])
}
