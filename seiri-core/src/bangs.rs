extern crate lazy_static;
extern crate regex;

use std::iter::Peekable;
use std::str::Chars;
use regex::{Match, Regex};

use error::{Error, Result};
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
    pub fn to_sql(&self) -> String {
        match *self {
            Bang::TitleSearch(ref title) => format!("Title LIKE '%{}%'", title),
            Bang::FullTextSearch(ref text) => format!("Title LIKE '%{}%' OR Album LIKE '%{}%' OR Artist LIKE '%{}%' OR AlbumArtists LIKE '%{}%'", 
                                        text, text, text, text),
            Bang::LogicalAnd(ref lhs, ref rhs) => format!("({}) AND ({})", lhs.to_sql(), rhs.to_sql()),
            Bang::LogicalOr(ref lhs, ref rhs) => format!("({}) OR ({})", lhs.to_sql(), rhs.to_sql()),
            _ => "".to_string()
        }
    }

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

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    BangBegin(char),
    BangIdentifier(String),
    ArgumentBegin,
    ArgumentEnd,
    Argument(String),
    LogicalOperator(char),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum LexerMode {
    Bang,
    BangIdentifier,
    Group,
    ArgumentEdge,
    Argument,
}

fn match_bang(c: &char, characters: &mut Peekable<Chars>) -> Result<Option<(Token, LexerMode)>> {
    if !c.is_whitespace() {
        return match c {
            &'!' => {
                characters.next();
                Ok(Some((Token::BangBegin(*c), LexerMode::BangIdentifier)))
            }
            _ => Err(Error::LexerUnexpectedCharacter(*c, LexerMode::Bang)),
        };
    }
    characters.next();
    Ok(None)
}

fn match_bang_identifier(
    c: &char,
    characters: &mut Peekable<Chars>,
) -> Result<Option<(Token, LexerMode)>> {
    if c.is_alphanumeric() {
        let mut bang_identifier: String = String::from("");
        while let Some(c) = characters.peek().cloned() {
            if c.is_alphanumeric() {
                bang_identifier.push(c);
                characters.next();
            } else {
                break;
            }
        }
        let token = Token::BangIdentifier(bang_identifier);
        Ok(Some((token, LexerMode::ArgumentEdge)))
    } else if c.is_whitespace() {
        characters.next();
        Ok(None)
    } else {
        return Err(Error::LexerUnexpectedCharacter(
            *c,
            LexerMode::BangIdentifier,
        ));
    }
}

fn match_argument_edge(
    c: &char,
    characters: &mut Peekable<Chars>,
) -> Result<Option<(Token, LexerMode)>> {
    if c.is_whitespace() {
        characters.next();
        Ok(None)
    } else {
        let token = match c {
            &'|' => Some((Token::LogicalOperator('|'), LexerMode::Bang)),
            &'&' => Some((Token::LogicalOperator('&'), LexerMode::Bang)),
            &'{' => Some((Token::ArgumentBegin, LexerMode::Argument)),
            &'}' => Some((Token::ArgumentEnd, LexerMode::ArgumentEdge)),
            _ => return Err(Error::LexerUnexpectedCharacter(*c, LexerMode::ArgumentEdge)),
        };
        characters.next();
        Ok(token)
    }
}

fn match_argument(
    c: &char,
    characters: &mut Peekable<Chars>,
) -> Result<Option<(Token, LexerMode)>> {
    let mut argument: String = String::from("");
    while let Some(c) = characters.peek().cloned() {
        if c.is_alphanumeric() {
            argument.push(c);
            characters.next();
        } else {
            break;
        }
    }
    let token = Token::Argument(argument);
    Ok(Some((token, LexerMode::ArgumentEdge)))
}

pub fn lex_query(query: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::<Token>::new();
    let mut mode = LexerMode::Bang;

    let query = query.to_owned();
    let mut characters = query.chars().peekable();

    while let Some(c) = characters.peek().cloned() {
        let result = match mode {
            LexerMode::Bang => match_bang(&c, &mut characters),
            LexerMode::BangIdentifier => match_bang_identifier(&c, &mut characters),
            LexerMode::ArgumentEdge => match_argument_edge(&c, &mut characters),
            LexerMode::Argument => match_argument(&c, &mut characters),
            _ => Ok(None),
        };
        match result {
            Ok(some) => match some {
                Some(token) => {
                    mode = token.1;
                    tokens.push(token.0);
                    println!("{:?}", tokens)
                }
                None => (),
            },
            Err(err) => return Err(err),
        }
    }

    Ok(tokens)
}
