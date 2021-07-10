use crate::db;
use crate::models::user::User;
use crate::utils::response::*;
use crate::*;
//use rocket_contrib::json::{Json, JsonError};

#[get("/all")]
fn find_all(conn: db::Connection) -> Result<ApiResponse, ApiError> {
    let result = User::findAll(&conn);
    match result {
        Ok(r) => Ok(success(json!(r))),
        Err(e) => Err(db_error(e)),
    }
}

// 라우팅
pub fn routes() -> Vec<rocket::Route> {
    routes![find_all]
}
