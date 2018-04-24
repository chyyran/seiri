use bangs::Bang;
use database::query_tracks;
use juniper;
use juniper::{FieldError, FieldResult};
use paths::get_connection_pool;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rayon::prelude::*;
use std::path::Path;
use track::Track;

pub struct Context {
    pub pool: Pool<SqliteConnectionManager>,
}
// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

impl Context {
    pub fn new() -> Context {
        Context {
            pool: get_connection_pool(),
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
        match query_tracks(Bang::All, &conn, None, None) {
            Ok(tracks) => Ok(tracks),
            Err(err) => Err(FieldError::from(err))
        }
    }

     field refresh_tracks(&executor, tracks: Vec<String>) -> FieldResult<Vec<Option<Track>>> {
       Ok(tracks.into_par_iter()
        .map(|track| {
        let conn = executor.context().pool.get().unwrap();
           match query_tracks(Bang::from(Path::new(&track)), &conn, None, None) {
                Ok(tracks) => tracks.into_iter().next(),
                Err(err) => None
           }
        })
        .filter(|track| !track.is_none())
        .collect::<Vec<Option<Track>>>())
    }


    field query_tracks(&executor, query: String, first: Option<i32>, after: Option<i32>) -> FieldResult<Vec<Track>> {
        let conn = executor.context().pool.get().unwrap();
        match Bang::new(&query) {
            Ok(bang) => {
             match query_tracks(bang, &conn, first, after) {
                    Ok(tracks) => Ok(tracks),
                    Err(err) => Err(FieldError::from(err))
                }
            },
            Err(err) => Err(FieldError::from(err))
        }
    }

   
});
