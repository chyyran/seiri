extern crate itertools;

use track::TrackFileType;
use error::{Result};
use super::lexer::{lex_query};
use super::parser::{parse_token_stream};
use std::path::{Path, PathBuf};

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
    BitrateLessThan(i32), 
    BitrateGreaterThan(i32),
    CoverArtWidthLessThan(i32),
    CoverArtWidthGreaterThan(i32),
    CoverArtHeightLessThan(i32),
    CoverArtHeightGreaterThan(i32),
    DurationLessThan(i64),
    DurationGreaterThan(i64),
    HasCoverArt(bool),
    HasMusicbrainzId(bool),
    HasDuplicates(bool),
    LogicalAnd(Box<Bang>, Box<Bang>),
    LogicalOr(Box<Bang>, Box<Bang>),
    Grouping(Box<Bang>),
    UpdatedBefore(String),
    UpdatedAfter(String),
    FilePath(String)
}

impl Bang {
    pub fn new(query: &str) -> Result<Bang> {
        let token_stream = lex_query(query)?;
        parse_token_stream(&mut token_stream.iter())
    }
}

impl From<PathBuf> for Bang {
    fn from(path: PathBuf) -> Bang {
        Bang::FilePath(path.to_string_lossy().into_owned())
    }
}

impl <'a> From<&'a Path> for Bang {
    fn from(path: &Path) -> Bang {
        Bang::FilePath(path.to_string_lossy().into_owned())
    }
}