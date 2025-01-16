use rocket::get;
use rocket::http::Status;

#[get("/")]
pub async fn index() -> Result<Status, Status> {
    Ok(Status::Ok)
}
