use crate::schema::_templates_;
use chrono::{NaiveDateTime, Utc};
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

#[table_name = "_templates_"]
#[derive(Serialize, AsChangeset, Deserialize, Queryable, Insertable)]
pub struct _Template_ {
    pub id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    // -- Complete this struct
}

impl _Template_ {
    pub fn create(_template_: _Template_, conn: &MysqlConnection) -> Result<_Template_, Error> {
        let new__template_ = _Template_ {
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            .._template_
        };

        let ops = diesel::insert_into(_templates_::table)
            .values(&new__template_)
            .execute(conn);
	
	match ops {
            Ok(_) => _templates_::table.order(_templates_::id.desc()).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn read(conn: &MysqlConnection) -> Result<Vec<_Template_>, Error> {
        _templates_::table.order(_templates_::id.asc()).load::<_Template_>(conn)
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<_Template_, Error> {
        _templates_::table.find(id).first(conn)
    }

    pub fn update(id: i32, _template_: _Template_, conn: &MysqlConnection) -> Result<_Template_, Error> {
        let new__template_ = _Template_ {
            updated_at: Some(Utc::now().naive_utc()),
            .._template_
        };
        
	let ops = diesel::update(_templates_::table.find(id))
            .set(&new__template_)
            .execute(conn);

	match ops {
            Ok(_) => _templates_::table.find(id).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(_templates_::table.find(id)).execute(conn).is_ok()
    }
}
