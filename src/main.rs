#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/secret")]
fn secret() -> &'static str {
    "Hello, secret world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, secret])
}
