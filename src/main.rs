mod packing;
mod schema;

use std::time::Duration;

use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    launch, options, post, routes,
    serde::json::Json,
    Request, Response, Rocket,
};
use schema::{PackingArea, PackingPlan};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

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
    rocket::build()
        .mount("/", routes![make_packing_plan, all_options])
        .attach(CORS)
}
