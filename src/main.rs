#[macro_use]
extern crate rocket;
use rpc_optimize::routes::{
    health_check::health_check,
    home::index, // users::DbConn,
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        // .attach(DbConn::fairing())
        .mount("/", routes![index, health_check])
}
