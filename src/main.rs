#[macro_use] extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
    "Hello world"
}

#[get("/muchachos")]
fn much() -> &'static str {
    "gringo"
}

#[launch]
fn rocket() -> _ {

    rocket::build()
        .mount("/hello", routes![world])
        .mount("/olla", routes![much])

    // can also be:
    // rocket::build().mount("", routes![world, muchachos])
}
