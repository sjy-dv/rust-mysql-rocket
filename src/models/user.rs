use crate::schema::user;
use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, Utc};
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

#[table_name = "user"]
#[derive(Serialize, AsChangeset, Deserialize, Queryable, Insertable)]
pub struct User {
    pub idx: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl User {
    pub fn findAll(conn: &MysqlConnection) -> Result<Vec<User>, Error> {
        user::table.order(user::idx.asc()).load::<User>(conn)
    }
}
