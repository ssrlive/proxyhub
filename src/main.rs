use rocket::catchers;
use rocket::{catch, get, routes, Request};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let rt_env = rocket::build();
    let _ = rt_env
        .register("/", catchers![not_found])
        .mount("/", routes![index])
        .launch()
        .await?;
    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}
