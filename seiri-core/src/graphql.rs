use bangs::Bang;
use database::query_tracks;
use juniper;
use juniper::{FieldError, FieldResult};
use paths::get_connection_pool;
use track::Track;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;

pub struct Context {
    pub pool: Pool<SqliteConnectionManager>
}
// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

impl Context {
    pub fn new() -> Context {
        Context {
            pool: get_connection_pool()
        }
    }
}

pub struct Query;

impl Query {
    pub fn new() -> Query {
        Query {}
    }
}

graphql_object!(Query: Context |&self| {

    field all_tracks(&executor) -> FieldResult<Vec<Track>> {
        let conn = executor.context().pool.get().unwrap();
        match query_tracks(Bang::All, &conn) {
            Ok(tracks) => Ok(tracks),
            Err(err) => Err(FieldError::from(err))
        }
    }


    field query_tracks(&executor, query: String) -> FieldResult<Vec<Track>> {
        let conn = executor.context().pool.get().unwrap();
        match Bang::new(&query) {
            Ok(bang) => {
             match query_tracks(bang, &conn) {
                    Ok(tracks) => Ok(tracks),
                    Err(err) => Err(FieldError::from(err))
                }
            },
            Err(err) => Err(FieldError::from(err))
        }
    }
});
