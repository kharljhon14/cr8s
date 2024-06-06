use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::models::*;
use crate::schema::*;

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub async fn find(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result(connection).await
    }

    pub async fn get_multiple(
        connection: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).get_results(connection).await
    }

    pub async fn create(
        connection: &mut AsyncPgConnection,
        new_rustacean: NewRustacean,
    ) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(connection)
            .await
    }

    pub async fn update(
        connection: &mut AsyncPgConnection,
        rustacean: Rustacean,
    ) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(rustacean.id))
            .set((
                rustaceans::name.eq(rustacean.name),
                rustaceans::email.eq(rustacean.email),
            ))
            .get_result(connection)
            .await
    }

    pub async fn delete(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id))
            .execute(connection)
            .await
    }
}

pub struct CratesRespository;

impl CratesRespository {
    pub async fn find(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result(connection).await
    }

    pub async fn get_multiple(
        connection: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<Crate>> {
        crates::table.limit(limit).get_results(connection).await
    }

    pub async fn create(
        connection: &mut AsyncPgConnection,
        new_crate: NewCrate,
    ) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_crate)
            .get_result(connection)
            .await
    }

    pub async fn update(connection: &mut AsyncPgConnection, a_crate: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(a_crate.id))
            .set((
                crates::name.eq(a_crate.name),
                crates::rustaceans_id.eq(a_crate.rustaceans_id),
                crates::code.eq(a_crate.code),
                crates::version.eq(a_crate.version),
                crates::description.eq(a_crate.description),
            ))
            .get_result(connection)
            .await
    }

    pub async fn delete(connection: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id))
            .execute(connection)
            .await
    }
}
