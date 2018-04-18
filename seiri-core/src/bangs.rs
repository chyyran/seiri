extern crate itertools;
extern crate lazy_static;
extern crate regex;

use std::str::Chars;
use regex::{Match, Regex};

use error::{Error, Result};
use std::str::FromStr;
use track::TrackFileType;

use itertools::Itertools;
use itertools::multipeek;
use itertools::MultiPeek;

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
    ArgumentEdge,
    Argument,
}

fn match_bang(c: &char, characters: &mut MultiPeek<Chars>) -> Result<Option<(Token, LexerMode)>> {
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
    characters: &mut MultiPeek<Chars>,
) -> Result<Option<(Token, LexerMode)>> {
    if c.is_alphanumeric() {
        let token = Token::BangIdentifier(
            characters
                .take_while_ref(|&c| c.is_alphanumeric())
                .collect(),
        );
        Ok(Some((token, LexerMode::ArgumentEdge)))
    } else if c == &'!' {
        characters.next();
        Ok(Some((
            Token::BangIdentifier(String::from("!")),
            LexerMode::ArgumentEdge,
        )))
    } else {
        return Err(Error::LexerUnexpectedCharacter(
            *c,
            LexerMode::BangIdentifier,
        ));
    }
}

fn match_argument_edge(
    c: &char,
    characters: &mut MultiPeek<Chars>,
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

/// Searches a multipeek for the next character not equal to the specified character.
/// Advances the peek cursor.
fn next_non_character(ignore: char, chars: &mut MultiPeek<Chars>) -> Result<(char, usize)> {
    let mut index: usize = 0;
    while let Some(c) = chars.peek().cloned() {
        match c {
            _ if c == ignore => index += 1,
            _ => return Ok((c, index)),
        }
    }
    Err(Error::LexerUnexpectedEndOfInput)
}

fn remaining(chars: &mut MultiPeek<Chars>) -> usize {
    let mut index: usize = 0;
    while let Some(c) = chars.peek().cloned() {
        index += 1;
    }
    chars.reset_peek();
    index
}
/// Counts the amount of mismatched braces current.
fn brace_counter(tokens: &Vec<Token>) -> usize {
    let mut counter: usize = 0;

    for token in tokens {
        match token {
            &Token::ArgumentBegin => counter += 1,
            &Token::ArgumentEnd => counter -= 1,
            _ => continue,
        };
    }
    counter
}

fn match_argument(
    c: &char,
    characters: &mut MultiPeek<Chars>,
    tokens: &Vec<Token>,
) -> Result<Option<(Token, LexerMode)>> {
    if let &Some(ref token) = &tokens.iter().rev().nth(1) {
        match token {
            &&Token::BangIdentifier(ref token) => match token.as_ref() {
                "!" => return match_bang(c, characters),
                _ => (),
            },
            _ => (),
        }
    };

    let mut argument = String::new();
    characters.reset_peek(); // Reset the peek to right before entering this fn.
                             // Note that 'c' in this scope now is now invalid.

    while let Some(c) = characters.peek().cloned() {
        match c {
            '}' => {
                // We need to determine if this bracket is a closing.
                if let Ok(bracket_after) = next_non_character(' ', characters) {
                    match bracket_after.0 {
                        '|' | '&' => {
                            // Need to see if the next non space character is a bang.
                            // If so, drop this '}' and switch immediately to edge
                            // mode.
                            match next_non_character(' ', characters) {
                                Ok(next_character) => {
                                    match next_character.0 {
                                        '!' => {
                                            // We found a bang!
                                            characters.reset_peek();
                                            return Ok(Some((
                                                Token::Argument(argument),
                                                LexerMode::ArgumentEdge,
                                            )));
                                        }
                                        _ => {
                                            // Consume this '}'
                                            characters.next();
                                            characters.reset_peek();
                                            argument.push(c);
                                        }
                                    }
                                }
                                Err(err) => return Err(err),
                            };
                        }
                        '}' => {
                            // Need to see if the next non '}' character is a logical operator.
                            // If so, count braces and switch to edge detection once braces match.
                            // Otherwise, consume this '}'
                            match next_non_character('}', characters) {
                                Ok(next_character) => {
                                    match next_character.0 {
                                        '|' | '&' => {
                                            // Need to see if the next non space character is a bang.
                                            // If so, take next_characters.0 - unmatched - 1 braces, then
                                            // and immediately to edge mode.
                                            match next_non_character(' ', characters) {
                                                Ok(next_character) => {
                                                    match next_character.0 {
                                                        '!' => {
                                                            // We found a bang!
                                                            characters.reset_peek();
                                                            let braces = brace_counter(tokens);

                                                            // The operator was found after
                                                            // n - 1 peeks (n = `next_character.1`)
                                                            // In other words, there are n braces between
                                                            // This '}' and the operator, with m braces,
                                                            // where m =``braces`.
                                                            // We will consume n - m braces.
                                                            for _ in 0..(next_character.1 - braces)
                                                            {
                                                                if let Some(c) = characters.next() {
                                                                    characters.reset_peek();
                                                                    argument.push(c);
                                                                } else {
                                                                    return Err(Error::LexerUnexpectedEndOfInput);
                                                                }
                                                            }
                                                            // Return the token with the proper amount of closing braces.
                                                            return Ok(Some((
                                                                Token::Argument(argument),
                                                                LexerMode::ArgumentEdge,
                                                            )));
                                                        }
                                                        _ => {
                                                            // Consume this '}'
                                                            characters.next();
                                                            characters.reset_peek();
                                                            argument.push(c);
                                                        }
                                                    }
                                                }
                                                Err(err) => return Err(err),
                                            };
                                        }
                                        _ => {
                                            // Consume this '}'
                                            characters.next();
                                            characters.reset_peek();
                                            argument.push(c);
                                        }
                                    }
                                }
                                Err(_) => {
                                    // We matched the end of the string,
                                    // and have to do brace counting now.

                                    characters.reset_peek(); // Include this '}'
                                                             // Matching the end of string here means that
                                                             // All the characters from here to the end are braces.
                                    let length_remaining = remaining(characters);

                                    // Braces to keep.
                                    let braces = brace_counter(tokens);

                                    for _ in 0..(length_remaining - braces) {
                                        if let Some(c) = characters.next() {
                                            characters.reset_peek();
                                            argument.push(c);
                                        } else {
                                            return Err(Error::LexerUnexpectedEndOfInput);
                                        }
                                    }
                                    return Ok(Some((
                                        Token::Argument(argument),
                                        LexerMode::ArgumentEdge,
                                    )));
                                }
                            };
                        }
                        _ => {
                            // Consume this '}'
                            characters.next(); // We consume this character
                            characters.reset_peek(); // Reset the peek to the next character.
                            argument.push(c);
                        }
                    }
                } else {
                    // This is a closing bracket at the end of string,
                    // and we are done parsing this argument.
                    // Do not consume this '}'
                    characters.reset_peek(); // Reset this peek.
                    return Ok(Some((Token::Argument(argument), LexerMode::ArgumentEdge)));
                };
            }
            // Support escapes as well.
            '\\' => {
                characters.next(); // Consume this '\' without adding it to the buffer.
                if let Some(escape_after) = characters.next() {
                    argument.push(escape_after);
                    characters.reset_peek();
                } else {
                    // If we try an escape at the end of the striing
                    return Err(Error::LexerUnexpectedEscapeCharacter(LexerMode::Argument));
                };
            }
            _ => {
                characters.next(); // We consume this character
                characters.reset_peek(); // Reset the peek to the next character.
                argument.push(c);
            }
        }
    }

    Ok(Some((Token::Argument(argument), LexerMode::ArgumentEdge)))
}

pub fn lex_query(query: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::<Token>::new();
    let mut mode = LexerMode::Bang;

    let query = query.to_owned();
    let mut characters = multipeek(query.chars());

    while let Some(c) = characters.peek().cloned() {
        let result = match mode {
            LexerMode::Bang => match_bang(&c, &mut characters),
            LexerMode::BangIdentifier => match_bang_identifier(&c, &mut characters),
            LexerMode::ArgumentEdge => match_argument_edge(&c, &mut characters),
            LexerMode::Argument => match_argument(&c, &mut characters, &tokens),
            _ => Ok(None),
        };
        match result {
            Ok(some) => match some {
                Some(token) => {
                    mode = token.1;
                    tokens.push(token.0)
                }
                None => (),
            },
            Err(err) => return Err(err),
        }
    }

    Ok(tokens)
}
