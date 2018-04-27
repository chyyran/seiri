use std::str::FromStr;
use std::slice::Iter;
use super::lexer::Token;
use super::bangs::Bang;
use track::TrackFileType;
use error::{Error, Result};
use humantime::Duration;
use chrono::NaiveDate;
use super::time::*;

trait BangIdentifier {
    fn as_bang_type(&self) -> BangType;
}

impl BangIdentifier for str {
    fn as_bang_type(&self) -> BangType {
        match self {
            "t" => BangType::TitleSearch,
            "T" => BangType::TitleSearchExact,
            "q" => BangType::FullTextSearch,
            "Q" => BangType::FullTextSearchExact,
            "al" => BangType::AlbumTitle,
            "AL" => BangType::AlbumTitleExact,
            "alar" => BangType::AlbumArtists,
            "ALAR" => BangType::AlbumArtistsExact,
            "ar" => BangType::Artist,
            "AR" => BangType::ArtistExact,
            "s" => BangType::Source,
            "f" => BangType::Format,
            "dlt" => BangType::DurationLessThan,
            "dgt" => BangType::DurationGreaterThan,
            "brlt" => BangType::BitrateLessThan,
            "brgt" => BangType::BitrateGreaterThan,
            "cwlt" => BangType::CoverArtWidthLessThan,
            "cwgt" => BangType::CoverArtWidthGreaterThan,
            "chlt" => BangType::CoverArtHeightLessThan,
            "chgt" => BangType::CoverArtHeightGreaterThan,
            "c" => BangType::HasCoverArt,
            "mb" => BangType::HasMusicbrainzId,
            "dup" => BangType::HasDuplicates,
            "ubf" => BangType::UpdatedBefore,
            "uaf" => BangType::UpdatedAfter,
            "!" => BangType::Grouping,
            unknown => BangType::Unknown(unknown.to_owned()),
        }
    }
}

enum BangType {
    TitleSearch,
    TitleSearchExact,
    FullTextSearch,
    FullTextSearchExact,
    AlbumTitle,
    AlbumTitleExact,
    AlbumArtists,
    AlbumArtistsExact,
    Artist,
    ArtistExact,
    Source,
    Format,
    BitrateLessThan,
    BitrateGreaterThan,
    DurationLessThan,
    DurationGreaterThan,
    CoverArtWidthLessThan,
    CoverArtWidthGreaterThan,
    CoverArtHeightLessThan,
    CoverArtHeightGreaterThan,
    HasCoverArt,
    HasMusicbrainzId,
    HasDuplicates,
    UpdatedBefore,
    UpdatedAfter,
    Grouping,
    Unknown(String),
}

/// Takes 3 tokens from the iterator,
/// and returns the middle one.
/// Intended to extract the argument token, will panic if
/// mismatches.
fn extract_argument(tokens: &mut Iter<Token>) -> Token {
    let mut tokens = tokens.take(3);
    let argument = tokens.nth(1).cloned().unwrap();
    tokens.next(); // Advance past the ArgumentEnd.
    argument
}

fn parse_bang<F, T>(producer: F, argument: Token) -> Result<Bang>
where
    T: FromStr,
    F: Fn(T) -> Bang,
{
    if let Token::Argument(argument) = argument {
        let parsed = argument.parse::<T>();
        if let Ok(parsed) = parsed {
            Ok(producer(parsed))
        } else {
            Err(Error::ParserInvalidInput(argument))
        }
    } else {
        Err(Error::LexerUnexpectedEndOfInput)
    }
}

pub fn take_until_braces_balanced<'a, 'b>(tokens: &'a mut Iter<Token>) -> Result<Vec<Token>> {
    let mut group = Vec::<Token>::new();
    // Assume that we have an argument begin here.
    if let Some(&Token::ArgumentBegin) = tokens.next() {
        let mut counter = 1;
        while let Some(token) = tokens.next().cloned() {
            match token {
                Token::ArgumentBegin => counter += 1,
                Token::ArgumentEnd => counter -= 1,
                _ => (),
            };
            if counter != 0 {
                group.push(token);
            };
            if counter == 0 {
                // We need to pad the grouping with the
                // InputEnd token, since parse_token_stream
                // expects an InputEnd at the end.
                group.push(Token::InputEnd);
                return Ok(group);
            }
        }
        Err(Error::LexerUnexpectedEndOfInput)
    } else {
        panic!("Sent the wrong token!");
    }
}

pub fn parse_token_stream(tokens: &mut Iter<Token>) -> Result<Bang> {
    // We're assuming that the slice begins at the
    // start of a token stream.
    // valid tokens at the beginning are either a bang prefix (!),
    // or the match all bang.

    let opening_token = tokens.next().cloned();
    match opening_token {
        Some(Token::BangPrefix(_)) => (),
        Some(Token::MatchAll) => return Ok(Bang::All),
        Some(token) => return Err(Error::ParserUnexpectedToken(token)),
        None => return Err(Error::LexerUnexpectedEndOfInput),
    }

    // At this point the opening_token is a bang prefix,
    // so the 2nd token must be a bang identifier.

    let bang_ident = tokens.next().cloned();

    let lhs = if let Some(Token::BangIdentifier(bang_ident)) = bang_ident {
        match bang_ident.as_bang_type() {
            // For all bangs that aren't groupings, we can just
            // assume that it follows the sequence
            // [ArgumentBegin, Argument, ArgumentEnd]
            BangType::TitleSearch => parse_bang(
                |search: String| Bang::TitleSearch(search),
                extract_argument(tokens),
            ),
            BangType::TitleSearchExact => parse_bang(
                |search: String| Bang::TitleSearchExact(search),
                extract_argument(tokens),
            ),
            BangType::FullTextSearch => parse_bang(
                |search: String| Bang::FullTextSearch(search),
                extract_argument(tokens),
            ),
            BangType::FullTextSearchExact => parse_bang(
                |search: String| Bang::FullTextSearchExact(search),
                extract_argument(tokens),
            ),
            BangType::AlbumTitle => parse_bang(
                |search: String| Bang::AlbumTitle(search),
                extract_argument(tokens),
            ),
            BangType::AlbumTitleExact => parse_bang(
                |search: String| Bang::AlbumTitleExact(search),
                extract_argument(tokens),
            ),
            BangType::AlbumArtists => parse_bang(
                |search: String| Bang::AlbumArtists(search),
                extract_argument(tokens),
            ),
            BangType::AlbumArtistsExact => parse_bang(
                |search: String| Bang::AlbumArtistsExact(search),
                extract_argument(tokens),
            ),
            BangType::Artist => parse_bang(
                |search: String| Bang::Artist(search),
                extract_argument(tokens),
            ),
            BangType::ArtistExact => parse_bang(
                |search: String| Bang::ArtistExact(search),
                extract_argument(tokens),
            ),
            BangType::Source => parse_bang(
                |search: String| Bang::Source(search),
                extract_argument(tokens),
            ),
            BangType::Format => parse_bang(
                |format: TrackFileType| Bang::Format(format),
                extract_argument(tokens),
            ),
            BangType::DurationLessThan => parse_bang(
                |duration: Duration| Bang::DurationLessThan(duration.to_ticks()),
                extract_argument(tokens),
            ),
            BangType::DurationGreaterThan => parse_bang(
                |duration: Duration| Bang::DurationGreaterThan(duration.to_ticks()),
                extract_argument(tokens),
            ),
            BangType::BitrateLessThan => parse_bang(
                |bitrate: i32| Bang::BitrateLessThan(bitrate),
                extract_argument(tokens),
            ),
            BangType::BitrateGreaterThan => parse_bang(
                |bitrate: i32| Bang::BitrateGreaterThan(bitrate),
                extract_argument(tokens),
            ),
            BangType::CoverArtWidthLessThan => parse_bang(
                |cw: i32| Bang::CoverArtWidthLessThan(cw),
                extract_argument(tokens),
            ),
            BangType::CoverArtWidthGreaterThan => parse_bang(
                |cw: i32| Bang::CoverArtWidthGreaterThan(cw),
                extract_argument(tokens),
            ),
            BangType::CoverArtHeightLessThan => parse_bang(
                |ch: i32| Bang::CoverArtHeightLessThan(ch),
                extract_argument(tokens),
            ),
            BangType::CoverArtHeightGreaterThan => parse_bang(
                |ch: i32| Bang::CoverArtHeightGreaterThan(ch),
                extract_argument(tokens),
            ),
            BangType::HasCoverArt => {
                parse_bang(|c: bool| Bang::HasCoverArt(c), extract_argument(tokens))
            }
            BangType::HasMusicbrainzId => parse_bang(
                |mb: bool| Bang::HasMusicbrainzId(mb),
                extract_argument(tokens),
            ),
            BangType::HasDuplicates => parse_bang(
                |dup: bool| Bang::HasDuplicates(dup),
                extract_argument(tokens),
            ),
            BangType::UpdatedBefore => parse_bang(
                |ubf: NaiveDate| Bang::UpdatedBefore(ubf.format("%Y-%m-%d").to_string()),
                extract_argument(tokens),
            ),
            BangType::UpdatedAfter => parse_bang(
                |uaf: NaiveDate| Bang::UpdatedAfter(uaf.format("%Y-%m-%d").to_string()),
                extract_argument(tokens),
            ),
            BangType::Grouping => {
                let mut grouping_token_stream = take_until_braces_balanced(tokens)?;
                Ok(Bang::Grouping(Box::new(parse_token_stream(
                    &mut grouping_token_stream.iter(),
                )?)))
            }

            BangType::Unknown(unknown) => return Err(Error::ParserUnknownBang(unknown)),
        }
    } else {
        return Err(Error::LexerUnexpectedEndOfInput);
    };

    // At this point, three tokens minimum should have been consumed.
    match tokens.next().cloned() {
        Some(Token::InputEnd) => lhs,
        Some(Token::LogicalOperator(operator)) => match operator {
            '|' => Ok(Bang::LogicalOr(
                Box::new(lhs?),
                Box::new(parse_token_stream(tokens)?),
            )),
            '&' => Ok(Bang::LogicalAnd(
                Box::new(lhs?),
                Box::new(parse_token_stream(tokens)?),
            )),
            c => Err(Error::ParserUnknownBang(c.to_string())),
        },
        Some(t) => Err(Error::ParserUnexpectedToken(t)),
        None => Err(Error::LexerUnexpectedEndOfInput),
    }
}
