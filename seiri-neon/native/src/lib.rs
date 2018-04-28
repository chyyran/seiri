#[macro_use]
extern crate neon;

extern crate app_dirs;
extern crate chrono;
extern crate humantime;
extern crate itertools;
extern crate lazy_static;

extern crate rand;
extern crate rayon;
extern crate regex;
extern crate rusqlite;
extern crate toml;
extern crate seiri;

mod path;

use seiri::Track;
use seiri::Bang;
use seiri::database;
use neon::vm::{Call, JsResult};
use neon::js::{Object, JsArray, JsString, JsObject, JsInteger, JsBoolean, JsNull};
use neon::vm::Throw;


fn get_appdata_path(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    match path::get_appdata_path() {
        Ok(path) => {
            Ok(JsString::new(scope, &path.to_string_lossy()).unwrap())
        }
        Err(_) => {
            Err(Throw)
        }
    }
}

#[allow(non_snake_case)]
fn query_tracks(call: Call) -> JsResult<JsObject> {
    let scope = call.scope;
    let ret = JsObject::new(scope);
    let query = &call.arguments.require(scope, 0)?.check::<JsString>()?.value();
    let bang = Bang::new(query).unwrap();
    let conn = path::get_database_connection();
    let results: Vec<Track> = database::query_tracks(bang, &conn, None, None).unwrap();
    let jsTracks = JsArray::new(scope, results.len() as u32);
    for (i, track) in results.into_iter().enumerate() {
        let mut jsTrack = JsObject::new(scope);
        jsTrack.set("filePath", JsString::new(scope, &track.file_path).unwrap())?;
        jsTrack.set("title", JsString::new(scope, &track.title).unwrap())?;
        jsTrack.set("artist", JsString::new(scope, &track.artist).unwrap())?;

        let jsAlbumArtists = JsArray::new(scope, track.album_artists.len() as u32);

        for (i, artist) in track.album_artists.into_iter().enumerate() {
            let jsArtistString = JsString::new(scope, &artist).unwrap();
            jsAlbumArtists.set(i as u32, jsArtistString)?;
        }
        jsTrack.set("albumArtists", jsAlbumArtists)?;
        jsTrack.set("album", JsString::new(scope, &track.album).unwrap())?;
        jsTrack.set("trackNumber", JsInteger::new(scope, track.track_number))?;
        
        match &track.musicbrainz_track_id {
            Some(track_id) => jsTrack.set("musicbrainzTrackId", JsString::new(scope, track_id).unwrap()),
            None => jsTrack.set("musicbrainzTrackId", JsNull::new())
        }?;

        jsTrack.set("hasFrontCover", JsBoolean::new(scope, track.has_front_cover))?;
        jsTrack.set("frontCoverHeight", JsInteger::new(scope, track.front_cover_height))?;
        jsTrack.set("frontCoverWidth", JsInteger::new(scope, track.front_cover_width))?;
        jsTrack.set("bitrate", JsInteger::new(scope, track.bitrate))?;
        jsTrack.set("sampleRate", JsInteger::new(scope, track.sample_rate))?;
        jsTrack.set("source", JsString::new(scope, &track.source).unwrap())?;
        jsTrack.set("discNumber", JsInteger::new(scope, track.disc_number))?;
        jsTrack.set("duration", JsInteger::new(scope, track.duration))?;
        jsTrack.set("fileType", JsString::new(scope, &track.file_type.to_string()).unwrap())?;
        jsTrack.set("updated", JsString::new(scope, &track.updated).unwrap())?;
        jsTracks.set(i as u32, jsTrack)?;
    }
    ret.set("tracks", jsTracks)?;
    Ok(ret)
}

register_module!(m, {
    m.export("queryTracks", query_tracks)
});
