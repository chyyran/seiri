extern crate lazy_static;
extern crate regex;

use regex::{Regex, Match};

use track::TrackFileType;

#[derive(Debug)]
pub enum Bang {
    TitleSearch(String),
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
    fn prefix(&self) -> &'static str {
        match *self {
            Bang::TitleSearch(_) => "",
            Bang::FullTextSearch(_) => "!q",
            Bang::FullTextSearchExact(_) => "!Q",
            Bang::AlbumTitle(_) => "!al",
            Bang::AlbumTitleExact(_) => "!AL",
            Bang::AlbumArtists(_) => "!alar",
            Bang::AlbumArtistsExact(_) => "!ALAR",
            Bang::Artist(_) => "!ar",
            Bang::ArtistExact(_) => "!AR",
            Bang::Format(_) => "!f",
            Bang::BitrateLessThan(_) => "!brlt",
            Bang::BitrateGreaterThan(_) => "!brgt",
            Bang::CoverArtHeightGreaterThan(_) => "!chgt",
            Bang::CoverArtHeightLessThan(_) => "!chlt",
            Bang::CoverArtWidthGreaterThan(_) => "!cwgt",
            Bang::CoverArtWidthLessThan(_) => "!cwlt",
            Bang::HasCoverArt(_) => "!c",
            Bang::HasMusicbrainzId(_) => "!mb",
            Bang::Duplicates => "!dup",
            Bang::LogicalAnd(_, _) => "&",
            Bang::LogicalOr(_, _) => "|",
            Bang::Grouping(_) => "!",
        }
    }
    pub fn parse(query: &str) -> Bang {
        lazy_static! {
            static ref WHITESPACE_BETWEEN_REGEX: Regex =
                Regex::new(r"(?P<open>^|}|\||&)(\s+)(?P<close>!|\||\&)").unwrap();
            static ref NON_LOGICAL_BANG_REGEX: Regex = 
                Regex::new(r"^(!\w+)").unwrap();
            static ref LOGICAL_BANG_REGEX: Regex = 
                Regex::new(r"^(\||&)").unwrap();
        }

        let query = WHITESPACE_BETWEEN_REGEX.replace_all(query, "$open$close");

        println!("{}", query);

        if !query.starts_with("!") {
            return Bang::TitleSearch(query.to_owned().to_string());
        };

        let bang: Option<Match> = match NON_LOGICAL_BANG_REGEX.find(&query) {
            Some(bang_str) => Some(bang_str),
            None => match LOGICAL_BANG_REGEX.find(&query) {
                Some(logical) => Some(logical),
                None => None
            }
        };
        
        Bang::TitleSearch(query.to_owned().to_string())
    }
}
