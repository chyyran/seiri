use neon::prelude::*;
use num_traits::cast::ToPrimitive;
use seiri::config::get_config;
use seiri::database;
use seiri::paths;
use seiri::Bang;
use seiri::Track;
use std::path::Path;

#[allow(non_snake_case)]
fn refresh_tracks(mut ctx: FunctionContext) -> JsResult<JsUndefined> {
    let config = get_config().unwrap();
    let conn = database::get_database_connection();
    let library_path = Path::new(&config.music_folder);

    let args = ctx.argument::<JsArray>(0)?;

    let mut track_filenames: Vec<String> = Vec::new();
    for i in 0..args.len(&mut ctx) {
        let result = args
            .get(&mut ctx, i)?
            .downcast::<JsString, _>(&mut ctx)
            .or_throw(&mut ctx)?
            .value(&mut ctx);
        track_filenames.push(result);
    }

    for file in track_filenames {
        let tracks = database::query_tracks(Bang::FilePath(file.clone()), &conn, None, None);
        if let Ok(tracks) = tracks {
            if let Some(track) = tracks.into_iter().next() {
                match paths::reconsider_track(&track, &library_path) {
                    Ok(Some(new_track)) => {
                        println!("RECONSIDERED OK {:?}", new_track);
                        database::remove_track(&track, &conn);
                        database::add_track(&new_track, &conn);
                    }
                    Ok(None) => {
                        println!("RECONSIDERED NOT FOUND {:?}", track);
                        database::remove_track(&track, &conn);
                    }
                    Err(_) => {
                        println!(
                            "RECONSIDER ERROR FOR {}. Is tools set up correctly?",
                            file
                        )
                    },
                }
            }
        }
    }
    Ok(ctx.undefined())
}

#[allow(non_snake_case)]
fn query_tracks(mut ctx: FunctionContext) -> JsResult<JsObject> {
    let ret = ctx.empty_object();

    let query = ctx.argument::<JsString>(0)?.value(&mut ctx);

    let bang = Bang::new(&query).unwrap();
    let conn = database::get_database_connection();
    let results = database::query_tracks(bang, &conn, None, None);

    let result: JsResult<JsObject> = match results {
        Ok(results) => {
            let jsTracks = ctx.empty_array();

            for (i, track) in results.into_iter().enumerate() {
                let jsTrack = ctx.empty_object();
                let filePath = ctx.string(&track.file_path.into_os_string().into_string().unwrap());
                jsTrack.set(&mut ctx, "filePath", filePath)?;
        
                let title = ctx.string(&track.title);
                jsTrack.set(&mut ctx, "title", title)?;
        
                let artist = ctx.string(&track.artist);
                jsTrack.set(&mut ctx, "artist", artist)?;
        
                let jsAlbumArtists = ctx.empty_array();
        
                for (i, artist) in track.album_artists.into_iter().enumerate() {
                    let jsArtistString = ctx.string(&artist);
                    jsAlbumArtists.set(&mut ctx, i as u32, jsArtistString)?;
                }
        
                jsTrack.set(&mut ctx, "albumArtists", jsAlbumArtists)?;
                let album = ctx.string(&track.album);
                jsTrack.set(&mut ctx, "album", album)?;
        
                let trackNumber = ctx.number(track.track_number);
                jsTrack.set(&mut ctx, "trackNumber", trackNumber)?;
        
                match &track.musicbrainz_track_id {
                    Some(track_id) => {
                        let trackId = ctx.string(track_id);
                        jsTrack.set(&mut ctx, "musicbrainzTrackId", trackId)
                    }
                    None => {
                        let null = ctx.null();
                        jsTrack.set(&mut ctx, "musicbrainzTrackId", null)
                    }
                }?;
        
                let hasFrontCover = ctx.boolean(track.has_front_cover);
                jsTrack.set(&mut ctx, "hasFrontCover", hasFrontCover)?;
        
                let frontCoverHeight = ctx.number(track.front_cover_height);
                jsTrack.set(&mut ctx, "frontCoverHeight", frontCoverHeight)?;
        
                let frontCoverWidth = ctx.number(track.front_cover_width);
                jsTrack.set(&mut ctx, "frontCoverWidth", frontCoverWidth)?;
        
                let bitrate = ctx.number(track.bitrate);
                jsTrack.set(&mut ctx, "bitrate", bitrate)?;
        
                let sampleRate = ctx.number(track.sample_rate);
                jsTrack.set(&mut ctx, "sampleRate", sampleRate)?;
        
                let source = ctx.string(&track.source);
                jsTrack.set(&mut ctx, "source", source)?;
        
                let discNumber = ctx.number(track.disc_number);
                jsTrack.set(&mut ctx, "discNumber", discNumber)?;
        
                let duration = ctx.number(track.duration);
                jsTrack.set(&mut ctx, "duration", duration)?;
        
                let fileType = ctx.number(track.file_type.to_i32().unwrap());
                jsTrack.set(&mut ctx, "fileType", fileType)?;
        
                let updated = ctx.string(&track.updated);
                jsTrack.set(&mut ctx, "updated", updated)?;
        
                jsTracks.set(&mut ctx, i as u32, jsTrack)?;
            }
            ret.set(&mut ctx, "tracks", jsTracks)?;
            Ok(ret)
        }
        Err(e) => {
            ctx.throw_error(e.to_string())
        }
    };
    result
}

register_module!(mut m, {
    m.export_function("queryTracks", query_tracks)?;
    m.export_function("refreshTracks", refresh_tracks)?;
    Ok(())
});
