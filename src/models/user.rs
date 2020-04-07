use crate::schema::users;
use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, Utc};
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

#[table_name = "users"]
#[derive(Serialize, AsChangeset, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub name: String,
    pub email: String,
    pub latitude: Option<BigDecimal>,
    pub longitude: Option<BigDecimal>,
    pub language: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub gender: Option<String>,
    pub age: Option<i32>,
}

impl User {
    pub fn create(user: User, conn: &MysqlConnection) -> Result<User, Error> {
        let new_user = User {
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..user
        };
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)
            .expect("Error creating new user");

        users::table.order(users::id.desc()).first(conn)
    }

    pub fn read(conn: &MysqlConnection) -> Result<Vec<User>, Error> {
        users::table.order(users::id.asc()).load::<User>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<Vec<User>, Error> {
        users::table
            .find(id)
            .order(users::id.asc())
            .load::<User>(conn)
    }

    pub fn update(id: i32, user: User, conn: &MysqlConnection) -> bool {
        let new_user = User {
            updated_at: Some(Utc::now().naive_utc()),
            ..user
        };
        diesel::update(users::table.find(id))
            .set(&new_user)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(users::table.find(id)).execute(conn).is_ok()
    }
}
