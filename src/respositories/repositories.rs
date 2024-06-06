use diesel::{connection, ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::{
    models::models::{NewRustacean, Rustacean},
    schema::{crates::rustaceans_id, rustaceans},
};

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
