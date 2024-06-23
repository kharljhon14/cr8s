use std::{io::Write, str::FromStr};

use chrono::NaiveDateTime;
use diesel::{
    associations::Identifiable,
    deserialize::{FromSql, FromSqlRow, Queryable},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::ToSql,
    sql_types::Text,
};
use rocket_db_pools::diesel::Insertable;
use serde::{Deserialize, Serialize};

use crate::schema::roles;

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
pub struct Role {
    pub id: i32,
    pub code: RoleCode,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=roles)]
pub struct NewRole {
    pub code: RoleCode,
    pub name: String,
}

#[derive(AsExpression, Debug, FromSqlRow, Deserialize, Serialize)]
#[diesel(sql_type=Text)]
pub enum RoleCode {
    Admin,
    Editor,
    Viewer,
}

impl ToString for RoleCode {
    fn to_string(&self) -> String {
        match self {
            RoleCode::Admin => String::from("admin"),
            RoleCode::Editor => String::from("editor"),
            RoleCode::Viewer => String::from("viewer"),
        }
    }
}

impl FromStr for RoleCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(RoleCode::Admin),
            "Editor" => Ok(RoleCode::Editor),
            "Viewer" => Ok(RoleCode::Viewer),
            _ => Err(()),
        }
    }
}

impl FromSql<Text, Pg> for RoleCode {
    fn from_sql(value: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"admin" => Ok(RoleCode::Admin),
            b"Editor" => Ok(RoleCode::Editor),
            b"Viewer" => Ok(RoleCode::Viewer),
            _ => Ok(RoleCode::Viewer),
        }
    }
}

impl ToSql<Text, Pg> for RoleCode {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match self {
            RoleCode::Admin => out.write_all(b"admin"),
            RoleCode::Editor => out.write_all(b"editor"),
            RoleCode::Viewer => out.write_all(b"viewer"),
        };

        Ok(diesel::serialize::IsNull::No)
    }
}
