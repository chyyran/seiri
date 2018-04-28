use juniper;
use juniper::{FieldError, FieldResult};
use paths::{get_connection_pool, ensure_music_folder, reconsider_track};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rayon::prelude::*;
use std::path::Path;
use seiri::Track;
use seiri::Bang;
use seiri::database::{query_tracks, remove_track, add_track};

use config;

pub struct Context {
    pub pool: Pool<SqliteConnectionManager>,
    pub config: config::Config,
}
// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

impl Context {
    pub fn new() -> Context {
        Context {
            pool: get_connection_pool(),
            config: config::get_config(),
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

     field refresh(&executor, tracks: Vec<String>) -> FieldResult<Vec<Option<Track>>> {
       Ok(tracks.into_par_iter()
        .map(|track| {
        let conn = executor.context().pool.get().unwrap();
        let config = &executor.context().config;
           match query_tracks(Bang::from(Path::new(&track)), &conn, None, None) {
                Ok(tracks) => { 
                    if let Some(track) = tracks.into_iter().next() {
                        let library_path = ensure_music_folder(&config.music_folder).unwrap().0;
                        match reconsider_track(&track, &library_path) {
                            Err(_) => None,
                            Ok(None) =>  {
                                remove_track(&track, &conn);
                                None
                            }
                            Ok(Some(new_track)) => {
                                remove_track(&track, &conn);
                                add_track(&new_track, &conn);
                                Some(track)
                            }
                        }
                    } else {
                        None
                    }
                },
                Err(err) => None
           }
        })
        .filter(|track| !track.is_none())
        .collect::<Vec<Option<Track>>>())
    }


    field tracks(&executor, query: String, first: Option<i32>, after: Option<i32>) -> FieldResult<Vec<Track>> {
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

    field count(&executor, query: String, first: Option<i32>, after: Option<i32>) -> FieldResult<i32> {
        let conn = executor.context().pool.get().unwrap();
        match Bang::new(&query) {
            Ok(bang) => {
             match query_tracks(bang, &conn, first, after) {
                    Ok(tracks) => Ok(tracks.len() as i32),
                    Err(err) => Err(FieldError::from(err))
                }
            },
            Err(err) => Err(FieldError::from(err))
        }
    }
    

});
