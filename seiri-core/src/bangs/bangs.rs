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
    TItleSearchExact(String),
    FullTextSearch(String),
    FullTextSearchExact(String),
    AlbumTitle(String),
    AlbumTitleExact(String),
    AlbumArtists(String),
    AlbumArtistsExact(String),
    Artist(String),
    ArtistExact(String),
    Format(TrackFileType),
    BitrateLessThan(i64),
    BitrateGreaterThan(i64),
    CoverArtWidthLessThan(i64),
    CoverArtWidthGreaterThan(i64),
    CoverArtHeightLessThan(i64),
    CoverArtHeightGreaterThan(i64),
    HasCoverArt(bool),
    HasMusicbrainzId(bool),
    Duplicates,
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
