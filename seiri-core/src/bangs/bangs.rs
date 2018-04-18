extern crate itertools;
extern crate lazy_static;
extern crate regex;

use regex::{Match, Regex};

use std::str::FromStr;
use track::TrackFileType;


trait ArgumentConverter<'a> {
    fn as_string(&self) -> String;
    fn as_i64(&self) -> i64;
    fn as_track_type(&self) -> TrackFileType;
    fn as_bool(&self) -> bool;
    fn get_index(&self) -> usize;
}

impl<'a> ArgumentConverter<'a> for Option<Match<'a>> {
    fn as_string(&self) -> String {
        self.and_then(|arg: Match| Some(arg.as_str()))
            .unwrap_or("")
            .to_owned()
    }

    fn as_track_type(&self) -> TrackFileType {
        match *self {
            // TFT::from_str never returns an error.
            Some(format) => TrackFileType::from_str(format.as_str()).unwrap(),
            None => TrackFileType::Unknown,
        }
    }

    fn as_i64(&self) -> i64 {
        self.and_then(|arg: Match| Some(arg.as_str()))
            .and_then(|arg: &str| arg.parse::<i64>().ok())
            .unwrap_or(0)
    }

    fn as_bool(&self) -> bool {
        self.and_then(|arg: Match| Some(arg.as_str()))
            .and_then(|arg: &str| arg.parse::<bool>().ok())
            .unwrap_or(false)
    }

    fn get_index(&self) -> usize {
        self.and_then(|arg: Match| Some(arg.end())).unwrap_or(0)
    }
}

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
    pub fn parse(query: &str) -> Bang {
        lazy_static! {
            static ref WHITESPACE_BETWEEN_REGEX: Regex =
                Regex::new(r"(?P<open>^|}+|\||&)*(\s+)(?P<close>!|\||\&)").unwrap();
            static ref NON_LOGICAL_BANG_REGEX: Regex = Regex::new(r"^(!\w*)").unwrap();
            static ref LOGICAL_BANG_REGEX: Regex = Regex::new(r"(?:\})(\||&)(?:!)").unwrap();
            static ref QUERY_ARGUMENT_REGEX: Regex = Regex::new(r"(?:\{)(.*?)(?:\}(\||&|$))").unwrap();
            //static ref QUERY_GROUP_REGEX: Regex = Regex::new(r"(?:\{)(.*)(?:\}(\||&|$))").unwrap();
        }

        let query = WHITESPACE_BETWEEN_REGEX.replace_all(query, "$open$close");

        println!("{}", query);

        if !query.starts_with("!") {
            return Bang::TitleSearch(query.to_owned().to_string());
        };

        let bang_str: Option<Match> = match NON_LOGICAL_BANG_REGEX.find(&query) {
            Some(bang_str) => Some(bang_str),
            None => None,
        };

        let arg_str: Option<Match> = match QUERY_ARGUMENT_REGEX.captures(&query) {
            Some(captures) => match captures.get(1) {
                Some(arg) => Some(arg),
                None => None,
            },
            None => None,
        };

        let bang: Bang = match bang_str {
            Some(bang_match) => match bang_match.as_str() {
                "!q" => Bang::FullTextSearch(arg_str.as_string()),
                "!Q" => Bang::FullTextSearchExact(arg_str.as_string()),
                "!al" => Bang::AlbumTitle(arg_str.as_string()),
                "!AL" => Bang::AlbumTitleExact(arg_str.as_string()),
                "!alar" => Bang::AlbumArtists(arg_str.as_string()),
                "!ALAR" => Bang::AlbumArtistsExact(arg_str.as_string()),
                "!ar" => Bang::Artist(arg_str.as_string()),
                "!AR" => Bang::ArtistExact(arg_str.as_string()),
                "!f" => Bang::Format(arg_str.as_track_type()),
                "!brlt" => Bang::BitrateLessThan(arg_str.as_i64()),
                "!brgt" => Bang::BitrateGreaterThan(arg_str.as_i64()),
                "!cwlt" => Bang::CoverArtWidthLessThan(arg_str.as_i64()),
                "!cwgt" => Bang::CoverArtWidthGreaterThan(arg_str.as_i64()),
                "!chlt" => Bang::CoverArtHeightLessThan(arg_str.as_i64()),
                "!chgt" => Bang::CoverArtHeightGreaterThan(arg_str.as_i64()),
                "!c" => Bang::HasCoverArt(arg_str.as_bool()),
                "!mb" => Bang::HasMusicbrainzId(arg_str.as_bool()),
                "!dup" => Bang::Duplicates,
                _ => Bang::TitleSearch(query.to_owned().to_string()),
            },
            None => Bang::TitleSearch(query.to_owned().to_string()),
        };

        let concatenator_str: Option<Match> = match LOGICAL_BANG_REGEX.captures(&query) {
            Some(captures) => match captures.get(1) {
                Some(operator) => Some(operator),
                None => None,
            },
            None => None,
        };

        if let Some(concatenator_str) = concatenator_str {
            let next_bang_position = concatenator_str.end();
            match concatenator_str.as_str() {
                "|" => Bang::LogicalOr(
                    Box::new(bang),
                    Box::new(Bang::parse(&query[next_bang_position..])),
                ),
                "&" => Bang::LogicalAnd(
                    Box::new(bang),
                    Box::new(Bang::parse(&query[next_bang_position..])),
                ),
                _ => bang,
            }
        } else {
            bang
        }
    }
}