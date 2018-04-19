extern crate itertools;
extern crate lazy_static;

use track::TrackFileType;
use error::{Result};
use super::lexer::{lex_query};
use super::parser::{parse_token_stream};

#[derive(Debug)]
pub enum Bang {
    All,
    TitleSearch(String),
    TitleSearchExact(String),
    FullTextSearch(String),
    FullTextSearchExact(String),
    AlbumTitle(String),
    AlbumTitleExact(String),
    AlbumArtists(String),
    AlbumArtistsExact(String),
    Artist(String),
    ArtistExact(String),
    Source(String),
    Format(TrackFileType),
    BitrateLessThan(i64),
    BitrateGreaterThan(i64),
    CoverArtWidthLessThan(i64),
    CoverArtWidthGreaterThan(i64),
    CoverArtHeightLessThan(i64),
    CoverArtHeightGreaterThan(i64),
    DurationLessThan(i64),
    DurationGreaterThan(i64),
    HasCoverArt(bool),
    HasMusicbrainzId(bool),
    HasDuplicates(bool),
    LogicalAnd(Box<Bang>, Box<Bang>),
    LogicalOr(Box<Bang>, Box<Bang>),
    Grouping(Box<Bang>),
}

impl Bang {
    pub fn new(query: &str) -> Result<Bang> {
        let token_stream = lex_query(query)?;
        parse_token_stream(&mut token_stream.iter())
    }
}
