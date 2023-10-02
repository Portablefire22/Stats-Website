use std::env;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str{
    "Hello, World!"
}

#[get("/<region>/<username>")]
fn userProfile(region: &str, username: &str) -> String{
    format!("Region: {}\nUsername: {}", region, username)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/search/", routes![userProfile])
}
