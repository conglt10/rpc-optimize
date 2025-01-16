use rocket::get;
use rocket::http::Status;

#[get("/health_check")]
pub async fn health_check() -> Result<Status, Status> {
    Ok(Status::Ok)
}
